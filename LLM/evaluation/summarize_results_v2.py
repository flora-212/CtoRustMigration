#!/usr/bin/env python3
"""
Enhanced evaluation results summarizer with both text and JSON output.
Groups by c2rust/non-c2rust and provides detailed pass/fail statistics.
"""

import json
import os
from pathlib import Path
from collections import defaultdict
from typing import Dict, List, Tuple
from datetime import datetime

RESULT_DIR = Path("/home/guoxy/concrat/LLM/result")
SUMMARY_DIR = RESULT_DIR / "summary"

def get_experiment_dirs() -> Tuple[List[Path], List[Path]]:
    """Separate experiments into c2rust and non-c2rust groups."""
    dirs = sorted([d for d in RESULT_DIR.iterdir() if d.is_dir() and not d.name.startswith('.')])
    
    c2rust_dirs = [d for d in dirs if '_c2rust' in d.name]
    non_c2rust_dirs = [d for d in dirs if '_c2rust' not in d.name]
    
    return non_c2rust_dirs, c2rust_dirs

def load_evaluation_results(eval_dir: Path) -> Dict:
    """Load all evaluation results from a single experiment."""
    results = {}
    
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
                print(f"Warning: Error loading {filepath}: {e}")
                results[eval_type] = {}
    
    return results

