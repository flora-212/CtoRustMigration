#!/usr/bin/env python3
"""
Generate concrat_cache.json by analyzing main.concrat.rs files in examples/
and main.c2rust.rs files in examples_negative/.

This script extracts metrics and lock_safety information for ConCrat-transformed code.
"""

import json
import os
import re
from pathlib import Path
from collections import defaultdict

def count_lines(content):
    """Count non-empty lines in code."""
    return len([line for line in content.split('\n') if line.strip()])

def extract_metrics(content):
    """Extract metrics from Rust code content."""
    metrics = {
        "unsafe": 0,
        "pthread": 0,
        "raw_ptr": 0,
        "static_mut": 0,
        "libc": 0,
        "std_mutex": 0,
        "std_arc": 0,
        "std_rwlock": 0,
        "std_condvar": 0,
        "std_thread": 0,
        "move_closure": 0,
        "arc_clone": 0,
        "join_handle": 0,
        "arc_mutex_combo": 0,
        "lines": count_lines(content)
    }
    
    # Count unsafe blocks/expressions
    unsafe_matches = re.findall(r'\bunsafe\b', content)
    metrics["unsafe"] = len(unsafe_matches)
    
    # Count pthread calls
    pthread_matches = re.findall(r'\bpthread_\w+', content)
    metrics["pthread"] = len(pthread_matches)
    
    # Count raw pointers (simplified: look for *const, *mut, as *const, as *mut)
    raw_ptr_matches = re.findall(r'(\*const|\*mut|\s+as\s+\*)', content)
    metrics["raw_ptr"] = len(raw_ptr_matches)
    
    # Count static mut declarations
    static_mut_matches = re.findall(r'\bstatic\s+mut\b', content)
    metrics["static_mut"] = len(static_mut_matches)
    
    # Count libc usage (libc::)
    libc_matches = re.findall(r'\blibc::', content)
    metrics["libc"] = len(libc_matches)
    
    # Count std::sync::Mutex
    std_mutex_matches = re.findall(r'Mutex\s*<', content)
    metrics["std_mutex"] = len(std_mutex_matches)
    
    # Count Arc usage (Arc<)
    arc_matches = re.findall(r'\bArc\s*<', content)
    metrics["std_arc"] = len(arc_matches)
    
    # Count RwLock
    rwlock_matches = re.findall(r'\bRwLock\s*<', content)
    metrics["std_rwlock"] = len(rwlock_matches)
    
    # Count Condvar
    condvar_matches = re.findall(r'\bCondvar\b', content)
    metrics["std_condvar"] = len(condvar_matches)
    
    # Count std::thread usage
    thread_matches = re.findall(r'\bthread::|std::thread|spawn', content)
    metrics["std_thread"] = len(thread_matches)
    
    # Count move closures
    move_closure_matches = re.findall(r'\bmove\s*\|', content)
    metrics["move_closure"] = len(move_closure_matches)
    
    # Count .clone() calls on Arc
    arc_clone_matches = re.findall(r'\barc_\w*\.clone\(\)', content)
    metrics["arc_clone"] = len(arc_clone_matches)
    
    # Count JoinHandle
    join_handle_matches = re.findall(r'\bJoinHandle\b', content)
    metrics["join_handle"] = len(join_handle_matches)
    
    # Count Arc<Mutex> combos
    arc_mutex_combo_matches = re.findall(r'\bArc\s*<[^>]*Mutex', content)
    metrics["arc_mutex_combo"] = len(arc_mutex_combo_matches)
    
    return metrics

def extract_lock_safety(content):
    """Extract lock safety information from code."""
    safety = {
        "issues": [],
        "has_std_mutex": False,
        "has_arc_mutex": False,
        "has_pthread": False,
        "has_thread_spawn": False,
        "has_join": False,
        "unsafe_count": len(re.findall(r'\bunsafe\b', content))
    }
    
    # Check for std::sync::Mutex
    if 'Mutex' in content and 'sync' in content:
        safety["has_std_mutex"] = True
    
    # Check for Arc<Mutex
    if re.search(r'\bArc\s*<[^>]*Mutex', content):
        safety["has_arc_mutex"] = True
    
    # Check for pthread
    if re.search(r'\bpthread_', content):
        safety["has_pthread"] = True
    
    # Check for thread spawn
    if re.search(r'\bspawn\b|thread::spawn|std::thread::spawn', content):
        safety["has_thread_spawn"] = True
    
    # Check for join
    if re.search(r'\.join\(\)', content):
        safety["has_join"] = True
    
    return safety

