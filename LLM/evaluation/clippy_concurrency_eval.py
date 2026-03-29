#!/usr/bin/env python3
"""Run Clippy-based concurrency safety evaluation for Original/Concrat/LLM outputs."""

import glob
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile

EXAMPLES_DIR = "/home/guoxy/concrat/examples"
CONCRAT_DIR = "/tmp/concrat_results"
NIGHTLY = "nightly-2022-07-05"

CONCURRENCY_CLIPPY_LINTS = [
    "clippy::arc_with_non_send_sync",
    "clippy::await_holding_lock",
    "clippy::mutex_atomic",
    "clippy::mut_mutex_lock",
]


def run_clippy(rs_file: str, example_dir: str) -> dict:
    """Run cargo clippy in a temporary Cargo context for one Rust file."""
    td = tempfile.mkdtemp()
    try:
        for f in ["Cargo.toml", "rust-toolchain"]:
            src = os.path.join(example_dir, f)
            if os.path.exists(src):
                shutil.copy(src, td)

        lib_rs = os.path.join(example_dir, "c2rust-lib.rs")
        if os.path.exists(lib_rs):
            shutil.copy(lib_rs, td)

        cargo_toml = os.path.join(td, "Cargo.toml")
        if not os.path.exists(cargo_toml):
            return {
                "ok": False,
                "warning_count": 0,
                "unique_lints": [],
                "lint_hits": {},
                "excerpt": "missing Cargo.toml",
            }

        shutil.copy(rs_file, os.path.join(td, "main.c2rust.rs"))

        nightly = NIGHTLY
        tc_file = os.path.join(td, "rust-toolchain")
        if os.path.exists(tc_file):
            with open(tc_file, "r") as f:
                nightly = f.read().strip()

        cmd = [
            "cargo",
            f"+{nightly}",
            "clippy",
            "--manifest-path",
            cargo_toml,
            "--",
            *[flag for lint in CONCURRENCY_CLIPPY_LINTS for flag in ("-W", lint)],
        ]

        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=120,
            env={**os.environ, "RUSTFLAGS": "-Awarnings"},
        )

        output = (result.stdout or "") + "\n" + (result.stderr or "")
        lint_matches = re.findall(r"clippy::[a-z0-9_]+", output)
        lint_hits = {}
        for lint in lint_matches:
            lint_hits[lint] = lint_hits.get(lint, 0) + 1

        unique_lints = sorted(lint_hits.keys())
        return {
            "ok": result.returncode == 0,
            "warning_count": sum(lint_hits.values()),
            "unique_lints": unique_lints,
            "lint_hits": lint_hits,
            "excerpt": "\n".join(output.splitlines()[:16]),
        }
    except Exception as e:
        return {
            "ok": False,
            "warning_count": 0,
            "unique_lints": [],
            "lint_hits": {},
            "excerpt": str(e)[:300],
        }
    finally:
        shutil.rmtree(td, ignore_errors=True)


def evaluate_one(label: str, rs_file: str, example_dir: str) -> dict:
    if not os.path.exists(rs_file):
        return {
            "exists": False,
            "ok": False,
            "warning_count": 0,
            "unique_lints": [],
            "lint_hits": {},
            "excerpt": "file not found",
        }

    result = run_clippy(rs_file, example_dir)
    result["exists"] = True
    return result


