#!/usr/bin/env python3
"""Generate Markdown comparison reports from comparison_report.json files."""

import json
import os
import sys
import re

# Ensure we can import from the same directory
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from metadata_utils import (
    get_round_from_metadata,
    extract_round_from_dirname,
    find_examples_dir,
    find_input_file
)
from report_config import (
    REPORT_TYPE_POSITIVE_ONLY,
    REPORT_TYPE_NEGATIVE_ONLY,
    REPORT_TYPE_ALL,
    ALL_METRICS,
    NEGATIVE_SAMPLE_METRICS,
    METRICS_LOWER_IS_BETTER,
    METRICS_HIGHER_IS_BETTER,
    IS_NEGATIVE_FIELD,
    NEGATIVE_SAMPLE_SEPARATOR,
    extract_positive_sample_name,
    is_negative_sample,
    get_metric_display_name,
    is_lower_better,
    is_higher_better
)


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
    if report_type == REPORT_TYPE_POSITIVE_ONLY:
        title = "Concurrency Transformation Comparison Report (Positive Samples Only)"
        subtitle = "Comparing **Original** vs **ConCrat** vs **LLM** for positive examples"
    elif report_type == REPORT_TYPE_NEGATIVE_ONLY:
        title = "Concurrency Transformation Comparison Report (Negative Samples Only)"
        subtitle = "Analyzing **Original** and **LLM** for negative examples (expected to fail)"
    else:  # REPORT_TYPE_ALL
        title = "Concurrency Transformation Comparison Report"
        subtitle = "Three-way comparison: **Original** (c2rust output) vs **ConCrat** (automated transform) vs **LLM** (LLM-based rewrite)"

    w(f"# {title}")
    w("")
    w(subtitle)
    w("")

    # ── Summary table ──
    w("## Summary Overview")
    w("")
    
    # Define which metrics to show in the quick summary table
    summary_metrics = ["unsafe", "pthread", "raw_ptr", "static_mut", "libc", "std_mutex", "std_thread"]
    
    if report_type == REPORT_TYPE_NEGATIVE_ONLY:
        # Build header for negative samples
        header_cols = "| # | Example | Type | Compiles (L) | Round | Pos | Pos Round |"
        for m in summary_metrics:
            header_cols += f" {get_metric_display_name(m)} |"
        w(header_cols)
        
        # Build separator
        sep_cols = "|---|---------|------|:----------:|:---:|:--:|:----------:|"
        for m in summary_metrics:
            sep_cols += "------|"
        w(sep_cols)
    else:
        # Build header for positive/all samples
        header_cols = "| # | Example | Compiles (C / L) | Round |"
        for m in summary_metrics:
            header_cols += f" {get_metric_display_name(m)} |"
        w(header_cols)
        
        # Build separator
        sep_cols = "|---|---------|:----------------:|:---:|"
        for m in summary_metrics:
            sep_cols += "------|"
        w(sep_cols)

    metric_keys = ALL_METRICS

    totals = {src: {k: 0 for k in metric_keys} for src in ["original", "concrat", "llm"]}
    compile_stats = {"concrat": {"yes": 0, "no": 0}, "llm": {"yes": 0, "no": 0}}
    sample_count = 0
    
    # Build a map of positive samples for quick lookup (used for negative sample status)
    positive_samples = {}
    if report_type == REPORT_TYPE_NEGATIVE_ONLY or report_type == REPORT_TYPE_ALL:
        # Use provided positive data if available (for negative_only reports)
        positive_data_src = positive_only_data if positive_only_data else data
        for item in positive_data_src:
            if not item.get(IS_NEGATIVE_FIELD, False):
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
        is_negative = item.get(IS_NEGATIVE_FIELD, False)
        om = item["original"]["metrics"]

        # Skip based on report type filter
        if report_type == REPORT_TYPE_NEGATIVE_ONLY and not is_negative:
            continue
        if report_type == REPORT_TYPE_POSITIVE_ONLY and is_negative:
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
            base_name = extract_positive_sample_name(name)
            pos_info = positive_samples.get(base_name, {})
            pos_compiles = "✅" if pos_info.get("compiles") else "❌"
            pos_round = pos_info.get("round") or "—"
            
            # Get this sample's round
            sample_round = get_sample_round(name) if get_sample_round else "—"
            
            if lm:
                row = f"| {sample_count + 1} | [{name}](#{name}) | NEG | {lc} | {sample_round} | {pos_compiles} | {pos_round} |"
                for m in summary_metrics:
                    row += f" {om.get(m, 0)}→{lm.get(m, 0)} |"
                w(row)
            else:
                # No attempt
                row = f"| {sample_count + 1} | [{name}](#{name}) | NEG | {lc} | {sample_round} | {pos_compiles} | {pos_round} |"
                for m in summary_metrics:
                    row += f" {om.get(m, 0)}→— |"
                w(row)
            
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
                    row = f"| {sample_count + 1} | [{name}](#{name}) | {cc} / {lc} | {sample_round} |"
                    for m in summary_metrics:
                        row += f" {om.get(m, 0)}→{cm.get(m, 0)}→{lm.get(m, 0)} |"
                    w(row)
                else:
                    # No LLM output - show ConCrat vs Original only
                    row = f"| {sample_count + 1} | [{name}](#{name}) | {cc} / {lc} | {sample_round} |"
                    for m in summary_metrics:
                        row += f" {om.get(m, 0)}→{cm.get(m, 0)}→— |"
                    w(row)
                
                sample_count += 1

    # Totals row
    if report_type == REPORT_TYPE_NEGATIVE_ONLY:
        total_llm = compile_stats['llm']['yes']
        total_count = compile_stats['llm']['yes'] + compile_stats['llm']['no']
        # Count positive samples that compile
        pos_compile_count = sum(1 for v in positive_samples.values() if v.get("compiles"))
        row = f"| | **TOTAL** | (NEG) | {total_llm}/{total_count} | — | {pos_compile_count}/{len(positive_samples)} | — |"
        for m in summary_metrics:
            row += f" {totals['original'][m]}→{totals['llm'].get(m, 0)} |"
        w(row)
    else:
        def ttrio(key):
            if totals["concrat"][key] == 0 and totals["llm"][key] == 0:
                return f"{totals['original'][key]}→0→0"
            return f"{totals['original'][key]}→{totals['concrat'].get(key, 0)}→{totals['llm'].get(key, 0)}"

        total_count = compile_stats['concrat']['yes'] + compile_stats['concrat'].get('no', 0)
        if total_count == 0:
            total_count = sample_count

        row = f"| | **TOTAL** | {compile_stats['concrat']['yes']}/{total_count} / {compile_stats['llm']['yes']}/{total_count} | — |"
        for m in summary_metrics:
            row += f" {ttrio(m)} |"
        w(row)

    w("")
    
    if report_type != REPORT_TYPE_NEGATIVE_ONLY:
        w("> **Reading the table**: Each metric cell shows **Original → ConCrat → LLM**. "
          "Compiles column shows **ConCrat / LLM**.")
    else:
        w("> **Reading the table**: Each metric cell shows **Original → LLM**. "
          "**Pos** column shows if the corresponding positive sample (before `____`) compiles with LLM. "
          "**Pos Round** shows the last successful round (1-N) for the positive sample, or `c2rust` if none compiled successfully. "
          "Negative samples are expected to fail (used for validation).")
    w("")

    # ── All Metrics Summary Table ──
    w("## All Metrics Summary")
    w("")
    w("This section displays all 15 metrics for each sample in a compact format.")
    w("")
    
    if report_type == REPORT_TYPE_NEGATIVE_ONLY:
        w("| Example | " + " | ".join([get_metric_display_name(m) for m in ALL_METRICS]) + " |")
        w("|---------|" + "|".join(["-----" for _ in ALL_METRICS]) + "|")
        
        for item in data:
            if not item.get(IS_NEGATIVE_FIELD, False):
                continue
            name = item["name"]
            om = item["original"]["metrics"]
            if "llm" in item:
                lm = item["llm"]["metrics"]
                row = f"| {name} |"
                for k in ALL_METRICS:
                    ov = om.get(k, 0)
                    lv = lm.get(k, 0)
                    row += f" {ov}→{lv} |"
                w(row)
            else:
                row = f"| {name} |"
                for k in ALL_METRICS:
                    ov = om.get(k, 0)
                    row += f" {ov}→— |"
                w(row)
        
        # Totals for negative samples
        row = f"| **TOTAL** |"
        for k in ALL_METRICS:
            ov = totals["original"][k]
            lv = totals["llm"].get(k, 0)
            row += f" {ov}→{lv} |"
        w(row)
    else:
        w("| Example | " + " | ".join([get_metric_display_name(m) for m in ALL_METRICS]) + " |")
        w("|---------|" + "|".join(["-----" for _ in ALL_METRICS]) + "|")
        
        for item in data:
            if item.get(IS_NEGATIVE_FIELD, False):
                continue
            if "concrat" not in item:
                continue
            
            name = item["name"]
            om = item["original"]["metrics"]
            cm = item["concrat"]["metrics"]
            lm = item["llm"]["metrics"] if "llm" in item else None
            
            row = f"| {name} |"
            for k in ALL_METRICS:
                ov = om.get(k, 0)
                cv = cm.get(k, 0)
                if lm:
                    lv = lm.get(k, 0)
                    row += f" {ov}→{cv}→{lv} |"
                else:
                    row += f" {ov}→{cv}→— |"
            w(row)
        
        # Totals for positive samples
        row = f"| **TOTAL** |"
        for k in ALL_METRICS:
            ov = totals["original"][k]
            cv = totals["concrat"].get(k, 0)
            lv = totals["llm"].get(k, 0)
            row += f" {ov}→{cv}→{lv} |"
        w(row)
    
    w("")
    w("> **All Metrics** table shows all 15 metrics (including std\\_arc, std\\_rwlock, std\\_condvar, move\\_closure, arc\\_clone, join\\_handle, arc\\_mutex\\_combo) for each sample. "
          "Format: **Original → ConCrat → LLM** (or **Original → LLM** for negative samples).")
    w("")

    # ── Aggregate statistics ──
    w("## Aggregate Statistics")
    w("")
    
    if report_type == REPORT_TYPE_NEGATIVE_ONLY:
        w("| Metric | Original | LLM | vs Original |")
        w("|--------|----------|-----|:------------:|")
        for k in ["unsafe", "pthread", "raw_ptr", "static_mut", "libc", "lines"]:
            o = totals["original"][k]
            l = totals["llm"][k]
            diff = f"{(o - l) / o * 100:+.1f}%" if o > 0 else "—"
            label = get_metric_display_name(k)
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
            label = get_metric_display_name(k)
            w(f"| {label} | {o} | {c} | {l} | {rc} | {rl} |")

    w("")
    
    total_for_stats = sample_count if sample_count > 0 else len(data)
    if report_type != REPORT_TYPE_NEGATIVE_ONLY:
        w(f"| **Compile success** | — | {compile_stats['concrat']['yes']}/{total_for_stats} "
          f"({(compile_stats['concrat']['yes']/total_for_stats*100 if total_for_stats > 0 else 0):.0f}%) "
          f"| {compile_stats['llm']['yes']}/{total_for_stats} "
          f"({(compile_stats['llm']['yes']/total_for_stats*100 if total_for_stats > 0 else 0):.0f}%) | | |")
    else:
        total_for_llm = compile_stats['llm']['yes'] + compile_stats['llm'].get('no', 0)
        w(f"| **LLM compile success** | — | {compile_stats['llm']['yes']}/{total_for_llm} "
          f"({(compile_stats['llm']['yes']/total_for_llm*100 if total_for_llm > 0 else 0):.0f}%) |  |")
    w("")

    # ── Metric Categories Summary ──
    w("## Metric Categories Summary")
    w("")
    w("Aggregate of metrics grouped by category (Lower is Better vs Higher is Better):")
    w("")
    
    if report_type == REPORT_TYPE_NEGATIVE_ONLY:
        w("| Category | Original | LLM | vs Original |")
        w("|----------|----------|-----|:------------:|")
        
        # Lower is better
        lower_total_o = sum(totals["original"].get(k, 0) for k in METRICS_LOWER_IS_BETTER)
        lower_total_l = sum(totals["llm"].get(k, 0) for k in METRICS_LOWER_IS_BETTER)
        lower_diff = f"{(lower_total_o - lower_total_l) / lower_total_o * 100:+.1f}%" if lower_total_o > 0 else "—"
        num_lower_metrics = len(METRICS_LOWER_IS_BETTER)
        lower_avg_o = round(lower_total_o / num_lower_metrics, 2)
        lower_avg_l = round(lower_total_l / num_lower_metrics, 2)
        w(f"| **Lower is Better** (∑unsafe, pthread, raw_ptr, static_mut, libc) | {lower_total_o} ({lower_avg_o}) | {lower_total_l} ({lower_avg_l}) | {lower_diff} |")
        
        # Higher is better
        higher_total_o = sum(totals["original"].get(k, 0) for k in METRICS_HIGHER_IS_BETTER)
        higher_total_l = sum(totals["llm"].get(k, 0) for k in METRICS_HIGHER_IS_BETTER)
        higher_diff = f"{(higher_total_l - higher_total_o) / higher_total_o * 100:+.1f}%" if higher_total_o > 0 else "—"
        num_higher_metrics = len(METRICS_HIGHER_IS_BETTER)
        higher_avg_o = round(higher_total_o / num_higher_metrics, 2)
        higher_avg_l = round(higher_total_l / num_higher_metrics, 2)
        w(f"| **Higher is Better** (∑std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | {higher_total_o} ({higher_avg_o}) | {higher_total_l} ({higher_avg_l}) | {higher_diff} |")
    else:
        w("| Category | Original | ConCrat | LLM | Reduction (O→C) | Reduction (O→L) |")
        w("|----------|----------|---------|-----|:----------------:|:----------------:|")
        
        # Lower is better
        lower_total_o = sum(totals["original"].get(k, 0) for k in METRICS_LOWER_IS_BETTER)
        lower_total_c = sum(totals["concrat"].get(k, 0) for k in METRICS_LOWER_IS_BETTER)
        lower_total_l = sum(totals["llm"].get(k, 0) for k in METRICS_LOWER_IS_BETTER)
        lower_rc = f"{(lower_total_o - lower_total_c) / lower_total_o * 100:.1f}%" if lower_total_o > 0 else "—"
        lower_rl = f"{(lower_total_o - lower_total_l) / lower_total_o * 100:.1f}%" if lower_total_o > 0 else "—"
        num_lower_metrics = len(METRICS_LOWER_IS_BETTER)
        lower_avg_o = round(lower_total_o / num_lower_metrics, 2)
        lower_avg_c = round(lower_total_c / num_lower_metrics, 2)
        lower_avg_l = round(lower_total_l / num_lower_metrics, 2)
        w(f"| **Lower is Better** (∑unsafe, pthread, raw_ptr, static_mut, libc) | {lower_total_o} ({lower_avg_o}) | {lower_total_c} ({lower_avg_c}) | {lower_total_l} ({lower_avg_l}) | {lower_rc} | {lower_rl} |")
        
        # Higher is better
        higher_total_o = sum(totals["original"].get(k, 0) for k in METRICS_HIGHER_IS_BETTER)
        higher_total_c = sum(totals["concrat"].get(k, 0) for k in METRICS_HIGHER_IS_BETTER)
        higher_total_l = sum(totals["llm"].get(k, 0) for k in METRICS_HIGHER_IS_BETTER)
        higher_rc = f"{(higher_total_c - higher_total_o) / higher_total_o * 100:.1f}%" if higher_total_o > 0 else "—"
        higher_rl = f"{(higher_total_l - higher_total_o) / higher_total_o * 100:.1f}%" if higher_total_o > 0 else "—"
        num_higher_metrics = len(METRICS_HIGHER_IS_BETTER)
        higher_avg_o = round(higher_total_o / num_higher_metrics, 2)
        higher_avg_c = round(higher_total_c / num_higher_metrics, 2)
        higher_avg_l = round(higher_total_l / num_higher_metrics, 2)
        w(f"| **Higher is Better** (∑std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | {higher_total_o} ({higher_avg_o}) | {higher_total_c} ({higher_avg_c}) | {higher_total_l} ({higher_avg_l}) | {higher_rc} | {higher_rl} |")
    
    w("")

    # ── Safety features adoption ──
    if report_type != REPORT_TYPE_NEGATIVE_ONLY:
        w("## Safety Features Adoption")
        w("")
        w("| Example | Round | std::sync::Mutex | Arc<Mutex> | RwLock | Condvar | std::thread | join() |")
        w("|---------|:---:|:---:|:---:|:---:|:---:|:---:|:---:|")

        for item in data:
            if item.get(IS_NEGATIVE_FIELD, False):
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
        if report_type == REPORT_TYPE_NEGATIVE_ONLY and not item.get(IS_NEGATIVE_FIELD, False):
            continue
        if report_type == REPORT_TYPE_POSITIVE_ONLY and item.get(IS_NEGATIVE_FIELD, False):
            continue
        
        name = item["name"]
        w(f"### {name}")
        w("")

        om = item["original"]["metrics"]
        
        # Handle negative samples (no concrat data, may not have llm data)
        if item.get(IS_NEGATIVE_FIELD, False):
            if "llm" in item:
                lm = item["llm"]["metrics"]
                lc = "✅ Yes" if item["llm"].get("compiles") else "❌ No"
                w(f"**Compiles**: LLM {lc}")
                w("")
                w("| Metric | Original | LLM | Difference |")
                w("|--------|:--------:|:---:|:----------:|")
                for k in NEGATIVE_SAMPLE_METRICS:
                    ov = om.get(k, 0)
                    lv = lm.get(k, 0)
                    diff = lv - ov
                    diff_str = f"{diff:+d}"
                    w(f"| {k} | {ov} | {lv} | {diff_str} |")
                
                # Category totals for negative sample
                w("")
                w("**Category Totals:**")
                w("")
                w("| Category | Original | LLM |")
                w("|----------|:--------:|:---:|")
                lower_sum_o = sum(om.get(k, 0) for k in METRICS_LOWER_IS_BETTER)
                lower_sum_l = sum(lm.get(k, 0) for k in METRICS_LOWER_IS_BETTER)
                num_lower_metrics = len(METRICS_LOWER_IS_BETTER)
                lower_avg_o = round(lower_sum_o / num_lower_metrics, 2)
                lower_avg_l = round(lower_sum_l / num_lower_metrics, 2)
                w(f"| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | {lower_sum_o} ({lower_avg_o}) | {lower_sum_l} ({lower_avg_l}) |")
                higher_sum_o = sum(om.get(k, 0) for k in METRICS_HIGHER_IS_BETTER)
                higher_sum_l = sum(lm.get(k, 0) for k in METRICS_HIGHER_IS_BETTER)
                num_higher_metrics = len(METRICS_HIGHER_IS_BETTER)
                higher_avg_o = round(higher_sum_o / num_higher_metrics, 2)
                higher_avg_l = round(higher_sum_l / num_higher_metrics, 2)
                w(f"| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | {higher_sum_o} ({higher_avg_o}) | {higher_sum_l} ({higher_avg_l}) |")
                
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
                for k in NEGATIVE_SAMPLE_METRICS:
                    ov = om.get(k, 0)
                    w(f"| {k} | {ov} |")
                
                # Category totals for negative sample (no LLM attempt)
                w("")
                w("**Category Totals:**")
                w("")
                w("| Category | Original |")
                w("|----------|:--------:|")
                lower_sum_o = sum(om.get(k, 0) for k in METRICS_LOWER_IS_BETTER)
                num_lower_metrics = len(METRICS_LOWER_IS_BETTER)
                lower_avg_o = round(lower_sum_o / num_lower_metrics, 2)
                w(f"| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | {lower_sum_o} ({lower_avg_o}) |")
                higher_sum_o = sum(om.get(k, 0) for k in METRICS_HIGHER_IS_BETTER)
                num_higher_metrics = len(METRICS_HIGHER_IS_BETTER)
                higher_avg_o = round(higher_sum_o / num_higher_metrics, 2)
                w(f"| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | {higher_sum_o} ({higher_avg_o}) |")
                
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
                    
                    # Determine if lower or higher is better for this metric
                    if is_lower_better(k):
                        best_val = min(cv, lv)
                        if not lm:
                            best = "ConCrat"
                        elif cv == lv:
                            best = "tie"
                        elif cv < lv:
                            best = "ConCrat"
                        else:
                            best = "LLM"
                    elif is_higher_better(k):
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
                    else:  # Default to lower_better for unknown metrics
                        if not lm:
                            best = "ConCrat"
                        elif cv == lv:
                            best = "tie"
                        elif cv < lv:
                            best = "ConCrat"
                        else:
                            best = "LLM"
                    label = get_metric_display_name(k)
                    
                    if lm:
                        w(f"| {label} | {ov} | {cv} | {lv} | {best} |")
                    else:
                        w(f"| {label} | {ov} | {cv} | — | {best} |")

                # Category totals for positive sample
                w("")
                w("**Category Totals:**")
                w("")
                w("| Category | Original | ConCrat | LLM |")
                w("|----------|:--------:|:-------:|:---:|")
                lower_sum_o = sum(om.get(k, 0) for k in METRICS_LOWER_IS_BETTER)
                lower_sum_c = sum(cm.get(k, 0) for k in METRICS_LOWER_IS_BETTER)
                lower_sum_l = sum(lm.get(k, 0) for k in METRICS_LOWER_IS_BETTER) if lm else 0
                num_lower_metrics = len(METRICS_LOWER_IS_BETTER)
                lower_avg_o = round(lower_sum_o / num_lower_metrics, 2)
                lower_avg_c = round(lower_sum_c / num_lower_metrics, 2)
                if lm:
                    lower_avg_l = round(lower_sum_l / num_lower_metrics, 2)
                    w(f"| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | {lower_sum_o} ({lower_avg_o}) | {lower_sum_c} ({lower_avg_c}) | {lower_sum_l} ({lower_avg_l}) |")
                else:
                    w(f"| Lower is Better (unsafe, pthread, raw_ptr, static_mut, libc) | {lower_sum_o} ({lower_avg_o}) | {lower_sum_c} ({lower_avg_c}) | — |")
                higher_sum_o = sum(om.get(k, 0) for k in METRICS_HIGHER_IS_BETTER)
                higher_sum_c = sum(cm.get(k, 0) for k in METRICS_HIGHER_IS_BETTER)
                higher_sum_l = sum(lm.get(k, 0) for k in METRICS_HIGHER_IS_BETTER) if lm else 0
                num_higher_metrics = len(METRICS_HIGHER_IS_BETTER)
                higher_avg_o = round(higher_sum_o / num_higher_metrics, 2)
                higher_avg_c = round(higher_sum_c / num_higher_metrics, 2)
                if lm:
                    higher_avg_l = round(higher_sum_l / num_higher_metrics, 2)
                    w(f"| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | {higher_sum_o} ({higher_avg_o}) | {higher_sum_c} ({higher_avg_c}) | {higher_sum_l} ({higher_avg_l}) |")
                else:
                    w(f"| Higher is Better (std_mutex, std_arc, std_rwlock, std_condvar, std_thread, move_closure, arc_clone, join_handle, arc_mutex_combo) | {higher_sum_o} ({higher_avg_o}) | {higher_sum_c} ({higher_avg_c}) | — |")

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
        (REPORT_TYPE_ALL, "comparison_report.json", "comparison_report.md"),
        (REPORT_TYPE_POSITIVE_ONLY, "comparison_report_positive_only.json", "comparison_report_positive_only.md"),
        (REPORT_TYPE_NEGATIVE_ONLY, "comparison_report_negative_only.json", "comparison_report_negative_only.md"),
    ]
    
    for report_type, input_file, output_file in report_versions:
        # Try to find input file in any of the directories
        input_path, input_dir = find_input_file(input_file, input_dirs)
        output_dir = input_dir
        
        if not input_path:
            print(f"⚠️  Input file not found: {input_file}")
            print(f"    Searched in: {', '.join(input_dirs)}")
            continue
        
        # Find examples directory (for reading rounds_metadata.json)
        examples_dir = find_examples_dir(input_dir)
        
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

        # Extract round number from directory path
        round_num = extract_round_from_dirname(input_dir)

        # For negative_only reports, also load positive_only data for lookup
        positive_only_data = None
        if report_type == REPORT_TYPE_NEGATIVE_ONLY:
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
