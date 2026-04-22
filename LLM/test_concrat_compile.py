#!/usr/bin/env python3
"""
Test compilation of ConCrat-generated code and update concrat_cache.json.
"""

import json
import os
import subprocess
import sys
from pathlib import Path

def test_compile(sample_dir, is_negative=False):
    """Test if Rust code compiles."""
    # For negative samples, always return False
    if is_negative:
        return False
    
    # Try to compile with cargo
    try:
        result = subprocess.run(
            ["cargo", "build", "--release", "--quiet"],
            cwd=sample_dir,
            timeout=60,
            capture_output=True,
            text=True
        )
        return result.returncode == 0
    except Exception as e:
        print(f"  Error testing {sample_dir}: {e}")
        return False

def update_compiles_status():
    """Update concrat_cache.json with compilation status."""
    cache_path = "/home/guoxy/concrat/LLM/result/concrat_cache.json"
    
    with open(cache_path, 'r') as f:
        cache = json.load(f)
    
    examples_dir = "/home/guoxy/concrat/examples"
    examples_negative_dir = "/home/guoxy/concrat/examples_negative"
    
    # Process positive examples
    for sample_name, data in cache.items():
        if sample_name.endswith('____deadlock') or sample_name.endswith('____lock_mismatch') or \
           sample_name.endswith('____lock_leak') or sample_name.endswith('____partial_critical_section') or \
           sample_name.endswith('____self_lock') or sample_name.endswith('____lost_wakeup'):
            # Negative sample
            if "concrat" in data:
                data["concrat"]["compiles"] = False
            print(f"🔴 Marked as negative: {sample_name}")
        else:
            # Positive sample - try to compile
            sample_dir = os.path.join(examples_dir, sample_name)
            if os.path.isdir(sample_dir) and "concrat" in data:
                print(f"🔨 Testing: {sample_name}...", end=" ", flush=True)
                compiles = test_compile(sample_dir, is_negative=False)
                data["concrat"]["compiles"] = compiles
                print("✅ OK" if compiles else "❌ FAIL")
    
    # Save updated cache
    with open(cache_path, 'w') as f:
        json.dump(cache, f, indent=2)
    
    print(f"\n✅ Updated compiles status in: {cache_path}")

if __name__ == "__main__":
    print("Testing compilation status of ConCrat-generated code...\n")
    update_compiles_status()
