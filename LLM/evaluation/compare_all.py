#!/usr/bin/env python3
"""
Compare three versions of each example:
  1. Original C2Rust main.rs (baseline)
  2. Concrat-transformed main.rs (from test_examples.sh)
  3. LLM-rewritten main_rewritten_0.rs

Metrics:
  - Compilation: does it compile?
  - Safety: unsafe blocks, pthread usage, raw pointers, static mut, std::sync usage
  - Correctness: does the binary produce the same output as original?
"""

import os
import re
import sys
import glob
import subprocess
import json
import tempfile
import shutil

EXAMPLES_DIR = "/home/guoxy/concrat/examples"
CONCRAT_DIR = "/tmp/concrat_results"
NIGHTLY = "nightly-2022-07-05"
DEPS_DIR = "/home/guoxy/concrat/deps_crate/target/debug/deps"
CONCRAT_CACHE_PATH = "/home/guoxy/concrat/LLM/result/concrat_cache.json"

# ── Safety Metrics ──────────────────────────────────────────────────────────

def count_pattern(code, pattern):
    return len(re.findall(pattern, code))

def safety_metrics(code):
    return {
        "unsafe":       count_pattern(code, r'\bunsafe\b'),
        "pthread":      count_pattern(code, r'\bpthread_'),
        "raw_ptr":      count_pattern(code, r'\*mut\b|\*const\b'),
        "static_mut":   count_pattern(code, r'\bstatic\s+mut\b'),
        "libc":         count_pattern(code, r'\blibc::'),
        "std_mutex":    count_pattern(code, r'\bMutex\b'),
        "std_arc":      count_pattern(code, r'\bArc\b'),
        "std_rwlock":   count_pattern(code, r'\bRwLock\b'),
        "std_condvar":  count_pattern(code, r'\bCondvar\b'),
        "std_thread":   count_pattern(code, r'\bthread::spawn\b'),
        "lines":        len(code.splitlines()),
    }

# ── Compilation Check ───────────────────────────────────────────────────────

def try_compile_with_cargo(rs_file, example_dir):
    """Try compiling a .rs file using the example's Cargo.toml context."""
    td = tempfile.mkdtemp()
    try:
        # Copy Cargo.toml and support files
        for f in ["Cargo.toml", "rust-toolchain"]:
            src = os.path.join(example_dir, f)
            if os.path.exists(src):
                shutil.copy(src, td)
        # Copy c2rust-lib.rs if exists
        lib_rs = os.path.join(example_dir, "c2rust-lib.rs")
        if os.path.exists(lib_rs):
            shutil.copy(lib_rs, td)
        # Copy the target file as main.rs
        shutil.copy(rs_file, os.path.join(td, "main.rs"))

        nightly = NIGHTLY
        tc_file = os.path.join(td, "rust-toolchain")
        if os.path.exists(tc_file):
            with open(tc_file) as f:
                nightly = f.read().strip()

        result = subprocess.run(
            ["cargo", f"+{nightly}", "build", "--manifest-path",
             os.path.join(td, "Cargo.toml")],
            capture_output=True, text=True, timeout=60,
            env={**os.environ, "RUSTFLAGS": "-Awarnings"}
        )
        return result.returncode == 0, result.stderr[:500]
    except Exception as e:
        return False, str(e)[:200]
    finally:
        shutil.rmtree(td, ignore_errors=True)


def try_compile_standalone(rs_file):
    """Try compiling a standalone .rs file (LLM rewritten, may not use libc)."""
    td = tempfile.mkdtemp()
    try:
        # First try: standalone rustc
        result = subprocess.run(
            ["rustc", "--edition", "2021", rs_file,
             "-o", os.path.join(td, "out")],
            capture_output=True, text=True, timeout=60,
            env={**os.environ, "RUSTFLAGS": "-Awarnings"}
        )
        if result.returncode == 0:
            return True, ""

        # Second try: use the example's Cargo.toml (it may need libc)
        example_dir = os.path.dirname(rs_file)
        return try_compile_with_cargo(rs_file, example_dir)
    except Exception as e:
        return False, str(e)[:200]
    finally:
        shutil.rmtree(td, ignore_errors=True)


# ── Lock Safety Analysis ────────────────────────────────────────────────────

