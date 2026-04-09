"""
Error formatting utilities for displaying validation errors to LLM and humans.
"""


def format_errors_for_llm(results: list) -> str:
    """
    Format validation errors for LLM feedback.
    Creates a detailed, structured error report.
    
    Args:
        results: List of ValidationResult objects
    
    Returns:
        Formatted error string for LLM
    """
    parts = []
    
    for result in results:
        if not result.passed:
            parts.append(f"\n[{result.category.upper()}]")
            parts.append(f"Status: FAILED")
            
            # Add structured errors with location info
            if result.errors:
                parts.append(f"\nDetailed errors ({len(result.errors)}):")
                for i, err in enumerate(result.errors[:5], 1):  # Limit to first 5
                    if err.error_code:
                        parts.append(f"  {i}. {err.error_code}: {err.message}")
                    else:
                        parts.append(f"  {i}. {err.error_type}: {err.message}")
                    
                    if err.location:
                        parts.append(f"     Location: {err.location}")
                    
                    if err.line > 0:
                        parts.append(f"     Line {err.line}, Column {err.column}")
            else:
                parts.append(f"Message: {result.message}")
    
    return "\n".join(parts) if parts else "Unknown validation error"


def format_errors_for_display(results: list) -> str:
    """
    Format validation errors for human-readable terminal display.
    Includes visual formatting with symbols and indentation.
    
    Args:
        results: List of ValidationResult objects
    
    Returns:
        Formatted error string for display
    """
    lines = []
    
    for result in results:
        status = "✅" if result.passed else "❌"
        lines.append(f"      {status} [{result.category}] {result.message}")
        
        if not result.passed and result.errors:
            for err in result.errors[:3]:  # Show first 3 errors
                if err.location:
                    lines.append(f"         ├─ {err.error_type}: {err.message}")
                    lines.append(f"         │  📍 {err.location}")
                    if err.error_code:
                        lines.append(f"         └─ Code: {err.error_code}")
                else:
                    lines.append(f"         ├─ {err.message}")
            
            if len(result.errors) > 3:
                lines.append(f"         └─ ... and {len(result.errors) - 3} more error(s)")
    
    return "\n".join(lines)


def extract_errors_for_storage(results: list) -> dict:
    """
    Extract errors in a format suitable for JSON storage.
    
    Args:
        results: List of ValidationResult objects
    
    Returns:
        Dictionary with structured error data
    """
    error_data = {
        "summary": {
            "total_checks": len(results),
            "failed_checks": sum(1 for r in results if not r.passed),
            "total_errors": sum(len(r.errors) for r in results if not r.passed)
        },
        "results": []
    }
    
    for result in results:
        result_dict = {
            "category": result.category,
            "passed": result.passed,
            "message": result.message,
            "errors": [e.to_dict() for e in result.errors]
        }
        error_data["results"].append(result_dict)
    
    return error_data
