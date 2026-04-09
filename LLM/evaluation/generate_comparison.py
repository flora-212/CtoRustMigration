#!/usr/bin/env python3
"""Generate Markdown comparison reports from comparison_report.json files."""

import json
import os
import sys
import re

def get_round_from_metadata(sample_name, examples_dir):
    """
    Extract round number from rounds_metadata.json for a sample.
    Returns the last successful round number, or "c2rust" if no round compiled successfully.
    """
    metadata_path = os.path.join(examples_dir, sample_name, "rounds_metadata.json")
    if not os.path.exists(metadata_path):
        return "c2rust"
    
    try:
        with open(metadata_path) as f:
            data = json.load(f)
            # Find last successful round
            last_successful_round = None
            for round_key in sorted(data.keys(), key=lambda x: int(x)):
                if data[round_key].get("compile_status"):
                    last_successful_round = round_key
            
            # Return last successful round if found, otherwise "c2rust"
            return last_successful_round if last_successful_round else "c2rust"
    except (json.JSONDecodeError, ValueError, KeyError):
        pass
    
    return "c2rust"


def generate_markdown_report(data, report_type, output_path, positive_only_data=None, round_num=None, get_sample_round=None):
    """
    Generate Markdown report from comparison data.
    
    report_type: "all", "positive_only", or "negative_only"
    positive_only_data: optional list of positive samples (used for negative report lookup)
    round_num: optional round number to display in tables (current processing round)
    get_sample_round: optional function to get sample's first successful round number
    """
    lines = []
    w = lines.append

    # Header based on report type
    if report_type == "positive_only":
        title = "Concurrency Transformation Comparison Report (Positive Samples Only)"
        subtitle = "Comparing **Original** vs **ConCrat** vs **LLM** for positive examples"
    elif report_type == "negative_only":
        title = "Concurrency Transformation Comparison Report (Negative Samples Only)"
        subtitle = "Analyzing **Original** and **LLM** for negative examples (expected to fail)"
    else:  # all
        title = "Concurrency Transformation Comparison Report"
        subtitle = "Three-way comparison: **Original** (c2rust output) vs **ConCrat** (automated transform) vs **LLM** (LLM-based rewrite)"

    w(f"# {title}")
    w("")
    w(subtitle)
    w("")

    # ── Summary table ──
    w("## Summary Overview")
    w("")
    
    if report_type == "negative_only":
        w("| # | Example | Type | Compiles (L) | Round | Pos | Pos Round | unsafe | pthread | raw_ptr | static_mut | libc | Lines |")
        w("|---|---------|------|:----------:|:---:|:--:|:----------:|--------|---------|---------|------------|------|-------|")
    else:
        w("| # | Example | Compiles (C / L) | Round | unsafe | pthread | raw_ptr | static_mut | libc | std_mutex | std_thread | Lines |")
        w("|---|---------|:----------------:|:---:|--------|---------|---------|------------|------|-----------|------------|-------|")

    metric_keys = ["unsafe", "pthread", "raw_ptr", "static_mut", "libc",
                   "std_mutex", "std_arc", "std_rwlock", "std_condvar", "std_thread", "lines"]

    totals = {src: {k: 0 for k in metric_keys} for src in ["original", "concrat", "llm"]}
    compile_stats = {"concrat": {"yes": 0, "no": 0}, "llm": {"yes": 0, "no": 0}}
    sample_count = 0
    
    # Build a map of positive samples for quick lookup (used for negative sample status)
    positive_samples = {}
    if report_type == "negative_only" or report_type == "all":
        # Use provided positive data if available (for negative_only reports)
        positive_data_src = positive_only_data if positive_only_data else data
        for item in positive_data_src:
            if not item.get("is_negative", False):
                # Get sample round from metadata if available
                sample_round = None
                if get_sample_round:
                    sample_round = get_sample_round(item["name"])
                
                positive_samples[item["name"]] = {
                    "compiles": item.get("llm", {}).get("compiles", False),
                    "round": sample_round,  # Round from metadata
                    "item": item
                }

    for idx, item in enumerate(data, 1):
        name = item["name"]
        is_negative = item.get("is_negative", False)
        om = item["original"]["metrics"]

        # Skip based on report type filter
        if report_type == "negative_only" and not is_negative:
            continue
        if report_type == "positive_only" and is_negative:
            continue

        # For negative samples, show Original → LLM only
        if is_negative:
            # Negative samples may not have LLM output
            if "llm" in item:
                lm = item["llm"]["metrics"]
                lc = "✅" if item["llm"].get("compiles") else "❌"
                compile_stats["llm"]["yes" if item["llm"].get("compiles") else "no"] += 1
                for k in metric_keys:
                    totals["llm"][k] += lm.get(k, 0)
            else:
                # No LLM attempt for this negative sample
                lc = "⚠"  # Not attempted
                lm = None
            
            for k in metric_keys:
                totals["original"][k] += om.get(k, 0)
                if lm:
                    totals["llm"][k] += lm.get(k, 0)
            
            # Extract base sample name (remove ____xxx suffix) to find corresponding positive sample
            base_name = name.split("____")[0]
            pos_info = positive_samples.get(base_name, {})
            pos_compiles = "✅" if pos_info.get("compiles") else "❌"
            pos_round = pos_info.get("round") or "—"
            
            # Get this sample's round
            sample_round = get_sample_round(name) if get_sample_round else "—"
            
            if lm:
                w(f"| {sample_count + 1} | [{name}](#{name}) | NEG | {lc} | {sample_round} | {pos_compiles} | {pos_round} "
                  f"| {om['unsafe']}→{lm.get('unsafe', 0)} | {om['pthread']}→{lm.get('pthread', 0)} | {om['raw_ptr']}→{lm.get('raw_ptr', 0)} "
                  f"| {om['static_mut']}→{lm.get('static_mut', 0)} | {om['libc']}→{lm.get('libc', 0)} "
                  f"| {om['lines']}→{lm.get('lines', 0)} |")
            else:
                # No attempt
                w(f"| {sample_count + 1} | [{name}](#{name}) | NEG | {lc} | {sample_round} | {pos_compiles} | {pos_round} "
                  f"| {om['unsafe']}→— | {om['pthread']}→— | {om['raw_ptr']}→— "
                  f"| {om['static_mut']}→— | {om['libc']}→— "
                  f"| {om['lines']}→— |")
            
            sample_count += 1
        else:
            # Positive samples (show ConCrat)
            if "concrat" in item:
                cm = item["concrat"]["metrics"]
                
                # Only count compile stats if both exist
                if "llm" in item:
                    lm = item["llm"]["metrics"]
                    cc = "✅" if item["concrat"].get("compiles") else "❌"
                    lc = "✅" if item["llm"].get("compiles") else "❌"
                    compile_stats["concrat"]["yes" if item["concrat"].get("compiles") else "no"] += 1
                    compile_stats["llm"]["yes" if item["llm"].get("compiles") else "no"] += 1
                else:
                    cc = "✅" if item["concrat"].get("compiles") else "❌"
                    lc = "⚠"  # No LLM output
                    compile_stats["concrat"]["yes" if item["concrat"].get("compiles") else "no"] += 1
                    lm = None

                for k in metric_keys:
                    totals["original"][k] += om.get(k, 0)
                    totals["concrat"][k] += cm.get(k, 0)
                    if lm:
                        totals["llm"][k] += lm.get(k, 0)

                # Get this sample's round
                sample_round = get_sample_round(name) if get_sample_round else "—"

                if lm:
                    w(f"| {sample_count + 1} | [{name}](#{name}) | {cc} / {lc} | {sample_round} "
                      f"| {om.get('unsafe', 0)}→{cm.get('unsafe', 0)}→{lm.get('unsafe', 0)} | {om.get('pthread', 0)}→{cm.get('pthread', 0)}→{lm.get('pthread', 0)} | {om.get('raw_ptr', 0)}→{cm.get('raw_ptr', 0)}→{lm.get('raw_ptr', 0)} "
                      f"| {om.get('static_mut', 0)}→{cm.get('static_mut', 0)}→{lm.get('static_mut', 0)} | {om.get('libc', 0)}→{cm.get('libc', 0)}→{lm.get('libc', 0)} | {om.get('std_mutex', 0)}→{cm.get('std_mutex', 0)}→{lm.get('std_mutex', 0)} "
                      f"| {om.get('std_thread', 0)}→{cm.get('std_thread', 0)}→{lm.get('std_thread', 0)} | {om.get('lines', 0)}→{cm.get('lines', 0)}→{lm.get('lines', 0)} |")
                else:
                    # No LLM output - show ConCrat vs Original only
                    w(f"| {sample_count + 1} | [{name}](#{name}) | {cc} / {lc} | {sample_round} "
                      f"| {om.get('unsafe', 0)}→{cm.get('unsafe', 0)}→— | {om.get('pthread', 0)}→{cm.get('pthread', 0)}→— | {om.get('raw_ptr', 0)}→{cm.get('raw_ptr', 0)}→— "
                      f"| {om.get('static_mut', 0)}→{cm.get('static_mut', 0)}→— | {om.get('libc', 0)}→{cm.get('libc', 0)}→— | {om.get('std_mutex', 0)}→{cm.get('std_mutex', 0)}→— "
                      f"| {om.get('std_thread', 0)}→{cm.get('std_thread', 0)}→— | {om.get('lines', 0)}→{cm.get('lines', 0)}→— |")
                
                sample_count += 1

    # Totals row
    if report_type == "negative_only":
        total_llm = compile_stats['llm']['yes']
        total_count = compile_stats['llm']['yes'] + compile_stats['llm']['no']
        # Count positive samples that compile
        pos_compile_count = sum(1 for v in positive_samples.values() if v.get("compiles"))
        w(f"| | **TOTAL** | (NEG) | {total_llm}/{total_count} | — | {pos_compile_count}/{len(positive_samples)} | — "
          f"| {totals['original']['unsafe']}→{totals['llm'].get('unsafe', 0)} "
          f"| {totals['original']['pthread']}→{totals['llm'].get('pthread', 0)} "
          f"| {totals['original']['raw_ptr']}→{totals['llm'].get('raw_ptr', 0)} "
          f"| {totals['original']['static_mut']}→{totals['llm'].get('static_mut', 0)} "
          f"| {totals['original']['libc']}→{totals['llm'].get('libc', 0)} "
          f"| {totals['original']['lines']}→{totals['llm'].get('lines', 0)} |")
    else:
        def ttrio(key):
            if totals["concrat"][key] == 0 and totals["llm"][key] == 0:
                return f"{totals['original'][key]}→0→0"
            return f"{totals['original'][key]}→{totals['concrat'].get(key, 0)}→{totals['llm'].get(key, 0)}"

        total_count = compile_stats['concrat']['yes'] + compile_stats['concrat'].get('no', 0)
        if total_count == 0:
            total_count = sample_count

        w(f"| | **TOTAL** | {compile_stats['concrat']['yes']}/{total_count} / {compile_stats['llm']['yes']}/{total_count} | — "
          f"| {ttrio('unsafe')} | {ttrio('pthread')} | {ttrio('raw_ptr')} "
          f"| {ttrio('static_mut')} | {ttrio('libc')} | {ttrio('std_mutex')} "
          f"| {ttrio('std_thread')} | {ttrio('lines')} |")

    w("")
    
    if report_type != "negative_only":
        w("> **Reading the table**: Each metric cell shows **Original → ConCrat → LLM**. "
          "Compiles column shows **ConCrat / LLM**.")
    else:
        w("> **Reading the table**: Each metric cell shows **Original → LLM**. "
          "**Pos** column shows if the corresponding positive sample (before `____`) compiles with LLM. "
          "**Pos Round** shows the last successful round (1-N) for the positive sample, or `c2rust` if none compiled successfully. "
          "Negative samples are expected to fail (used for validation).")
    w("")

    # ── Aggregate statistics ──
    w("## Aggregate Statistics")
    w("")
    
    if report_type == "negative_only":
        w("| Metric | Original | LLM | vs Original |")
        w("|--------|----------|-----|:------------:|")
        for k in ["unsafe", "pthread", "raw_ptr", "static_mut", "libc", "lines"]:
            o = totals["original"][k]
            l = totals["llm"][k]
            diff = f"{(o - l) / o * 100:+.1f}%" if o > 0 else "—"
            label = k.replace("_", "\\_")
            w(f"| {label} | {o} | {l} | {diff} |")
    else:
        w("| Metric | Original | ConCrat | LLM | Reduction (O→C) | Reduction (O→L) |")
        w("|--------|----------|---------|-----|:----------------:|:----------------:|")

        for k in metric_keys:
            o = totals["original"][k]
            c = totals["concrat"].get(k, 0)
            l = totals["llm"].get(k, 0)
            rc = f"{(o - c) / o * 100:.1f}%" if o > 0 else "—"
            rl = f"{(o - l) / o * 100:.1f}%" if o > 0 else "—"
            label = k.replace("_", "\\_")
            w(f"| {label} | {o} | {c} | {l} | {rc} | {rl} |")

    w("")
    
    total_for_stats = sample_count if sample_count > 0 else len(data)
    if report_type != "negative_only":
        w(f"| **Compile success** | — | {compile_stats['concrat']['yes']}/{total_for_stats} "
          f"({(compile_stats['concrat']['yes']/total_for_stats*100 if total_for_stats > 0 else 0):.0f}%) "
          f"| {compile_stats['llm']['yes']}/{total_for_stats} "
          f"({(compile_stats['llm']['yes']/total_for_stats*100 if total_for_stats > 0 else 0):.0f}%) | | |")
    else:
        total_for_llm = compile_stats['llm']['yes'] + compile_stats['llm'].get('no', 0)
        w(f"| **LLM compile success** | — | {compile_stats['llm']['yes']}/{total_for_llm} "
          f"({(compile_stats['llm']['yes']/total_for_llm*100 if total_for_llm > 0 else 0):.0f}%) |  |")
    w("")

    # ── Safety features adoption ──
    if report_type != "negative_only":
        w("## Safety Features Adoption")
        w("")
        w("| Example | Round | std::sync::Mutex | Arc<Mutex> | RwLock | Condvar | std::thread | join() |")
        w("|---------|:---:|:---:|:---:|:---:|:---:|:---:|:---:|")

        for item in data:
            if item.get("is_negative", False):
                continue  # Skip negative samples in this section
            
            if "concrat" not in item:
                continue
                
            name = item["name"]
            cls = item["concrat"]["lock_safety"]
            lls = item["llm"]["lock_safety"] if "llm" in item else {}

            def icon(c_val, l_val):
                c = "C" if c_val else "·"
                l = "L" if l_val else "·"
                return f"{c},{l}"

            # Get this sample's round
            sample_round = get_sample_round(name) if get_sample_round else "—"

            w(f"| {name} | {sample_round} "
              f"| {icon(cls['has_std_mutex'], lls.get('has_std_mutex', False))} "
              f"| {icon(cls.get('has_arc_mutex', False), lls.get('has_arc_mutex', False))} "
              f"| {icon(item['concrat']['metrics'].get('std_rwlock',0)>0, item['llm']['metrics'].get('std_rwlock',0)>0 if 'llm' in item else False)} "
              f"| {icon(item['concrat']['metrics'].get('std_condvar',0)>0, item['llm']['metrics'].get('std_condvar',0)>0 if 'llm' in item else False)} "
              f"| {icon(cls.get('has_thread_spawn', False), lls.get('has_thread_spawn', False))} "
              f"| {icon(cls.get('has_join', False), lls.get('has_join', False))} |")

        w("")
        w("> **C** = ConCrat uses it, **L** = LLM uses it, **·** = not used")
        w("")

    # ── Per-example detailed cards ──
    w("## Per-Example Details")
    w("")

    for item in data:
        if report_type == "negative_only" and not item.get("is_negative", False):
            continue
        if report_type == "positive_only" and item.get("is_negative", False):
            continue
        
        name = item["name"]
        w(f"### {name}")
        w("")

        om = item["original"]["metrics"]
        
        # Handle negative samples (no concrat data, may not have llm data)
        if item.get("is_negative", False):
            if "llm" in item:
                lm = item["llm"]["metrics"]
                lc = "✅ Yes" if item["llm"].get("compiles") else "❌ No"
                w(f"**Compiles**: LLM {lc}")
                w("")
                w("| Metric | Original | LLM | Difference |")
                w("|--------|:--------:|:---:|:----------:|")
                for k in ["unsafe", "pthread", "raw_ptr", "static_mut", "libc", "lines"]:
                    ov = om.get(k, 0)
                    lv = lm.get(k, 0)
                    diff = lv - ov
                    diff_str = f"{diff:+d}"
                    w(f"| {k} | {ov} | {lv} | {diff_str} |")
                
                l_issues = item["llm"]["lock_safety"].get("issues", [])
                if l_issues:
                    w("")
                    w("**Remaining Issues:**")
                    w("")
                    w("- **LLM**: " + "; ".join(l_issues))
            else:
                # No LLM attempt for this negative sample
                w("**Type**: Negative (expected to fail) - LLM output not attempted")
                w("")
                w("| Metric | Original |")
                w("|--------|:--------:|")
                for k in ["unsafe", "pthread", "raw_ptr", "static_mut", "libc", "lines"]:
                    ov = om.get(k, 0)
                    w(f"| {k} | {ov} |")
                
                o_issues = item["original"]["lock_safety"].get("issues", [])
                if o_issues:
                    w("")
                    w("**Issues in Original:**")
                    w("")
                    w("- " + "; ".join(o_issues))
        else:
            # Positive samples
            if "concrat" in item:
                cm = item["concrat"]["metrics"]
                
                if "llm" in item:
                    lm = item["llm"]["metrics"]
                    cc = "✅ Yes" if item["concrat"].get("compiles") else "❌ No"
                    lc = "✅ Yes" if item["llm"].get("compiles") else "❌ No"
                else:
                    cc = "✅ Yes" if item["concrat"].get("compiles") else "❌ No"
                    lc = "⚠ Not attempted"
                    lm = None

                w(f"**Compiles**: ConCrat {cc} | LLM {lc}")
                w("")

                w("| Metric | Original | ConCrat | LLM | Best |")
                w("|--------|:--------:|:-------:|:---:|:----:|")

                for k in metric_keys:
                    ov = om.get(k, 0)
                    cv = cm.get(k, 0)
                    lv = lm.get(k, 0) if lm else 0
                    
                    # For these metrics, lower is better
                    if k in ("unsafe", "pthread", "raw_ptr", "static_mut", "libc"):
                        best_val = min(cv, lv)
                        if not lm:
                            best = "ConCrat"
                        elif cv == lv:
                            best = "tie"
                        elif cv < lv:
                            best = "ConCrat"
                        else:
                            best = "LLM"
                    elif k in ("std_mutex", "std_arc", "std_rwlock", "std_condvar", "std_thread"):
                        # Higher is generally better (more idiomatic)
                        best_val = max(cv, lv)
                        if not lm:
                            best = "ConCrat"
                        elif cv == lv:
                            best = "tie"
                        elif cv > lv:
                            best = "ConCrat"
                        else:
                            best = "LLM"
                    else:  # lines
                        if not lm:
                            best = "ConCrat"
                        elif cv == lv:
                            best = "tie"
                        elif cv < lv:
                            best = "ConCrat"
                        else:
                            best = "LLM"
                    label = k.replace("_", "\\_")
                    
                    if lm:
                        w(f"| {label} | {ov} | {cv} | {lv} | {best} |")
                    else:
                        w(f"| {label} | {ov} | {cv} | — | {best} |")

                # Issues
                c_issues = item["concrat"]["lock_safety"].get("issues", [])
                l_issues = item["llm"]["lock_safety"].get("issues", []) if "llm" in item else []

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

    return output_path