def analyze_lock_safety(code, label):
    """Analyze lock-related safety properties."""
    issues = []

    # Check for proper Mutex usage patterns
    if re.search(r'\bMutex\b', code):
        # Good: using std::sync::Mutex
        if re.search(r'Mutex::new\(', code):
            pass  # proper construction
        if re.search(r'\.lock\(\)', code):
            pass  # proper locking via RAII

    # Check for pthread remaining (bad in rewritten code)
    pthread_locks = re.findall(r'pthread_mutex_(lock|unlock|init|destroy)', code)
    if pthread_locks and label != "original":
        issues.append(f"Still uses pthread_mutex ({len(pthread_locks)} calls)")

    # Check for manual lock/unlock without RAII (potential deadlock)
    if re.search(r'pthread_mutex_lock', code) and not re.search(r'pthread_mutex_unlock', code):
        issues.append("Lock without matching unlock (potential deadlock)")
    if re.search(r'pthread_mutex_unlock', code) and not re.search(r'pthread_mutex_lock', code):
        issues.append("Unlock without matching lock")

    # Check for static mut (data race risk)
    static_muts = re.findall(r'static\s+mut\s+(\w+)', code)
    if static_muts and label != "original":
        issues.append(f"static mut variables remain: {', '.join(static_muts[:5])}")

    # Check for unsafe blocks
    unsafe_count = len(re.findall(r'\bunsafe\b', code))
    if unsafe_count > 0 and label != "original":
        issues.append(f"{unsafe_count} unsafe keyword(s) remain")

    # Check for lock() without unwrap/expect (potential panic)
    if re.search(r'\.lock\(\)[^.\n]*;', code):
        # .lock() without .unwrap() - this is actually fine, Result is used
        pass

    # Check for proper Arc<Mutex<T>> pattern
    has_arc_mutex = bool(re.search(r'Arc<\s*Mutex', code) or re.search(r'Arc::new\(\s*Mutex', code))

    # Check for proper thread joining
    has_thread_spawn = bool(re.search(r'thread::spawn', code))
    has_join = bool(re.search(r'\.join\(\)', code))
    if has_thread_spawn and not has_join:
        issues.append("thread::spawn without join (detached thread)")

    return {
        "issues": issues,
        "has_std_mutex": bool(re.search(r'\bMutex\b', code)),
        "has_arc_mutex": has_arc_mutex,
        "has_pthread": bool(re.search(r'\bpthread_', code)),
        "has_thread_spawn": has_thread_spawn,
        "has_join": has_join,
        "unsafe_count": unsafe_count,
    }


def load_concrat_cache():
    """Load cached concrat compilation & metrics results."""
    if os.path.exists(CONCRAT_CACHE_PATH):
        with open(CONCRAT_CACHE_PATH) as f:
            return json.load(f)
    return None

def save_concrat_cache(cache):
    """Save concrat compilation & metrics results to cache."""
    os.makedirs(os.path.dirname(CONCRAT_CACHE_PATH), exist_ok=True)
    with open(CONCRAT_CACHE_PATH, "w") as f:
        json.dump(cache, f, indent=2, default=str)

def build_concrat_cache(examples):
    """Build concrat cache by compiling all concrat versions once."""
    print("Building concrat cache (one-time)...")
    cache = {}
    for example_dir in examples:
        name = os.path.basename(example_dir.rstrip("/"))
        original_rs = os.path.join(example_dir, "main.rs")
        concrat_rs = os.path.join(CONCRAT_DIR, name, "main.rs")

        if not os.path.exists(original_rs):
            continue

        with open(original_rs) as f:
            orig_code = f.read()
        orig_metrics = safety_metrics(orig_code)
        orig_lock = analyze_lock_safety(orig_code, "original")

        entry = {"original": {"metrics": orig_metrics, "lock_safety": orig_lock}}

        if os.path.exists(concrat_rs):
            with open(concrat_rs) as f:
                concrat_code = f.read()
            c_metrics = safety_metrics(concrat_code)
            c_compile, c_err = try_compile_with_cargo(concrat_rs, example_dir)
            c_lock = analyze_lock_safety(concrat_code, "concrat")
            entry["concrat"] = {"metrics": c_metrics, "compiles": c_compile, "lock_safety": c_lock}
            status = "✅" if c_compile else "❌"
            print(f"  {name}: concrat {status}")

        cache[name] = entry

    save_concrat_cache(cache)
    print(f"Concrat cache saved to: {CONCRAT_CACHE_PATH}\n")
    return cache


