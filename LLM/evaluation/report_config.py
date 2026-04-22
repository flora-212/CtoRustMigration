#!/usr/bin/env python3
"""
Report configuration and keyword definitions.

This module defines all the keywords, metrics, report types, and patterns used
in comparison report generation.
"""

# ════════════════════════════════════════════════════════════════════════════
# Report Types
# ════════════════════════════════════════════════════════════════════════════

REPORT_TYPES = ["all", "positive_only", "negative_only"]
REPORT_TYPE_ALL = "all"
REPORT_TYPE_POSITIVE_ONLY = "positive_only"
REPORT_TYPE_NEGATIVE_ONLY = "negative_only"


# ════════════════════════════════════════════════════════════════════════════
# Metric Keywords
# ════════════════════════════════════════════════════════════════════════════

# All available metrics
ALL_METRICS = [
    "unsafe",
    "pthread",
    "raw_ptr",
    "static_mut",
    "libc",
    "std_mutex",
    "std_arc",
    "std_rwlock",
    "std_condvar",
    "std_thread",
    "move_closure",
    "arc_clone",
    "join_handle",
    "arc_mutex_combo",
    "lines"
]

# Metrics where LOWER is better (unsafe code patterns to minimize)
METRICS_LOWER_IS_BETTER = [
    "unsafe",
    "pthread",
    "raw_ptr",
    "static_mut",
    "libc"
]

# Metrics where HIGHER is better (idiomatic Rust patterns to maximize)
METRICS_HIGHER_IS_BETTER = [
    "std_mutex",
    "std_arc",
    "std_rwlock",
    "std_condvar",
    "std_thread",
    "move_closure",
    "arc_clone",
    "join_handle",
    "arc_mutex_combo"
]

# Metrics for negative samples (subset with simpler analysis)
NEGATIVE_SAMPLE_METRICS = [
    "unsafe",
    "pthread",
    "raw_ptr",
    "static_mut",
    "libc",
    "lines"
]

# Metrics for safety analysis in tables
SAFETY_METRICS = [
    "unsafe",
    "pthread",
    "raw_ptr",
    "static_mut",
    "libc"
]


# ════════════════════════════════════════════════════════════════════════════
# Sample Classification
# ════════════════════════════════════════════════════════════════════════════

# Field name for negative sample classification
IS_NEGATIVE_FIELD = "is_negative"

# Separator for negative sample naming convention
# Format: {positive_sample_name}____{issue_type}
# Example: array_simple____deadlock, global_simple____lock_mismatch
NEGATIVE_SAMPLE_SEPARATOR = "____"


# ════════════════════════════════════════════════════════════════════════════
# Data Source Keys
# ════════════════════════════════════════════════════════════════════════════

# Keys for different implementation sources in comparison data
SOURCE_ORIGINAL = "original"
SOURCE_CONCRAT = "concrat"
SOURCE_LLM = "llm"

ALL_SOURCES = [SOURCE_ORIGINAL, SOURCE_CONCRAT, SOURCE_LLM]

# Metric and metadata sub-keys
KEY_METRICS = "metrics"
KEY_LOCK_SAFETY = "lock_safety"
KEY_COMPILES = "compiles"
KEY_NAME = "name"
KEY_ISSUES = "issues"


# ════════════════════════════════════════════════════════════════════════════
# Lock Safety Features
# ════════════════════════════════════════════════════════════════════════════

# Lock safety feature detection keywords
LOCK_SAFETY_FEATURES = {
    "has_std_mutex": "std::sync::Mutex",
    "has_arc_mutex": "Arc<Mutex>",
    "has_thread_spawn": "std::thread::spawn",
    "has_join": "thread.join()",
}


# ════════════════════════════════════════════════════════════════════════════
# Metric Categories and Descriptions
# ════════════════════════════════════════════════════════════════════════════

# New concurrency pattern metrics (count-based)
# - move_closure: Usage of 'move' keyword in closures for thread safety
# - arc_clone: Count of Arc clones for shared ownership
# - join_handle: Count of join handles for thread synchronization
# - arc_mutex_combo: Count of Arc<Mutex<T>> combinations (complete thread-safe pattern)


# ════════════════════════════════════════════════════════════════════════════
# Helper Functions
# ════════════════════════════════════════════════════════════════════════════

def get_metric_category(metric: str) -> str:
    """
    Determine whether lower or higher is better for a metric.
    
    Args:
        metric: The metric name
    
    Returns:
        "lower_better", "higher_better", or None if unknown
    """
    if metric in METRICS_LOWER_IS_BETTER:
        return "lower_better"
    elif metric in METRICS_HIGHER_IS_BETTER:
        return "higher_better"
    return None


def is_lower_better(metric: str) -> bool:
    """Check if lower values are better for this metric."""
    return metric in METRICS_LOWER_IS_BETTER


def is_higher_better(metric: str) -> bool:
    """Check if higher values are better for this metric."""
    return metric in METRICS_HIGHER_IS_BETTER


def extract_positive_sample_name(negative_sample_name: str) -> str:
    """
    Extract the positive sample name from a negative sample name.
    
    Negative samples use format: {positive_name}____{issue_type}
    
    Args:
        negative_sample_name: Name like "array_simple____deadlock"
    
    Returns:
        The base positive sample name: "array_simple"
    """
    if NEGATIVE_SAMPLE_SEPARATOR in negative_sample_name:
        return negative_sample_name.split(NEGATIVE_SAMPLE_SEPARATOR)[0]
    return negative_sample_name


def is_negative_sample(item: dict) -> bool:
    """
    Check if a sample item represents a negative test case.
    
    Args:
        item: A sample item from the comparison data
    
    Returns:
        True if this is a negative sample, False otherwise
    """
    return item.get(IS_NEGATIVE_FIELD, False)


def get_metric_display_name(metric: str) -> str:
    """
    Get the display name for a metric (with underscores escaped for markdown).
    
    Args:
        metric: The metric name
    
    Returns:
        The metric name with underscores escaped for markdown tables
    """
    return metric.replace("_", "\\_")