def main():
    prompt_idx = int(sys.argv[1]) if len(sys.argv) > 1 and not sys.argv[1].startswith("--") else 0
    force = "--force" in sys.argv

    # Parse --llm-output-dir
    llm_output_dir = None
    for i, arg in enumerate(sys.argv):
        if arg == "--llm-output-dir" and i + 1 < len(sys.argv):
            llm_output_dir = sys.argv[i + 1]
            break

    # Determine input/output directory - try multiple locations
    input_dirs = []
    
    if llm_output_dir:
        # Primary: timestamped output directory evaluation path
        input_dirs.append(os.path.join(llm_output_dir, "evaluation"))
        # Fallback: timestamped output directory root (compare_all.py might save there)
        input_dirs.append(llm_output_dir)
    else:
        # Try to read from .last_refactor_output file first
        last_output_file = "/home/guoxy/concrat/LLM/.last_refactor_output"
        if os.path.exists(last_output_file):
            with open(last_output_file) as f:
                last_output_dir = f.read().strip()
                if last_output_dir and os.path.isdir(last_output_dir):
                    # Timestamped directory - primary in evaluation/
                    input_dirs.append(os.path.join(last_output_dir, "evaluation"))
                    # Fallback in root
                    input_dirs.append(last_output_dir)
        
        # Also try legacy result/{prompt_idx} path
        legacy_dir = f"/home/guoxy/concrat/LLM/result/{prompt_idx}"
        if os.path.isdir(legacy_dir):
            input_dirs.append(legacy_dir)
    
    # Process all three report types
    report_versions = [
        ("all", "comparison_report.json", "comparison_report.md"),
        ("positive_only", "comparison_report_positive_only.json", "comparison_report_positive_only.md"),
        ("negative_only", "comparison_report_negative_only.json", "comparison_report_negative_only.md"),
    ]
    
    for report_type, input_file, output_file in report_versions:
        # Try to find input file in any of the directories
        input_path = None
        input_dir = None
        output_dir = None
        
        for candidate_dir in input_dirs:
            candidate_path = os.path.join(candidate_dir, input_file)
            if os.path.exists(candidate_path):
                input_path = candidate_path
                input_dir = candidate_dir
                output_dir = candidate_dir
                break
        
        if not input_path:
            print(f"⚠️  Input file not found: {input_file}")
            print(f"    Searched in: {', '.join(input_dirs)}")
            continue
        
        # Find examples directory (for reading rounds_metadata.json)
        examples_dir = None
        if input_dir.endswith('/evaluation') or input_dir.endswith('evaluation'):
            # If input_dir is evaluation/, check parent for examples
            parent_dir = os.path.dirname(input_dir.rstrip('/'))
            if os.path.isdir(os.path.join(parent_dir, 'examples')):
                examples_dir = os.path.join(parent_dir, 'examples')
        else:
            # If input_dir is root, check for examples
            if os.path.isdir(os.path.join(input_dir, 'examples')):
                examples_dir = os.path.join(input_dir, 'examples')
        
        # Create a function to get sample rounds
        def get_sample_round(sample_name):
            if examples_dir:
                return get_round_from_metadata(sample_name, examples_dir)
            return None
        
        output_path = os.path.join(output_dir, output_file)

        if os.path.exists(output_path) and not force:
            print(f"Report already exists: {output_path} (use --force to regenerate)")
            continue

        with open(input_path) as f:
            try:
                data = json.load(f)
            except json.JSONDecodeError as e:
                print(f"❌ Failed to parse {input_file}: {e}")
                continue

        if not data:
            print(f"⚠️  No data in {input_file}, skipping {output_file}")
            continue

        # Extract round number from directory path (format: YYYYmmdd_hhmmss_N[_description])
        round_num = None
        # Try to extract from input_dir path - check both current dir and parent if in evaluation/
        check_dir = input_dir.rstrip('/')
        if check_dir.endswith('/evaluation') or check_dir.endswith('evaluation'):
            # If in evaluation subdir, use parent
            check_dir = os.path.dirname(check_dir)
        
        dir_name = os.path.basename(check_dir)
        # Match format: YYYYmmdd_hhmmss_N[_optional_suffix]
        match = re.search(r'^\d{8}_\d{6}_(\d+)(?:_|$)', dir_name)
        if match:
            round_num = match.group(1)

        # For negative_only reports, also load positive_only data for lookup
        positive_only_data = None
        if report_type == "negative_only":
            positive_file = "comparison_report_positive_only.json"
            for candidate_dir in input_dirs:
                positive_path = os.path.join(candidate_dir, positive_file)
                if os.path.exists(positive_path):
                    with open(positive_path) as f:
                        try:
                            positive_only_data = json.load(f)
                        except json.JSONDecodeError:
                            pass
                    break

        output = generate_markdown_report(data, report_type, output_path, positive_only_data, round_num, get_sample_round)
        print(f"✅ Generated: {output}")


if __name__ == "__main__":
    main()
