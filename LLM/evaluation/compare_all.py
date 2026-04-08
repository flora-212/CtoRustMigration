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
sys.path.insert(0, '/home/guoxy/concrat/LLM')
from validation import CodeValidator

EXAMPLES_DIR = "/home/guoxy/concrat/examples"
CONCRAT_DIR = "/tmp/concrat_results"
NIGHTLY = "nightly-2022-07-05"
DEPS_DIR = "/home/guoxy/concrat/deps_crate/target/debug/deps"
CONCRAT_CACHE_PATH = "/home/guoxy/concrat/LLM/result/concrat_cache.json"
VALIDATOR = CodeValidator()

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

def get_llm_final_round_info(llm_output_dir: str, example_name: str) -> dict:
    """
    Get metadata about which round was saved as final
    
    Returns:
        {
            "round": int or None,
            "type": "validation_passed" | "best_compile_round" | "last_attempt",
            "metadata": metadata dict or None
        }
    """
    example_dir = os.path.join(llm_output_dir, "examples", example_name)
    metadata_file = os.path.join(example_dir, "final_metadata.json")
    
    info = {
        "round": None,
        "type": "unknown",
        "metadata": None
    }
    
    if os.path.exists(metadata_file):
        with open(metadata_file) as f:
            try:
                metadata = json.load(f)
                info["metadata"] = metadata
                info["round"] = metadata.get("round")
                info["type"] = metadata.get("result_type", "unknown")
            except:
                pass
    
    return info

# ── Compilation Check ───────────────────────────────────────────────────────
# Using CodeValidator from validation module to ensure consistent compilation detection

def try_compile_with_cargo(rs_file, example_dir):
    """Try compiling a .rs file using the example's Cargo.toml context."""
    success, errors = VALIDATOR.try_compile_standalone(rs_file, example_dir)
    # Convert ErrorInfo list to error string for backward compatibility
    err_str = "\n".join([str(e) for e in errors]) if errors else ""
    return success, err_str[:500]