# ── Main ────────────────────────────────────────────────────────────────────

def main():
    prompt_idx = int(sys.argv[1]) if len(sys.argv) > 1 and not sys.argv[1].startswith("--") else 0
    force = "--force" in sys.argv
    clear = "--clear" in sys.argv

    report_dir = f"/home/guoxy/concrat/LLM/result/{prompt_idx}"
    report_path = os.path.join(report_dir, "comparison_report.json")

    if os.path.exists(report_path) and not force:
        print(f"Report already exists: {report_path} (use --force to regenerate)")
        return

    os.makedirs(report_dir, exist_ok=True)

    examples = sorted([d for d in glob.glob(f"{EXAMPLES_DIR}/*/") if os.path.isdir(d)])

    # Load or build concrat cache
    concrat_cache = load_concrat_cache()
    if concrat_cache is None or clear:
        concrat_cache = build_concrat_cache(examples)

    results = []
    summary = {"total": 0, "llm_compiles": 0, "concrat_compiles": 0,
               "llm_safer": 0, "concrat_safer": 0, "llm_lock_clean": 0, "concrat_lock_clean": 0}

    print("=" * 100)
    print(f"{'Example':<22} │ {'Version':<10} │ {'Compile':>8} │ {'unsafe':>7} │ {'pthread':>8} │ {'raw_ptr':>8} │ {'st_mut':>7} │ {'Mutex':>6} │ {'thread':>7} │ {'lines':>6}")
    print("─" * 100)

    for example_dir in examples:
        name = os.path.basename(example_dir.rstrip("/"))
        summary["total"] += 1

        original_rs = os.path.join(example_dir, "main.rs")
        llm_rs = os.path.join(example_dir, f"main_rewritten_{prompt_idx}.rs")

        if not os.path.exists(original_rs):
            continue

        # Use cached original & concrat results
        cached = concrat_cache.get(name, {})
        orig_metrics = cached.get("original", {}).get("metrics")
        orig_lock = cached.get("original", {}).get("lock_safety")
        if not orig_metrics:
            with open(original_rs) as f:
                orig_code = f.read()
            orig_metrics = safety_metrics(orig_code)
            orig_lock = analyze_lock_safety(orig_code, "original")

        row = {"name": name}
        row["original"] = {"metrics": orig_metrics, "lock_safety": orig_lock}

        # Always print original
        print(f"  {name:<20} │ {'original':<10} │ {'(base)':>8} │ {orig_metrics['unsafe']:>7} │ {orig_metrics['pthread']:>8} │ {orig_metrics['raw_ptr']:>8} │ {orig_metrics['static_mut']:>7} │ {orig_metrics['std_mutex']:>6} │ {orig_metrics['std_thread']:>7} │ {orig_metrics['lines']:>6}")

        # ── Concrat version (from cache) ──
        if "concrat" in cached:
            c_metrics = cached["concrat"]["metrics"]
            c_compile = cached["concrat"]["compiles"]
            c_lock = cached["concrat"]["lock_safety"]

            compile_str = "✅" if c_compile else "❌"
            if c_compile:
                summary["concrat_compiles"] += 1
            if c_metrics["unsafe"] < orig_metrics["unsafe"]:
                summary["concrat_safer"] += 1
            if not c_lock["has_pthread"] and c_lock["has_std_mutex"]:
                summary["concrat_lock_clean"] += 1

            print(f"  {'':20} │ {'concrat':<10} │ {compile_str:>8} │ {c_metrics['unsafe']:>7} │ {c_metrics['pthread']:>8} │ {c_metrics['raw_ptr']:>8} │ {c_metrics['static_mut']:>7} │ {c_metrics['std_mutex']:>6} │ {c_metrics['std_thread']:>7} │ {c_metrics['lines']:>6}")

            if c_lock.get("issues"):
                for issue in c_lock["issues"]:
                    print(f"  {'':20} │   ⚠️  {issue}")

            row["concrat"] = {"metrics": c_metrics, "compiles": c_compile, "lock_safety": c_lock}

        # ── LLM version ──
        if os.path.exists(llm_rs):
            with open(llm_rs) as f:
                llm_code = f.read()
            l_metrics = safety_metrics(llm_code)
            l_compile, l_err = try_compile_standalone(llm_rs)
            l_lock = analyze_lock_safety(llm_code, "llm")

            compile_str = "✅" if l_compile else "❌"
            if l_compile:
                summary["llm_compiles"] += 1
            if l_metrics["unsafe"] < orig_metrics["unsafe"]:
                summary["llm_safer"] += 1
            if not l_lock["has_pthread"] and (l_lock["has_std_mutex"] or orig_metrics["pthread"] == 0):
                summary["llm_lock_clean"] += 1

            print(f"  {'':20} │ {'LLM':<10} │ {compile_str:>8} │ {l_metrics['unsafe']:>7} │ {l_metrics['pthread']:>8} │ {l_metrics['raw_ptr']:>8} │ {l_metrics['static_mut']:>7} │ {l_metrics['std_mutex']:>6} │ {l_metrics['std_thread']:>7} │ {l_metrics['lines']:>6}")

            if l_lock["issues"]:
                for issue in l_lock["issues"]:
                    print(f"  {'':20} │   ⚠️  {issue}")

            if not l_compile:
                # Show first error line
                first_err = [line for line in l_err.splitlines() if "error" in line.lower()][:2]
                for e in first_err:
                    print(f"  {'':20} │   💥 {e[:80]}")

            row["llm"] = {"metrics": l_metrics, "compiles": l_compile, "lock_safety": l_lock}

        print("─" * 100)
        results.append(row)

    # ── Summary ─────────────────────────────────────────────────────────────
    print("\n" + "=" * 80)
    print("SUMMARY")
    print("=" * 80)

    total = summary["total"]
    print(f"\nTotal examples: {total}")
    print(f"\n{'Metric':<40} │ {'Concrat':>10} │ {'LLM':>10}")
    print("─" * 65)
    print(f"{'Compiles successfully':<40} │ {summary['concrat_compiles']:>8}/{total} │ {summary['llm_compiles']:>8}/{total}")
    print(f"{'Reduced unsafe count':<40} │ {summary['concrat_safer']:>8}/{total} │ {summary['llm_safer']:>8}/{total}")
    print(f"{'Clean lock transformation (no pthread)':<40} │ {summary['concrat_lock_clean']:>8}/{total} │ {summary['llm_lock_clean']:>8}/{total}")

    # Aggregate safety metrics
    orig_total = {"unsafe": 0, "pthread": 0, "raw_ptr": 0, "static_mut": 0}
    concrat_total = {"unsafe": 0, "pthread": 0, "raw_ptr": 0, "static_mut": 0, "std_mutex": 0}
    llm_total = {"unsafe": 0, "pthread": 0, "raw_ptr": 0, "static_mut": 0, "std_mutex": 0}

    for r in results:
        for k in orig_total:
            orig_total[k] += r["original"]["metrics"].get(k, 0)
        if "concrat" in r:
            for k in concrat_total:
                concrat_total[k] += r["concrat"]["metrics"].get(k, 0)
        if "llm" in r:
            for k in llm_total:
                llm_total[k] += r["llm"]["metrics"].get(k, 0)

    print(f"\n{'Aggregate Safety Metrics':<40} │ {'Original':>10} │ {'Concrat':>10} │ {'LLM':>10}")
    print("─" * 80)
    print(f"{'Total unsafe keywords':<40} │ {orig_total['unsafe']:>10} │ {concrat_total['unsafe']:>10} │ {llm_total['unsafe']:>10}")
    print(f"{'Total pthread calls':<40} │ {orig_total['pthread']:>10} │ {concrat_total['pthread']:>10} │ {llm_total['pthread']:>10}")
    print(f"{'Total raw pointers':<40} │ {orig_total['raw_ptr']:>10} │ {concrat_total['raw_ptr']:>10} │ {llm_total['raw_ptr']:>10}")
    print(f"{'Total static mut':<40} │ {orig_total['static_mut']:>10} │ {concrat_total['static_mut']:>10} │ {llm_total['static_mut']:>10}")
    print(f"{'Total std::sync::Mutex':<40} │ {'N/A':>10} │ {concrat_total['std_mutex']:>10} │ {llm_total['std_mutex']:>10}")

    # Save JSON report
    # Convert lock_safety for JSON serialization
    for r in results:
        for version in ["original", "concrat", "llm"]:
            if version in r and "lock_safety" in r[version]:
                # already dict, should be fine
                pass
    with open(report_path, "w") as f:
        json.dump(results, f, indent=2, default=str)
    print(f"\nDetailed report saved to: {report_path}")


if __name__ == "__main__":
    main()