def main() -> None:
    prompt_idx = int(sys.argv[1]) if len(sys.argv) > 1 and not sys.argv[1].startswith("--") else 0
    force = "--force" in sys.argv
    
    # Parse --llm-output-dir
    llm_output_dir = None
    for i, arg in enumerate(sys.argv):
        if arg == "--llm-output-dir" and i + 1 < len(sys.argv):
            llm_output_dir = sys.argv[i + 1]
            break

    # Determine result directory - use llm_output_dir/evaluation if provided, otherwise use legacy path
    if llm_output_dir:
        result_dir = os.path.join(llm_output_dir, "evaluation")
    else:
        result_dir = f"/home/guoxy/concrat/LLM/result/{prompt_idx}"
    
    json_path = os.path.join(result_dir, "clippy_concurrency_report.json")
    md_path = os.path.join(result_dir, "clippy_concurrency_report.md")

    if os.path.exists(json_path) and not force:
        print(f"Report already exists: {json_path} (use --force to regenerate)")
        return

    os.makedirs(result_dir, exist_ok=True)

    examples = sorted([d for d in glob.glob(f"{EXAMPLES_DIR}/*/") if os.path.isdir(d)])

    # Print LLM source directory info
    if llm_output_dir:
        print(f"📁 LLM Output Directory: {llm_output_dir}")
        print(f"   Looking for files at: {llm_output_dir}/examples/{{example_name}}/final.rs")
        print()

    rows = []
    summary = {
        "total": 0,
        "concrat_clean": 0,
        "llm_clean": 0,
        "concrat_warn_total": 0,
        "llm_warn_total": 0,
    }

    print("=" * 110)
    print(f"{'Example':<24} | {'Version':<10} | {'clippy_ok':<10} | {'warnings':>8} | {'Top lint(s)':<50}")
    print("-" * 110)

    for example_dir in examples:
        name = os.path.basename(example_dir.rstrip("/"))
        summary["total"] += 1

        original_rs = os.path.join(example_dir, "main.c2rust.rs")
        concrat_rs = os.path.join(CONCRAT_DIR, name, "main.c2rust.rs")
        # Try to read from llm_output_dir if provided, otherwise use the old location
        if llm_output_dir:
            llm_rs = os.path.join(llm_output_dir, "examples", name, "final.rs")
        else:
            llm_rs = os.path.join(example_dir, f"main_rewritten_{prompt_idx}.rs")

        row = {"name": name}
        row["original"] = evaluate_one("original", original_rs, example_dir)
        row["concrat"] = evaluate_one("concrat", concrat_rs, example_dir)
        row["llm"] = evaluate_one("llm", llm_rs, example_dir)

        for version in ["original", "concrat", "llm"]:
            item = row[version]
            ok_text = "yes" if item["ok"] else "no"
            lint_text = ", ".join(item["unique_lints"][:2]) if item["unique_lints"] else "-"
            prefix = name if version == "original" else ""
            
            # Add diagnostic info if file not found
            diagnostic = ""
            if version == "llm" and not item["exists"]:
                if llm_output_dir:
                    diagnostic = f" 📍 not at {llm_rs}"
                else:
                    diagnostic = f" 📍 expected {llm_rs}"
            
            print(
                f"{prefix:<24} | {version:<10} | {ok_text:<10} | {item['warning_count']:>8} | {lint_text:<50}{diagnostic}"
            )

        summary["concrat_warn_total"] += row["concrat"]["warning_count"]
        summary["llm_warn_total"] += row["llm"]["warning_count"]
        if row["concrat"]["exists"] and row["concrat"]["warning_count"] == 0:
            summary["concrat_clean"] += 1
        if row["llm"]["exists"] and row["llm"]["warning_count"] == 0:
            summary["llm_clean"] += 1

        print("-" * 110)
        rows.append(row)

    with open(json_path, "w") as f:
        json.dump({"summary": summary, "results": rows}, f, indent=2, default=str)

    md_lines = []
    w = md_lines.append
    w("# Clippy Concurrency Evaluation")
    w("")
    w("Lints checked: " + ", ".join(f"`{x}`" for x in CONCURRENCY_CLIPPY_LINTS))
    w("")
    w("## Summary")
    w("")
    w(f"- Total examples: {summary['total']}")
    w(f"- Concrat warning-free examples: {summary['concrat_clean']}")
    w(f"- LLM warning-free examples: {summary['llm_clean']}")
    w(f"- Concrat total warning hits: {summary['concrat_warn_total']}")
    w(f"- LLM total warning hits: {summary['llm_warn_total']}")
    w("")
    w("## Detail")
    w("")
    w("| Example | Version | clippy_ok | warnings | lints |")
    w("|---|---|---|---:|---|")

    for row in rows:
        for version in ["original", "concrat", "llm"]:
            item = row[version]
            w(
                f"| {row['name']} | {version} | {'yes' if item['ok'] else 'no'} | "
                f"{item['warning_count']} | {', '.join(item['unique_lints']) if item['unique_lints'] else '-'} |"
            )

    with open(md_path, "w") as f:
        f.write("\n".join(md_lines) + "\n")

    print("\nSummary")
    print("=" * 80)
    print(f"Concrat warning-free: {summary['concrat_clean']}/{summary['total']}")
    print(f"LLM warning-free: {summary['llm_clean']}/{summary['total']}")
    print(f"Concrat warning hits: {summary['concrat_warn_total']}")
    print(f"LLM warning hits: {summary['llm_warn_total']}")
    print(f"\nSaved JSON: {json_path}")
    print(f"Saved Markdown: {md_path}")


if __name__ == "__main__":
    main()