def process_sample(sample_path, rs_file, fallback_file=None):
    """Process a single sample and extract metrics."""
    file_path = os.path.join(sample_path, rs_file)
    
    # Try primary file first
    if not os.path.exists(file_path) and fallback_file:
        # Try fallback file
        file_path = os.path.join(sample_path, fallback_file)
    
    if not os.path.exists(file_path):
        return None
    
    with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
        content = f.read()
    
    metrics = extract_metrics(content)
    lock_safety = extract_lock_safety(content)
    
    return {
        "metrics": metrics,
        "lock_safety": lock_safety,
        "compiles": None  # To be determined by compilation
    }

def generate_cache():
    """Generate concrat_cache.json from main.concrat.rs files."""
    cache = {}
    
    examples_dir = "/home/guoxy/concrat/examples"
    examples_negative_dir = "/home/guoxy/concrat/examples_negative"
    
    # Process positive examples
    if os.path.isdir(examples_dir):
        for sample_dir in sorted(os.listdir(examples_dir)):
            sample_path = os.path.join(examples_dir, sample_dir)
            if not os.path.isdir(sample_path):
                continue
            
            # Initialize cache entry if not exists
            if sample_dir not in cache:
                cache[sample_dir] = {}
            
            # Try to load concrat metrics (main.c2rust.rs is the ConCrat result, fallback to main.concrat.rs)
            concrat_data = process_sample(sample_path, "main.c2rust.rs", "main.concrat.rs")
            if concrat_data:
                cache[sample_dir]["concrat"] = concrat_data
                print(f"✅ Processed: {sample_dir} (concrat)")
            else:
                print(f"⚠️  No main.c2rust.rs or main.concrat.rs found: {sample_dir}")
    
    # Process negative examples
    if os.path.isdir(examples_negative_dir):
        for sample_dir in sorted(os.listdir(examples_negative_dir)):
            sample_path = os.path.join(examples_negative_dir, sample_dir)
            if not os.path.isdir(sample_path):
                continue
            
            # Initialize cache entry if not exists
            if sample_dir not in cache:
                cache[sample_dir] = {
                    "is_negative": True
                }
            else:
                cache[sample_dir]["is_negative"] = True
            
            # For negative examples, main.c2rust.rs is the "concrat" version
            concrat_data = process_sample(sample_path, "main.c2rust.rs")
            if concrat_data:
                cache[sample_dir]["concrat"] = concrat_data
                print(f"✅ Processed: {sample_dir} (negative)")
            else:
                print(f"⚠️  No main.c2rust.rs found: {sample_dir}")
    
    return cache

def merge_with_existing(new_cache, existing_path):
    """Merge new concrat data with existing cache."""
    try:
        with open(existing_path, 'r') as f:
            existing = json.load(f)
    except:
        existing = {}
    
    # Merge: keep existing original data, add/update concrat data
    for sample_name, new_data in new_cache.items():
        if sample_name in existing:
            # Update with new concrat data
            existing[sample_name].update(new_data)
        else:
            # New entry
            existing[sample_name] = new_data
    
    return existing

def main():
    print("🔍 Generating concrat_cache.json from main.concrat.rs files...\n")
    
    new_cache = generate_cache()
    
    # Merge with existing cache
    cache_path = "/home/guoxy/concrat/LLM/result/concrat_cache.json"
    merged_cache = merge_with_existing(new_cache, cache_path)
    
    # Save updated cache
    with open(cache_path, 'w') as f:
        json.dump(merged_cache, f, indent=2)
    
    print(f"\n✅ Updated cache saved to: {cache_path}")
    print(f"   Total entries: {len(merged_cache)}")
    
    # Show summary
    concrat_count = sum(1 for v in merged_cache.values() if 'concrat' in v)
    print(f"   With concrat data: {concrat_count}")

if __name__ == "__main__":
    main()
