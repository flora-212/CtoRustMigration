#!/usr/bin/env python3
"""
Summarize evaluation results from all experiment runs.
Groups by c2rust/non-c2rust and shows pass/fail stats for each example across multiple runs.
"""

import json
import os
from pathlib import Path
from collections import defaultdict
from typing import Dict, List, Tuple

RESULT_DIR = Path("/home/guoxy/concrat/LLM/result")

def get_experiment_dirs() -> Tuple[List[Path], List[Path]]:
    """Separate experiments into c2rust and non-c2rust groups."""
    dirs = sorted([d for d in RESULT_DIR.iterdir() if d.is_dir() and not d.name.startswith('.')])
    
    c2rust_dirs = [d for d in dirs if '_c2rust' in d.name]
    non_c2rust_dirs = [d for d in dirs if '_c2rust' not in d.name]
    
    return non_c2rust_dirs, c2rust_dirs

def load_evaluation_results(eval_dir: Path) -> Dict:
    """Load all evaluation results from a single experiment."""
    results = {}
    
    # Load each evaluation type
    eval_files = {
        'loom': 'loom_results.json',
        'miri': 'miri_results.json',
        'output': 'output_verification_results.json',
        'clippy': 'clippy_concurrency_report.json',
        'compile_pass_rate': 'comparison_report.json'
    }
    
    for eval_type, filename in eval_files.items():
        filepath = eval_dir / filename
        if filepath.exists():
            try:
                with open(filepath) as f:
                    data = json.load(f)
                    results[eval_type] = data
            except Exception as e:
                print(f"Error loading {filepath}: {e}")
                results[eval_type] = {}
    
    return results

def extract_example_results(results: Dict) -> Dict[str, Dict]:
    """Extract results for each example across all evaluation types."""
    example_results = defaultdict(lambda: {
        'loom': {'pass': False, 'message': ''},
        'miri': {'pass': False, 'message': ''},
        'output': {'pass': False, 'message': ''},
        'clippy': {'pass': False, 'message': ''},
        'compile_pass_rate': {}
    })
    
    # Process loom results
    if 'loom' in results and 'results' in results['loom']:
        for example, data in results['loom']['results'].items():
            example_results[example]['loom'] = {
                'pass': data.get('success', False),
                'message': data.get('message', ''),
                'error': data.get('message', '').replace('✅', '').replace('❌', '').strip() if not data.get('success') else ''
            }
    
    # Process miri results
    if 'miri' in results and 'results' in results['miri']:
        for example, data in results['miri']['results'].items():
            example_results[example]['miri'] = {
                'pass': data.get('success', False),
                'message': data.get('message', ''),
                'error': data.get('message', '').replace('✅', '').replace('❌', '').strip() if not data.get('success') else ''
            }
    
    # Process output verification results
    if 'output' in results and 'results' in results['output']:
        for example, data in results['output']['results'].items():
            msg = data.get('message', '')
            details = data.get('details', '')
            example_results[example]['output'] = {
                'pass': data.get('success', False),
                'message': msg.replace('✅', '').replace('❌', '').strip(),
                'error': details[:200] if details else ''
            }
    
    # Process clippy results
    if 'clippy' in results and 'results' in results['clippy']:
        for item in results['clippy']['results']:
            example = item.get('name', '')
            llm_ok = item.get('llm', {}).get('ok', False)
            example_results[example]['clippy'] = {
                'pass': llm_ok,
                'message': f"LLM warnings: {item.get('llm', {}).get('warning_count', 0)}"
            }
    
    # Process comparison results
    if 'compile_pass_rate' in results and isinstance(results['compile_pass_rate'], list):
        for item in results['compile_pass_rate']:
            example = item.get('name', '')
            llm_compiles = item.get('llm', {}).get('compiles', False)
            example_results[example]['compile_pass_rate'] = {
                'compiles': llm_compiles,
                'metrics': item.get('llm', {}).get('metrics', {})
            }
    
    return dict(example_results)

def summarize_group(group_name: str, dirs: List[Path]) -> Dict[str, List]:
    """Summarize results for a group of experiments."""
    group_summary = defaultdict(lambda: {
        'loom': {'pass': [], 'fail': []},
        'miri': {'pass': [], 'fail': []},
        'output': {'pass': [], 'fail': []},
        'clippy': {'pass': [], 'fail': []},
        'compile_pass_rate': {'pass': [], 'fail': []}
    })
    
    print(f"\n{'='*80}")
    print(f"Processing {group_name} ({len(dirs)} experiments)")
    print(f"{'='*80}")
    
    for idx, exp_dir in enumerate(dirs, 1):
        print(f"  [{idx}/{len(dirs)}] Processing {exp_dir.name}...")
        
        eval_dir = exp_dir / 'evaluation'
        if not eval_dir.exists():
            print(f"    Warning: No evaluation directory found")
            continue
        
        results = load_evaluation_results(eval_dir)
        examples = extract_example_results(results)
        
        for example, data in examples.items():
            for eval_type in ['loom', 'miri', 'output', 'clippy', 'compile_pass_rate']:
                if eval_type == 'compile_pass_rate':
                    if 'compiles' in data.get('compile_pass_rate', {}):
                        if data['compile_pass_rate']['compiles']:
                            group_summary[example]['compile_pass_rate']['pass'].append({
                                'run': exp_dir.name,
                                'metrics': data['compile_pass_rate'].get('metrics', {})
                            })
                        else:
                            group_summary[example]['compile_pass_rate']['fail'].append(exp_dir.name)
                else:
                    eval_data = data.get(eval_type, {})
                    if eval_data.get('pass'):
                        group_summary[example][eval_type]['pass'].append(exp_dir.name)
                    else:
                        group_summary[example][eval_type]['fail'].append({
                            'run': exp_dir.name,
                            'error': eval_data.get('error', eval_data.get('message', ''))
                        })
    
    return dict(group_summary)