def extract_example_results(results: Dict) -> Dict[str, Dict]:
    """Extract results for each example across all evaluation types."""
    example_results = defaultdict(lambda: {
        'loom': {'pass': False, 'message': '', 'error': ''},
        'miri': {'pass': False, 'message': '', 'error': ''},
        'output': {'pass': False, 'message': '', 'error': ''},
        'clippy': {'pass': False, 'message': '', 'error': ''},
        'compile_pass_rate': {'compiles': False, 'metrics': {}}
    })
    
    # Loom
    if 'loom' in results and 'results' in results['loom']:
        for example, data in results['loom']['results'].items():
            success = data.get('success', False)
            message = data.get('message', '').replace('✅', '').replace('❌', '').strip()
            example_results[example]['loom'] = {
                'pass': success,
                'message': message,
                'error': message if not success else ''
            }
    
    # Miri
    if 'miri' in results and 'results' in results['miri']:
        for example, data in results['miri']['results'].items():
            success = data.get('success', False)
            message = data.get('message', '').replace('✅', '').replace('❌', '').strip()
            example_results[example]['miri'] = {
                'pass': success,
                'message': message,
                'error': message if not success else ''
            }
    
    # Output verification
    if 'output' in results and 'results' in results['output']:
        for example, data in results['output']['results'].items():
            success = data.get('success', False)
            msg = data.get('message', '').replace('✅', '').replace('❌', '').strip()
            details = data.get('details', '')
            example_results[example]['output'] = {
                'pass': success,
                'message': msg,
                'error': details[:200] if details and not success else ''
            }
    
    # Clippy
    if 'clippy' in results and 'results' in results['clippy']:
        for item in results['clippy']['results']:
            example = item.get('name', '')
            llm_data = item.get('llm', {})
            llm_ok = llm_data.get('ok', False)
            warnings = llm_data.get('warning_count', 0)
            example_results[example]['clippy'] = {
                'pass': llm_ok,
                'message': f"Warnings: {warnings}",
                'error': '' if llm_ok else f"Clippy warnings: {warnings}"
            }
    
    # Compile pass rate
    if 'compile_pass_rate' in results and isinstance(results['compile_pass_rate'], list):
        for item in results['compile_pass_rate']:
            example = item.get('name', '')
            llm_data = item.get('llm', {})
            compiles = llm_data.get('compiles', False)
            metrics = llm_data.get('metrics', {})
            
            # Extract iteration info
            round_info = llm_data.get('round_info', {})
            iterations = None
            if round_info and isinstance(round_info, dict):
                r = round_info.get('round')
                if isinstance(r, int):
                    iterations = r
            
            example_results[example]['compile_pass_rate'] = {
                'compiles': compiles,
                'metrics': metrics,
                'iterations': iterations
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
    
    print(f"Processing {group_name} ({len(dirs)} experiments)")
    
    for idx, exp_dir in enumerate(dirs, 1):
        print(f"  [{idx}/{len(dirs)}] {exp_dir.name}...", end='', flush=True)
        
        eval_dir = exp_dir / 'evaluation'
        if not eval_dir.exists():
            print(" ⚠ no evaluation dir")
            continue
        
        results = load_evaluation_results(eval_dir)
        examples = extract_example_results(results)
        
        for example, data in examples.items():
            for eval_type in ['loom', 'miri', 'output', 'clippy', 'compile_pass_rate']:
                if eval_type == 'compile_pass_rate':
                    compiles = data.get('compile_pass_rate', {}).get('compiles', False)
                    iterations = data.get('compile_pass_rate', {}).get('iterations')
                    if compiles:
                        group_summary[example]['compile_pass_rate']['pass'].append({
                            'run': exp_dir.name,
                            'metrics': data['compile_pass_rate'].get('metrics', {}),
                            'iterations': iterations
                        })
                    else:
                        group_summary[example]['compile_pass_rate']['fail'].append({
                            'run': exp_dir.name,
                            'iterations': iterations
                        })
                else:
                    eval_data = data.get(eval_type, {})
                    if eval_data.get('pass'):
                        group_summary[example][eval_type]['pass'].append(exp_dir.name)
                    else:
                        group_summary[example][eval_type]['fail'].append({
                            'run': exp_dir.name,
                            'error': eval_data.get('error', '')
                        })
        
        print(" ✓")
    
    # Calculate average iterations for compile_pass_rate
    for example in group_summary:
        pass_data = group_summary[example]['compile_pass_rate']['pass']
        fail_data = group_summary[example]['compile_pass_rate']['fail']
        
        # Collect all iteration values
        all_iters = []
        for item in pass_data:
            if isinstance(item, dict) and item.get('iterations') is not None:
                all_iters.append(item['iterations'])
        for item in fail_data:
            if isinstance(item, dict) and item.get('iterations') is not None:
                all_iters.append(item['iterations'])
        
        # Calculate average
        if all_iters:
            avg = sum(all_iters) / len(all_iters)
            group_summary[example]['compile_pass_rate']['avg_iterations'] = f"{avg:.1f}"
    
    return dict(group_summary)

def generate_json_report(non_c2rust_summary: Dict, c2rust_summary: Dict) -> Dict:
    """Generate comprehensive JSON report."""
    all_examples = set(non_c2rust_summary.keys()) | set(c2rust_summary.keys())
    all_examples = sorted(all_examples)
    
    report = {
        'timestamp': datetime.now().isoformat(),
        'total_examples': len(all_examples),
        'groups': {
            'non_c2rust': {'runs': 5, 'summary': {}},
            'c2rust': {'runs': 5, 'summary': {}}
        },
        'examples': {}
    }
    
    # Initialize group summaries
    for eval_type in ['loom', 'miri', 'output', 'clippy', 'compile_pass_rate']:
        report['groups']['non_c2rust']['summary'][eval_type] = {'passed': 0, 'failed': 0}
        report['groups']['c2rust']['summary'][eval_type] = {'passed': 0, 'failed': 0}
    
    for example in all_examples:
        report['examples'][example] = {
            'non_c2rust': {},
            'c2rust': {}
        }
        
        # Process non-c2rust group
        non_c2rust = non_c2rust_summary.get(example, {})
        for eval_type in ['loom', 'miri', 'output', 'clippy', 'compile_pass_rate']:
            data = non_c2rust.get(eval_type, {})
            if eval_type == 'compile_pass_rate':
                pass_count = len(data.get('pass', []))
                fail_count = len(data.get('fail', []))
                
                # Calculate average iterations
                all_iters = []
                for item in data.get('pass', []):
                    if isinstance(item, dict) and item.get('iterations') is not None:
                        all_iters.append(item['iterations'])
                for item in data.get('fail', []):
                    if isinstance(item, dict) and item.get('iterations') is not None:
                        all_iters.append(item['iterations'])
                
                avg_iters = sum(all_iters) / len(all_iters) if all_iters else None
            else:
                pass_count = len(data.get('pass', []))
                fail_count = len(data.get('fail', []))
                avg_iters = None
            
            report_entry = {
                'passed': pass_count,
                'failed': fail_count,
                'pass_rate': f"{pass_count}/5",
                'failures': data.get('fail', [])
            }
            if eval_type == 'compile_pass_rate' and avg_iters is not None:
                report_entry['avg_iterations'] = f"{avg_iters:.1f}"
            
            report['examples'][example]['non_c2rust'][eval_type] = report_entry
            report['groups']['non_c2rust']['summary'][eval_type]['passed'] += pass_count
            report['groups']['non_c2rust']['summary'][eval_type]['failed'] += fail_count
        
        # Process c2rust group
        c2rust = c2rust_summary.get(example, {})
        for eval_type in ['loom', 'miri', 'output', 'clippy', 'compile_pass_rate']:
            data = c2rust.get(eval_type, {})
            if eval_type == 'compile_pass_rate':
                pass_count = len(data.get('pass', []))
                fail_count = len(data.get('fail', []))
                
                # Calculate average iterations
                all_iters = []
                for item in data.get('pass', []):
                    if isinstance(item, dict) and item.get('iterations') is not None:
                        all_iters.append(item['iterations'])
                for item in data.get('fail', []):
                    if isinstance(item, dict) and item.get('iterations') is not None:
                        all_iters.append(item['iterations'])
                
                avg_iters = sum(all_iters) / len(all_iters) if all_iters else None
            else:
                pass_count = len(data.get('pass', []))
                fail_count = len(data.get('fail', []))
                avg_iters = None
            
            report_entry = {
                'passed': pass_count,
                'failed': fail_count,
                'pass_rate': f"{pass_count}/5",
                'failures': data.get('fail', [])
            }
            if eval_type == 'compile_pass_rate' and avg_iters is not None:
                report_entry['avg_iterations'] = f"{avg_iters:.1f}"
            
            report['examples'][example]['c2rust'][eval_type] = report_entry
            
            report['groups']['c2rust']['summary'][eval_type]['passed'] += pass_count
            report['groups']['c2rust']['summary'][eval_type]['failed'] += fail_count
    
    return report

def format_text_report(non_c2rust_summary: Dict, c2rust_summary: Dict) -> str:
    """Format comprehensive text report."""
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
                avg_iterations = non_c2rust.get('compile_pass_rate', {}).get('avg_iterations')
                status = '✅' if pass_count == 5 else '❌'
                iter_str = f" | 平均迭代: {avg_iterations}" if avg_iterations else ""
                output.append(f"     {'编译通过率':12} Pass: {pass_count}/5 {status}{iter_str}")
                if fail_data:
                    for item in fail_data[:3]:  # Show first 3 failures
                        output.append(f"  ║    └─ Failed: {item}")
            else:
                pass_count = len(non_c2rust.get(eval_type, {}).get('pass', []))
                fail_data = non_c2rust.get(eval_type, {}).get('fail', [])
                status = '✅' if pass_count == 5 else '❌'
                output.append(f"  ║  {eval_type.upper():12} Pass: {pass_count}/5 {status}")
                if fail_data:
                    for item in fail_data[:3]:  # Show first 3 failures
                        if isinstance(item, dict):
                            error_msg = item.get('error', '')[:70]
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
                avg_iterations = c2rust.get('compile_pass_rate', {}).get('avg_iterations')
                status = '✅' if pass_count == 5 else '❌'
                iter_str = f" | 平均迭代: {avg_iterations}" if avg_iterations else ""
                output.append(f"     {'编译通过率':12} Pass: {pass_count}/5 {status}{iter_str}")
                if fail_data:
                    for item in fail_data[:3]:
                        output.append(f"     └─ Failed: {item}")
            else:
                pass_count = len(c2rust.get(eval_type, {}).get('pass', []))
                fail_data = c2rust.get(eval_type, {}).get('fail', [])
                status = '✅' if pass_count == 5 else '❌'
                output.append(f"     {eval_type.upper():12} Pass: {pass_count}/5 {status}")
                if fail_data:
                    for item in fail_data[:3]:
                        if isinstance(item, dict):
                            error_msg = item.get('error', '')[:70]
                            output.append(f"     └─ {item.get('run', 'unknown')}: {error_msg}")
                        else:
                            output.append(f"     └─ Failed: {item}")
    
    return "\n".join(output)

def main():
    print("Starting comprehensive evaluation results summarization...")
    print("="*80)
    
    non_c2rust_dirs, c2rust_dirs = get_experiment_dirs()
    
    print(f"\nFound {len(non_c2rust_dirs)} non-c2rust experiments")
    print(f"Found {len(c2rust_dirs)} c2rust experiments\n")
    
    non_c2rust_summary = summarize_group("NON-C2RUST", non_c2rust_dirs)
    c2rust_summary = summarize_group("C2RUST", c2rust_dirs)
    
    # Generate reports
    print("\nGenerating reports...")
    text_report = format_text_report(non_c2rust_summary, c2rust_summary)
    json_report = generate_json_report(non_c2rust_summary, c2rust_summary)
    
    # Save text report
    text_file = SUMMARY_DIR / "comprehensive_summary.txt"
    with open(text_file, 'w') as f:
        f.write(text_report)
    print(f"✓ Text report: {text_file}")
    
    # Save JSON report
    json_file = SUMMARY_DIR / "comprehensive_summary.json"
    with open(json_file, 'w') as f:
        json.dump(json_report, f, indent=2)
    print(f"✓ JSON report: {json_file}")
    
    # Print summary statistics
    print("\n" + "="*80)
    print("SUMMARY STATISTICS")
    print("="*80)
    print("\nNON-C2RUST GROUP:")
    for eval_type in ['loom', 'miri', 'output', 'clippy', 'compile_pass_rate']:
        stats = json_report['groups']['non_c2rust']['summary'][eval_type]
        total = stats['passed'] + stats['failed']
        rate = f"{stats['passed']}/{total}" if total > 0 else "N/A"
        print(f"  {eval_type.upper():12} {rate:6} examples passed")
    
    print("\nC2RUST GROUP:")
    for eval_type in ['loom', 'miri', 'output', 'clippy', 'compile_pass_rate']:
        stats = json_report['groups']['c2rust']['summary'][eval_type]
        total = stats['passed'] + stats['failed']
        rate = f"{stats['passed']}/{total}" if total > 0 else "N/A"
        print(f"  {eval_type.upper():12} {rate:6} examples passed")

if __name__ == '__main__':
    main()
