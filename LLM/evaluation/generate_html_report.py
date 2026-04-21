#!/usr/bin/env python3
"""
Generate HTML visualization of evaluation results.
"""

import json
from pathlib import Path

RESULT_DIR = Path("/home/guoxy/concrat/LLM/result")

def generate_html():
    # Load JSON report
    with open(RESULT_DIR / "summary" / "comprehensive_summary.json") as f:
        data = json.load(f)
    
    html = """<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Evaluation Results Summary</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            padding: 20px;
            color: #333;
        }
        .container { max-width: 1400px; margin: 0 auto; }
        
        header {
            background: white;
            padding: 30px;
            border-radius: 10px;
            margin-bottom: 20px;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        }
        header h1 { font-size: 2em; margin-bottom: 10px; color: #667eea; }
        header p { color: #666; font-size: 1.1em; }
        
        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 20px;
            margin-bottom: 30px;
        }
        
        .stat-card {
            background: white;
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        }
        
        .stat-card h3 {
            font-size: 0.9em;
            color: #666;
            text-transform: uppercase;
            margin-bottom: 10px;
            font-weight: 600;
        }
        
        .stat-card .value {
            font-size: 2em;
            font-weight: bold;
            margin-bottom: 5px;
        }
        
        .stat-card.non-c2rust { border-left: 4px solid #3498db; }
        .stat-card.c2rust { border-left: 4px solid #27ae60; }
        
        .filter-section {
            background: white;
            padding: 20px;
            border-radius: 8px;
            margin-bottom: 20px;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        }
        
        .filter-section input, .filter-section select {
            padding: 10px;
            margin-right: 10px;
            border: 1px solid #ddd;
            border-radius: 4px;
            font-size: 1em;
        }
        
        .filter-section input:focus, .filter-section select:focus {
            outline: none;
            border-color: #667eea;
            box-shadow: 0 0 5px rgba(102, 126, 234, 0.3);
        }
        
        .examples-grid {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(800px, 1fr));
            gap: 20px;
        }
        
        .example-card {
            background: white;
            border-radius: 8px;
            overflow: hidden;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
            transition: transform 0.2s;
        }
        
        .example-card:hover { transform: translateY(-5px); box-shadow: 0 8px 12px rgba(0, 0, 0, 0.15); }
        
        .example-header {
            padding: 15px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            font-weight: bold;
            font-size: 1.1em;
        }
        
        .example-content {
            padding: 20px;
        }
        
        .group-section {
            margin-bottom: 20px;
        }
        
        .group-title {
            font-weight: bold;
            margin-bottom: 10px;
            padding-bottom: 8px;
            border-bottom: 2px solid #eee;
            color: #667eea;
        }
        
        .eval-row {
            display: grid;
            grid-template-columns: 100px 1fr;
            gap: 15px;
            padding: 10px 0;
            border-bottom: 1px solid #f0f0f0;
        }
        
        .eval-row:last-child { border-bottom: none; }
        
        .eval-type {
            font-weight: 600;
            color: #333;
        }
        
        .eval-stats {
            display: flex;
            align-items: center;
            gap: 15px;
        }
        
        .pass-rate {
            padding: 4px 12px;
            border-radius: 12px;
            font-weight: bold;
            font-size: 0.9em;
        }
        
        .pass-rate.perfect { background: #d4edda; color: #155724; }
        .pass-rate.partial { background: #fff3cd; color: #856404; }
        .pass-rate.failed { background: #f8d7da; color: #721c24; }
        
        .failure-reason {
            font-size: 0.85em;
            color: #666;
            flex: 1;
        }
        
        .search-highlight { background: #fff59d; }
        
        .legend {
            background: white;
            padding: 20px;
            border-radius: 8px;
            margin-bottom: 20px;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        }
        
        .legend h3 { margin-bottom: 10px; }
        
        .legend-item {
            display: inline-block;
            margin-right: 20px;
            margin-bottom: 10px;
        }
        
        .legend-dot {
            display: inline-block;
            width: 12px;
            height: 12px;
            border-radius: 50%;
            margin-right: 5px;
            vertical-align: middle;
        }
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>🧪 Evaluation Results Summary</h1>
            <p>Comprehensive analysis across 62 examples with 5 experimental runs each</p>
        </header>
        
        <div class="legend">
            <h3>📊 Evaluation Types</h3>
            <div>
                <strong>LOOM:</strong> Concurrent execution testing
                | <strong>MIRI:</strong> Undefined behavior detection
                | <strong>OUTPUT:</strong> Output verification
                | <strong>CLIPPY:</strong> Concurrency lints
                | <strong>编译通过率:</strong> Compilation success
            </div>
        </div>
        
        <div class="stats-grid">
"""
    
    # Add summary statistics
    for group_name, group_key in [("NON-C2RUST", "non_c2rust"), ("C2RUST", "c2rust")]:
        summary = data['groups'][group_key]['summary']
        for eval_type in ['loom', 'miri', 'output', 'clippy', 'compile_pass_rate']:
            stats = summary[eval_type]
            total = stats['passed'] + stats['failed']
            pct = (stats['passed'] / total * 100) if total > 0 else 0
            html += f"""            <div class="stat-card {group_key}">
                <h3>{group_name} - {eval_type.upper()}</h3>
                <div class="value">{stats['passed']}/{total}</div>
                <div style="font-size: 0.9em; color: #999;">{pct:.1f}% passed</div>
            </div>
"""
    
    html += """        </div>
        
        <div class="filter-section">
            <input type="text" id="searchInput" placeholder="🔍 Search examples...">
            <select id="filterType">
                <option value="">All Evaluation Types</option>
                <option value="loom">LOOM</option>
                <option value="miri">MIRI</option>
                <option value="output">OUTPUT</option>
                <option value="clippy">CLIPPY</option>
                <option value="comparison">编译通过率</option>
            </select>
            <select id="statusFilter">
                <option value="">All Status</option>
                <option value="perfect">All 5/5 in both</option>
                <option value="diff">Different results</option>
                <option value="failed">Any failures</option>
            </select>
        </div>
        
        <div class="examples-grid" id="examplesContainer">
"""
    
    # Generate example cards
    for example in sorted(data['examples'].keys()):
        example_data = data['examples'][example]
        
        html += f"""            <div class="example-card" data-example="{example}">
                <div class="example-header">{example}</div>
                <div class="example-content">
                    <div class="group-section">
                        <div class="group-title">NON-C2RUST (5 runs)</div>
"""
        
        for eval_type in ['loom', 'miri', 'output', 'clippy', 'compile_pass_rate']:
            stats = example_data['non_c2rust'][eval_type]
            passed = stats['passed']
            total = 5
            
            if passed == total:
                status_class = "pass-rate perfect"
            elif passed == 0:
                status_class = "pass-rate failed"
            else:
                status_class = "pass-rate partial"
            
            failures = stats['failures']
            failure_text = ""
            if failures:
                if isinstance(failures[0], dict):
                    # For compile_pass_rate, failures may have iterations but no error
                    if 'error' in failures[0]:
                        failure_text = failures[0]['error'][:50]
                else:
                    failure_text = str(failures[0])[:50]
            
            # Add iteration info if available
            iter_text = ""
            if eval_type == 'compile_pass_rate' and 'avg_iterations' in stats:
                iter_text = f" | 平均迭代: {stats['avg_iterations']}"
            
            html += f"""                        <div class="eval-row">
                            <div class="eval-type">{eval_type.upper()}</div>
                            <div class="eval-stats">
                                <span class="{status_class}">{passed}/5</span>
                                <span class="failure-reason">{failure_text}{iter_text}</span>
                            </div>
                        </div>
"""
        
        html += """                    </div>
                    <div class="group-section">
                        <div class="group-title">C2RUST (5 runs)</div>
"""
        
        for eval_type in ['loom', 'miri', 'output', 'clippy', 'compile_pass_rate']:
            stats = example_data['c2rust'][eval_type]
            passed = stats['passed']
            total = 5
            
            if passed == total:
                status_class = "pass-rate perfect"
            elif passed == 0:
                status_class = "pass-rate failed"
            else:
                status_class = "pass-rate partial"
            
            failures = stats['failures']
            failure_text = ""
            if failures:
                if isinstance(failures[0], dict):
                    # For compile_pass_rate, failures may have iterations but no error
                    if 'error' in failures[0]:
                        failure_text = failures[0]['error'][:50]
                else:
                    failure_text = str(failures[0])[:50]
            
            # Add iteration info if available
            iter_text = ""
            if eval_type == 'compile_pass_rate' and 'avg_iterations' in stats:
                iter_text = f" | 平均迭代: {stats['avg_iterations']}"
            
            html += f"""                        <div class="eval-row">
                            <div class="eval-type">{eval_type.upper()}</div>
                            <div class="eval-stats">
                                <span class="{status_class}">{passed}/5</span>
                                <span class="failure-reason">{failure_text}{iter_text}</span>
                            </div>
                        </div>
"""
        
        html += """                    </div>
                </div>
            </div>
"""
    
    html += """        </div>
    </div>
    
    <script>
        const examples = document.querySelectorAll('.example-card');
        const searchInput = document.getElementById('searchInput');
        const filterType = document.getElementById('filterType');
        const statusFilter = document.getElementById('statusFilter');
        
        function filterExamples() {
            const searchTerm = searchInput.value.toLowerCase();
            const evalType = filterType.value;
            const status = statusFilter.value;
            
            examples.forEach(card => {
                const exampleName = card.getAttribute('data-example').toLowerCase();
                let show = true;
                
                // Search filter
                if (searchTerm && !exampleName.includes(searchTerm)) {
                    show = false;
                }
                
                // Eval type filter
                if (evalType && show) {
                    const c2rustStats = card.innerText.match(new RegExp(evalType.toUpperCase() + '\\\\s*([0-9])/5'));
                    if (!c2rustStats) show = false;
                }
                
                card.style.display = show ? 'block' : 'none';
            });
        }
        
        searchInput.addEventListener('input', filterExamples);
        filterType.addEventListener('change', filterExamples);
        statusFilter.addEventListener('change', filterExamples);
    </script>
</body>
</html>"""
    
    return html

def main():
    html = generate_html()
    output_file = RESULT_DIR / "summary" / "results_visualization.html"
    with open(output_file, 'w') as f:
        f.write(html)
    print(f"✅ HTML visualization generated: {output_file}")

if __name__ == '__main__':
    main()