def format_output(non_c2rust_summary: Dict, c2rust_summary: Dict):
    """Format and print the comprehensive summary."""
    
    # Get all unique examples
    all_examples = set(non_c2rust_summary.keys()) | set(c2rust_summary.keys())
    all_examples = sorted(all_examples)
    
    output = []
    output.append("\n" + "="*100)
    output.append("COMPREHENSIVE EVALUATION RESULTS SUMMARY")
    output.append("="*100)
    
    for example in all_examples:
        output.append(f"\n\n{'▼'*50} Example: {example} {'▼'*50}")
        
        # Non-c2rust group
        non_c2rust = non_c2rust_summary.get(example, {})
        output.append(f"\n  ╔══ NON-C2RUST GROUP (5 runs) ══")
        
        for eval_type in ['loom', 'miri', 'output', 'clippy', 'compile_pass_rate']:
            if eval_type == 'compile_pass_rate':
                pass_count = len(non_c2rust.get('compile_pass_rate', {}).get('pass', []))
                fail_data = non_c2rust.get('compile_pass_rate', {}).get('fail', [])
                output.append(f"  ║  {eval_type.upper():12} Pass: {pass_count}/5 {'✅' if pass_count == 5 else '❌'}")
                if fail_data:
                    for item in fail_data:
                        output.append(f"  ║    └─ Failed: {item}")
            else:
                pass_count = len(non_c2rust.get(eval_type, {}).get('pass', []))
                fail_data = non_c2rust.get(eval_type, {}).get('fail', [])
                output.append(f"  ║  {eval_type.upper():12} Pass: {pass_count}/5 {'✅' if pass_count == 5 else '❌'}")
                if fail_data:
                    for item in fail_data:
                        if isinstance(item, dict):
                            error_msg = item.get('error', '')[:80]
                            output.append(f"  ║    └─ {item.get('run', 'unknown')}: {error_msg}")
                        else:
                            output.append(f"  ║    └─ Failed: {item}")
        
        # C2rust group
        c2rust = c2rust_summary.get(example, {})
        output.append(f"  ║")
        output.append(f"  ╚══ C2RUST GROUP (5 runs) ══")
        
        for eval_type in ['loom', 'miri', 'output', 'clippy', 'compile_pass_rate']:
            if eval_type == 'compile_pass_rate':
                pass_count = len(c2rust.get('compile_pass_rate', {}).get('pass', []))
                fail_data = c2rust.get('compile_pass_rate', {}).get('fail', [])
                output.append(f"     {eval_type.upper():12} Pass: {pass_count}/5 {'✅' if pass_count == 5 else '❌'}")
                if fail_data:
                    for item in fail_data:
                        output.append(f"     └─ Failed: {item}")
            else:
                pass_count = len(c2rust.get(eval_type, {}).get('pass', []))
                fail_data = c2rust.get(eval_type, {}).get('fail', [])
                output.append(f"     {eval_type.upper():12} Pass: {pass_count}/5 {'✅' if pass_count == 5 else '❌'}")
                if fail_data:
                    for item in fail_data:
                        if isinstance(item, dict):
                            error_msg = item.get('error', '')[:80]
                            output.append(f"     └─ {item.get('run', 'unknown')}: {error_msg}")
                        else:
                            output.append(f"     └─ Failed: {item}")
    
    return "\n".join(output)

def main():
    non_c2rust_dirs, c2rust_dirs = get_experiment_dirs()
    
    print(f"Found {len(non_c2rust_dirs)} non-c2rust experiments")
    print(f"Found {len(c2rust_dirs)} c2rust experiments")
    
    non_c2rust_summary = summarize_group("NON-C2RUST", non_c2rust_dirs)
    c2rust_summary = summarize_group("C2RUST", c2rust_dirs)
    
    # Print comprehensive report
    report = format_output(non_c2rust_summary, c2rust_summary)
    print(report)
    
    # Save to file
    output_file = RESULT_DIR / "comprehensive_summary.txt"
    with open(output_file, 'w') as f:
        f.write(report)
    
    print(f"\n\n📁 Summary saved to: {output_file}")

if __name__ == '__main__':
    main()
