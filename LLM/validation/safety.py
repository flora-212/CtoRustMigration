#!/usr/bin/env python3
"""
Safety and lock safety validation tools.

Provides static analysis for safety patterns and lock usage.
"""

import re
from typing import Dict, List
from .errors import ErrorInfo, ValidationResult


class SafetyAnalyzer:
    """Analyzer for safety patterns in Rust code."""
    
    @staticmethod
    def count_pattern(code: str, pattern: str) -> int:
        """Count occurrences of a regex pattern in code."""
        return len(re.findall(pattern, code))
    
    @staticmethod
    def safety_metrics(code: str) -> Dict[str, int]:
        """Extract safety metrics from code."""
        return {
            "unsafe":       SafetyAnalyzer.count_pattern(code, r'\bunsafe\b'),
            "pthread":      SafetyAnalyzer.count_pattern(code, r'\bpthread_'),
            "raw_ptr":      SafetyAnalyzer.count_pattern(code, r'\*mut\b|\*const\b'),
            "static_mut":   SafetyAnalyzer.count_pattern(code, r'\bstatic\s+mut\b'),
            "libc":         SafetyAnalyzer.count_pattern(code, r'\blibc::'),
            "std_mutex":    SafetyAnalyzer.count_pattern(code, r'\bMutex\b'),
            "std_arc":      SafetyAnalyzer.count_pattern(code, r'\bArc\b'),
            "std_rwlock":   SafetyAnalyzer.count_pattern(code, r'\bRwLock\b'),
            "std_condvar":  SafetyAnalyzer.count_pattern(code, r'\bCondvar\b'),
            "std_thread":   SafetyAnalyzer.count_pattern(code, r'\bthread::spawn\b'),
            "lines":        len(code.splitlines()),
        }
    
    @staticmethod
    def analyze_lock_safety(code: str, label: str = "code") -> Dict:
        """Analyze lock-related safety properties."""
        issues = []
        
        # Check for pthread remaining (bad in rewritten code)
        pthread_locks = re.findall(r'pthread_mutex_(lock|unlock|init|destroy)', code)
        if pthread_locks and label != "original":
            issues.append(f"Found {len(pthread_locks)} pthread_mutex calls (should use std::sync::Mutex)")
        
        # Check for manual lock/unlock without RAII
        if re.search(r'pthread_mutex_lock', code) and not re.search(r'pthread_mutex_unlock', code):
            issues.append("Found lock without unlock (potential deadlock)")
        if re.search(r'pthread_mutex_unlock', code) and not re.search(r'pthread_mutex_lock', code):
            issues.append("Found unlock without corresponding lock")
        
        # Check for static mut (data race risk)
        static_muts = re.findall(r'static\s+mut\s+(\w+)', code)
        if static_muts and label != "original":
            issues.append(f"Found {len(static_muts)} static mut variables (data race risk)")
        
        # Check for unsafe blocks
        unsafe_count = len(re.findall(r'\bunsafe\b', code))
        if unsafe_count > 0 and label != "original":
            issues.append(f"Found {unsafe_count} unsafe blocks")
        
        # Check for proper Arc<Mutex<T>> pattern
        has_arc_mutex = bool(re.search(r'Arc<\s*Mutex', code) or re.search(r'Arc::new\(\s*Mutex', code))
        
        # Check for proper thread joining
        has_thread_spawn = bool(re.search(r'thread::spawn', code))
        has_join = bool(re.search(r'\.join\(\)', code))
        if has_thread_spawn and not has_join:
            issues.append("Found thread::spawn without join (thread not waited)")
        
        return {
            "issues": issues,
            "has_std_mutex": bool(re.search(r'\bMutex\b', code)),
            "has_arc_mutex": has_arc_mutex,
            "has_pthread": bool(re.search(r'\bpthread_', code)),
            "has_thread_spawn": has_thread_spawn,
            "has_join": has_join,
            "unsafe_count": unsafe_count,
        }

    @staticmethod
    def validate_safety(code: str) -> ValidationResult:
        """Validate safety metrics."""
        metrics = SafetyAnalyzer.safety_metrics(code)
        
        errors = []
        if metrics["unsafe"] > 0:
            errors.append(ErrorInfo(
                error_type="safety_issue",
                message=f"{metrics['unsafe']} unsafe block(s) found",
                details=f"Unsafe blocks should be minimized. Found: {metrics['unsafe']}"
            ))
        if metrics["pthread"] > 0:
            errors.append(ErrorInfo(
                error_type="safety_issue",
                message=f"{metrics['pthread']} pthread call(s) found",
                details=f"Rewritten code should use std::sync instead of pthread"
            ))
        if metrics["raw_ptr"] > 0:
            errors.append(ErrorInfo(
                error_type="safety_issue",
                message=f"{metrics['raw_ptr']} raw pointer(s) found",
                details=f"Raw pointers are unsafe and should be minimized"
            ))
        if metrics["static_mut"] > 0:
            errors.append(ErrorInfo(
                error_type="safety_issue",
                message=f"{metrics['static_mut']} static mut variable(s) found",
                details=f"Static mutable variables are data race risks"
            ))
        
        if errors:
            msg = f"Safety concerns: {len(errors)} issue(s)"
            return ValidationResult(False, "safety", msg, {"metrics": metrics}, errors=errors)
        else:
            return ValidationResult(True, "safety", "No major safety concerns", {"metrics": metrics}, errors=[])

    @staticmethod
    def validate_lock_safety(code: str) -> ValidationResult:
        """Validate lock safety."""
        analysis = SafetyAnalyzer.analyze_lock_safety(code, label="rewritten")
        
        errors = []
        for issue in analysis["issues"]:
            errors.append(ErrorInfo(
                error_type="lock_safety_issue",
                message=issue,
                details=f"Lock safety analysis detected: {issue}"
            ))
        
        if errors:
            msg = f"Lock safety issues: {len(errors)} issue(s)"
            return ValidationResult(False, "lock_safety", msg, analysis, errors=errors)
        else:
            msg = "Lock safety OK"
            if analysis["has_std_mutex"]:
                msg += " (uses std::sync::Mutex)"
            if not analysis["has_pthread"]:
                msg += " (no pthread calls)"
            return ValidationResult(True, "lock_safety", msg, analysis, errors=[])
