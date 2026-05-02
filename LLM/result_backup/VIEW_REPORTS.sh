#!/bin/bash

echo "📊 Evaluation Results Summary - Report Viewer"
echo "=============================================="
echo ""
echo "Generated Reports:"
echo ""
echo "1. 📋 Text Report (Human-Readable)"
echo "   File: comprehensive_summary.txt (135K)"
echo "   View: less comprehensive_summary.txt"
echo ""
echo "2. 📊 JSON Report (Machine-Readable)"
echo "   File: comprehensive_summary.json (344K)"
echo "   Use:  python3 -m json.tool comprehensive_summary.json | less"
echo ""
echo "3. 🌐 Interactive Dashboard"
echo "   File: results_visualization.html (281K)"
echo "   View: firefox results_visualization.html"
echo "   Or:   open results_visualization.html"
echo ""
echo "4. 📄 Documentation"
echo "   - README.md: Usage guide and statistics"
echo "   - FINDINGS.md: Executive summary and recommendations"
echo ""
echo "Quick Commands:"
echo "==============="
echo "# View summary statistics"
echo "python3 << 'END'
import json
d = json.load(open('comprehensive_summary.json'))
print(f\"Total Examples: {d['total_examples']}\")
for group in ['non_c2rust', 'c2rust']:
    summary = d['groups'][group]['summary']
    miri_rate = summary['miri']['passed'] / (summary['miri']['passed'] + summary['miri']['failed']) * 100
    print(f\"{group.upper()}: MIRI {miri_rate:.1f}% pass rate\")
END"
echo ""
echo "# Search for specific example"
echo "grep -A 30 'Example: array_const' comprehensive_summary.txt"
echo ""
echo "# Count failures by type"
echo "grep -c 'error\\[' comprehensive_summary.txt"
echo ""