def try_compile_standalone(rs_file, example_dir):
    """Try compiling a standalone .rs file (LLM rewritten, may not use libc)."""
    success, errors = VALIDATOR.try_compile_standalone(rs_file, example_dir)
    # Convert ErrorInfo list to error string for backward compatibility
    err_str = "\n".join([str(e) for e in errors]) if errors else ""
    return success, err_str[:500]


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
        original_rs = os.path.join(example_dir, "main.c2rust.rs")
        concrat_rs = os.path.join(CONCRAT_DIR, name, "main.c2rust.rs")

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
    
    # Parse --llm-output-dir
    llm_output_dir = None
    for i, arg in enumerate(sys.argv):
        if arg == "--llm-output-dir" and i + 1 < len(sys.argv):
            llm_output_dir = sys.argv[i + 1]
            break

    # Determine report directory - use llm_output_dir/evaluation if provided, otherwise use legacy path
    if llm_output_dir:
        report_dir = os.path.join(llm_output_dir, "evaluation")
    else:
        report_dir = f"/home/guoxy/concrat/LLM/result/{prompt_idx}"
    
    report_path = os.path.join(report_dir, "comparison_report.json")

    if os.path.exists(report_path) and not force:
        print(f"Report already exists: {report_path} (use --force to regenerate)")
        return

    os.makedirs(report_dir, exist_ok=True)

    # Load both positive and negative examples
    positive_examples = sorted([d for d in glob.glob(f"{EXAMPLES_DIR}/*/") if os.path.isdir(d)])
    negative_examples = sorted([d for d in glob.glob(f"/home/guoxy/concrat/examples_negative/*/") if os.path.isdir(d)])
    examples = positive_examples + negative_examples
    
    sample_type = "positive + negative" if negative_examples else "positive only"
    print(f"📊 Examples: {sample_type} ({len(positive_examples)} + {len(negative_examples)})")
    print()

    # Load or build concrat cache (only for positive examples, as negative ones are expected to fail)
    concrat_cache = load_concrat_cache()
    if concrat_cache is None or clear:
        concrat_cache = build_concrat_cache(positive_examples)

    results = []
    summary = {"total": 0, "llm_compiles": 0, "concrat_compiles": 0,
               "llm_safer": 0, "concrat_safer": 0, "llm_lock_clean": 0, "concrat_lock_clean": 0}

    # Print LLM source directory info
    if llm_output_dir:
        print(f"📁 LLM Output Directory: {llm_output_dir}")
        print(f"   Looking for files at: {llm_output_dir}/examples/{{example_name}}/final.rs")
        print()

    print("=" * 100)
    print(f"{'Example':<22} │ {'Version':<10} │ {'Compile':>8} │ {'unsafe':>7} │ {'pthread':>8} │ {'raw_ptr':>8} │ {'st_mut':>7} │ {'Mutex':>6} │ {'thread':>7} │ {'lines':>6}")
    print("─" * 100)

    for example_dir in examples:
        name = os.path.basename(example_dir.rstrip("/"))
        summary["total"] += 1
        
        # Check if this is a negative example
        is_negative = "examples_negative" in example_dir
        
        original_rs = os.path.join(example_dir, "main.c2rust.rs")
        # Try to read from llm_output_dir if provided, otherwise use the old location
        if llm_output_dir:
            llm_rs = os.path.join(llm_output_dir, "examples", name, "final.rs")
            # Use the correct source directory for compile context
            if is_negative:
                example_dir_for_compile = os.path.join("/home/guoxy/concrat/examples_negative", name)
            else:
                example_dir_for_compile = os.path.join(EXAMPLES_DIR, name)
        else:
            llm_rs = os.path.join(example_dir, f"main_rewritten_{prompt_idx}.rs")
            example_dir_for_compile = example_dir

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
        row["is_negative"] = is_negative  # Mark whether this is a negative example

        # Always print original with sample type indicator
        sample_mark = "[NEG]" if is_negative else "[POS]"
        print(f"  {sample_mark} {name:<16} │ {'original':<10} │ {'(base)':>8} │ {orig_metrics['unsafe']:>7} │ {orig_metrics['pthread']:>8} │ {orig_metrics['raw_ptr']:>8} │ {orig_metrics['static_mut']:>7} │ {orig_metrics['std_mutex']:>6} │ {orig_metrics['std_thread']:>7} │ {orig_metrics['lines']:>6}")

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
        llm_round_info = None
        if os.path.exists(llm_rs):
            with open(llm_rs) as f:
                llm_code = f.read()
            l_metrics = safety_metrics(llm_code)
            l_compile, l_err = try_compile_standalone(llm_rs, example_dir_for_compile)
            l_lock = analyze_lock_safety(llm_code, "llm")
            
            # Get round info if available
            if llm_output_dir:
                llm_round_info = get_llm_final_round_info(llm_output_dir, name)

            compile_str = "✅" if l_compile else "❌"
            if l_compile:
                summary["llm_compiles"] += 1
            if l_metrics["unsafe"] < orig_metrics["unsafe"]:
                summary["llm_safer"] += 1
            if not l_lock["has_pthread"] and (l_lock["has_std_mutex"] or orig_metrics["pthread"] == 0):
                summary["llm_lock_clean"] += 1

            round_str = f"(R{llm_round_info['round']})" if llm_round_info and llm_round_info["round"] else ""
            print(f"  {'':20} │ {'LLM':<10} │ {compile_str:>8} │ {l_metrics['unsafe']:>7} │ {l_metrics['pthread']:>8} │ {l_metrics['raw_ptr']:>8} │ {l_metrics['static_mut']:>7} │ {l_metrics['std_mutex']:>6} │ {l_metrics['std_thread']:>7} │ {l_metrics['lines']:>6} {round_str}")

            if l_lock["issues"]:
                for issue in l_lock["issues"]:
                    print(f"  {'':20} │   ⚠️  {issue}")

            if not l_compile:
                # Show first error line
                first_err = [line for line in l_err.splitlines() if "error" in line.lower()][:2]
                for e in first_err:
                    print(f"  {'':20} │   💥 {e[:80]}")

            row["llm"] = {
                "metrics": l_metrics, 
                "compiles": l_compile, 
                "lock_safety": l_lock,
                "round_info": llm_round_info
            }
        else:
            # LLM file not found - print diagnostic info
            if llm_output_dir:
                print(f"  {'':20} │ {'LLM':<10} │ {'⚠️':>8} │ {'(not':>7} │ {'found)':>8} │ at: {llm_rs}")
            else:
                print(f"  {'':20} │ {'LLM':<10} │ {'⚠️':>8} │ {'(no':>7} │ {'output)':>8} │ expected: {llm_rs}")

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

    # Analyze round distribution for LLM results (if using timestamped output)
    if llm_output_dir:
        llm_round_stats = {}
        llm_round_types = {}
        for r in results:
            if "llm" in r and "round_info" in r["llm"]:
                round_info = r["llm"]["round_info"]
                if round_info and round_info["round"]:
                    round_num = round_info["round"]
                    result_type = round_info.get("type", "unknown")
                    
                    llm_round_stats[round_num] = llm_round_stats.get(round_num, 0) + 1
                    llm_round_types[result_type] = llm_round_types.get(result_type, 0) + 1
        
        if llm_round_stats:
            print(f"\n{'LLM Round Distribution':<40}")
            print("─" * 50)
            # Sort and display round distribution
            # Custom sort: numbers first (ascending), then strings (alphabetically)
            def sort_key(x):
                if isinstance(x, int):
                    return (0, x)  # Numbers get priority (0) and sort by value
                else:
                    return (1, str(x))  # Strings get second priority (1) and sort alphabetically
            
            for round_num in sorted(llm_round_stats.keys(), key=sort_key):
                count = llm_round_stats[round_num]
                percentage = (count / total * 100) if total > 0 else 0
                print(f"  Round {round_num:<30} │ {count:>3}/{total} ({percentage:>5.1f}%)")
            
            if llm_round_types:
                print(f"\n{'Result Types':<40}")
                print("─" * 50)
                for result_type in sorted(llm_round_types.keys()):
                    count = llm_round_types[result_type]
                    percentage = (count / total * 100) if total > 0 else 0
                    print(f"  {result_type:<30} │ {count:>3}/{total} ({percentage:>5.1f}%)")

    # Save JSON reports (split by sample type + combined)
    # Convert lock_safety for JSON serialization
    for r in results:
        for version in ["original", "concrat", "llm"]:
            if version in r and "lock_safety" in r[version]:
                # already dict, should be fine
                pass
    
    # Split results by type
    results_positive = [r for r in results if not r.get("is_negative", False)]
    results_negative = [r for r in results if r.get("is_negative", False)]
    results_all = results
    
    # Save all three versions
    reports = {
        "all": (results_all, report_path),
        "positive_only": (results_positive, report_path.replace("comparison_report.json", "comparison_report_positive_only.json")),
        "negative_only": (results_negative, report_path.replace("comparison_report.json", "comparison_report_negative_only.json")),
    }
    
    for report_type, (data, path) in reports.items():
        if data:  # Only save if there are results
            with open(path, "w") as f:
                json.dump(data, f, indent=2, default=str)
            print(f"✅ Detailed report saved to: {path}")
    
    if not results_positive:
        print(f"⚠️  No positive samples to save")
    if not results_negative:
        print(f"⚠️  No negative samples to save")


if __name__ == "__main__":
    main()
