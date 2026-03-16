#!/usr/bin/env python3
"""Generate a clear Markdown comparison report from comparison_report.json."""

import json
import os
import sys

def main():
    prompt_idx = int(sys.argv[1]) if len(sys.argv) > 1 and not sys.argv[1].startswith("--") else 0
    force = "--force" in sys.argv

    result_dir = f"/home/guoxy/concrat/LLM/result/{prompt_idx}"
    input_path = os.path.join(result_dir, "comparison_report.json")
    output_path = os.path.join(result_dir, "comparison_report.md")

    if os.path.exists(output_path) and not force:
        print(f"Report already exists: {output_path} (use --force to regenerate)")
        return

    with open(input_path) as f:
        data = json.load(f)

    lines = []
    w = lines.append

    w("# Concurrency Transformation Comparison Report")
    w("")
    w("Three-way comparison: **Original** (c2rust output) vs **ConCrat** (automated transform) vs **LLM** (LLM-based rewrite)")
    w("")

    # ── Summary table ──
    w("## Summary Overview")
    w("")
    w("| # | Example | Compiles (C / L) | unsafe | pthread | raw_ptr | static_mut | libc | std_mutex | std_thread | Lines |")
    w("|---|---------|:----------------:|--------|---------|---------|------------|------|-----------|------------|-------|")

    metric_keys = ["unsafe", "pthread", "raw_ptr", "static_mut", "libc",
                   "std_mutex", "std_arc", "std_rwlock", "std_condvar", "std_thread", "lines"]

    totals = {src: {k: 0 for k in metric_keys} for src in ["original", "concrat", "llm"]}
    compile_stats = {"concrat": {"yes": 0, "no": 0}, "llm": {"yes": 0, "no": 0}}

    for idx, item in enumerate(data, 1):
        name = item["name"]
        om = item["original"]["metrics"]
        cm = item["concrat"]["metrics"]
        lm = item["llm"]["metrics"]

        cc = "✅" if item["concrat"].get("compiles") else "❌"
        lc = "✅" if item["llm"].get("compiles") else "❌"

        compile_stats["concrat"]["yes" if item["concrat"].get("compiles") else "no"] += 1
        compile_stats["llm"]["yes" if item["llm"].get("compiles") else "no"] += 1

        for src, m in [("original", om), ("concrat", cm), ("llm", lm)]:
            for k in metric_keys:
                totals[src][k] += m.get(k, 0)

        # Show O → C → L for key metrics
        def trio(key):
            return f"{om[key]}→{cm[key]}→{lm[key]}"

        w(f"| {idx} | [{name}](#{name}) | {cc} / {lc} "
          f"| {trio('unsafe')} | {trio('pthread')} | {trio('raw_ptr')} "
          f"| {trio('static_mut')} | {trio('libc')} | {trio('std_mutex')} "
          f"| {trio('std_thread')} | {trio('lines')} |")

    # Totals row
    def ttrio(key):
        return f"{totals['original'][key]}→{totals['concrat'][key]}→{totals['llm'][key]}"

    w(f"| | **TOTAL** | {compile_stats['concrat']['yes']}/{len(data)} / {compile_stats['llm']['yes']}/{len(data)} "
      f"| {ttrio('unsafe')} | {ttrio('pthread')} | {ttrio('raw_ptr')} "
      f"| {ttrio('static_mut')} | {ttrio('libc')} | {ttrio('std_mutex')} "
      f"| {ttrio('std_thread')} | {ttrio('lines')} |")

    w("")
    w("> **Reading the table**: Each metric cell shows **Original → ConCrat → LLM**. "
      "Compiles column shows **ConCrat / LLM**.")
    w("")

    # ── Aggregate statistics ──
    w("## Aggregate Statistics")
    w("")
    w("| Metric | Original | ConCrat | LLM | Reduction (O→C) | Reduction (O→L) |")
    w("|--------|----------|---------|-----|:----------------:|:----------------:|")

    for k in metric_keys:
        o = totals["original"][k]
        c = totals["concrat"][k]
        l = totals["llm"][k]
        rc = f"{(o - c) / o * 100:.1f}%" if o > 0 else "—"
        rl = f"{(o - l) / o * 100:.1f}%" if o > 0 else "—"
        label = k.replace("_", "\\_")
        w(f"| {label} | {o} | {c} | {l} | {rc} | {rl} |")

    w("")
    w(f"| **Compile success** | — | {compile_stats['concrat']['yes']}/{len(data)} "
      f"({compile_stats['concrat']['yes']/len(data)*100:.0f}%) "
      f"| {compile_stats['llm']['yes']}/{len(data)} "
      f"({compile_stats['llm']['yes']/len(data)*100:.0f}%) | | |")
    w("")

    # ── Safety features adoption ──
    w("## Safety Features Adoption")
    w("")
    w("| Example | std::sync::Mutex | Arc<Mutex> | RwLock | Condvar | std::thread | join() |")
    w("|---------|:---:|:---:|:---:|:---:|:---:|:---:|")

    for item in data:
        name = item["name"]
        cls = item["concrat"]["lock_safety"]
        lls = item["llm"]["lock_safety"]

        def icon(c_val, l_val):
            c = "C" if c_val else "·"
            l = "L" if l_val else "·"
            return f"{c},{l}"

        w(f"| {name} "
          f"| {icon(cls['has_std_mutex'], lls['has_std_mutex'])} "
          f"| {icon(cls.get('has_arc_mutex', False), lls.get('has_arc_mutex', False))} "
          f"| {icon(item['concrat']['metrics'].get('std_rwlock',0)>0, item['llm']['metrics'].get('std_rwlock',0)>0)} "
          f"| {icon(item['concrat']['metrics'].get('std_condvar',0)>0, item['llm']['metrics'].get('std_condvar',0)>0)} "
          f"| {icon(cls.get('has_thread_spawn', False), lls.get('has_thread_spawn', False))} "
          f"| {icon(cls.get('has_join', False), lls.get('has_join', False))} |")

    w("")
    w("> **C** = ConCrat uses it, **L** = LLM uses it, **·** = not used")
    w("")

    # ── Per-example detailed cards ──
    w("## Per-Example Details")
    w("")

    for item in data:
        name = item["name"]
        w(f"### {name}")
        w("")

        om = item["original"]["metrics"]
        cm = item["concrat"]["metrics"]
        lm = item["llm"]["metrics"]

        cc = "✅ Yes" if item["concrat"].get("compiles") else "❌ No"
        lc = "✅ Yes" if item["llm"].get("compiles") else "❌ No"

        w(f"**Compiles**: ConCrat {cc} | LLM {lc}")
        w("")

        w("| Metric | Original | ConCrat | LLM | Best |")
        w("|--------|:--------:|:-------:|:---:|:----:|")

        for k in metric_keys:
            ov = om.get(k, 0)
            cv = cm.get(k, 0)
            lv = lm.get(k, 0)
            # For these metrics, lower is better (except std_mutex/std_thread/lines where context matters)
            if k in ("unsafe", "pthread", "raw_ptr", "static_mut", "libc"):
                best_val = min(cv, lv)
                if cv == lv:
                    best = "tie"
                elif cv < lv:
                    best = "ConCrat"
                else:
                    best = "LLM"
            elif k in ("std_mutex", "std_arc", "std_rwlock", "std_condvar", "std_thread"):
                # Higher is generally better (more idiomatic)
                best_val = max(cv, lv)
                if cv == lv:
                    best = "tie"
                elif cv > lv:
                    best = "ConCrat"
                else:
                    best = "LLM"
            else:  # lines
                if cv == lv:
                    best = "tie"
                elif cv < lv:
                    best = "ConCrat"
                else:
                    best = "LLM"
            label = k.replace("_", "\\_")
            w(f"| {label} | {ov} | {cv} | {lv} | {best} |")

        # Issues
        c_issues = item["concrat"]["lock_safety"].get("issues", [])
        l_issues = item["llm"]["lock_safety"].get("issues", [])

        if c_issues or l_issues:
            w("")
            w("**Remaining Issues:**")
            w("")
            if c_issues:
                w("- **ConCrat**: " + "; ".join(c_issues))
            if l_issues:
                w("- **LLM**: " + "; ".join(l_issues))

        w("")
        w("---")
        w("")

    with open(output_path, "w") as f:
        f.write("\n".join(lines))

    print(f"Generated: {output_path}")


if __name__ == "__main__":
    main()
