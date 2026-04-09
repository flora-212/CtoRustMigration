# Refraction - Modularized Rust Code Refactoring Tool

## Overview

The original `refractor.py` has been refactored into a modular package structure for better maintainability and organization.

## Directory Structure

```
LLM/
├── refractor.py                 # Backward compatible entry point
└── refraction/                  # Main package
    ├── __init__.py              # Package initialization and public API
    ├── config.py                # Configuration constants (MODEL, MAX_RETRIES, etc.)
    ├── prompts.py               # System prompts (SYSTEM_PROMPT_0 to SYSTEM_PROMPT_6)
    ├── error_formatter.py        # Error formatting utilities
    ├── code_rewriter.py         # Core code rewriting functions
    ├── utils.py                 # Utility functions (extract_code, verify_syntax)
    └── main.py                  # Main CLI logic and entry point
```

## Module Descriptions

### `config.py`
Centralized configuration module containing:
- `MODEL`: Default LLM model name
- `MAX_RETRIES`: Number of retry attempts
- `RETRY_DELAY`: Delay between retries (seconds)
- `VALIDATION_STRATEGY`: Default validation tools list

### `prompts.py`
Contains all system prompts and prompt management:
- `SYSTEM_PROMPT_0` through `SYSTEM_PROMPT_6`: Different optimization levels
- `FIXING_PROMPT`: Prompt template for fixing failed validations
- `SYSTEM_PROMPTS`: Dictionary of all prompts indexed by level
- `PROMPT_CONFIGS`: Configuration structure for prompt combinations
- `get_prompt_config()`: Function to retrieve prompt configuration

### `error_formatter.py`
Error formatting utilities for different audiences:
- `format_errors_for_llm()`: Format errors for LLM feedback
- `format_errors_for_display()`: Format errors for terminal display
- `extract_errors_for_storage()`: Format errors for JSON storage

### `code_rewriter.py`
Core code rewriting functionality:
- `rewrite_file()`: Simple rewrite without validation
- `rewrite_file_with_validation()`: Iterative rewrite with validation and fallback mechanism

### `utils.py`
Utility functions:
- `extract_code()`: Extract code from markdown code blocks
- `verify_syntax()`: Check Rust syntax with rustfmt

### `main.py`
Main CLI entry point:
- `main()`: Orchestrates the entire refactoring process
- Handles command-line argument parsing
- Manages the refactoring workflow
- Produces output summaries

### `__init__.py`
Package initialization that exports all public APIs:
- Re-exports all major functions and classes
- Provides convenient import interface

## Usage

The tool maintains backward compatibility - you can continue using it exactly as before:

```bash
# Use the original refractor.py command
python3 refractor.py 5 --validate --tools compile --max-iterations 5

# This internally delegates to refraction.main.main()
```

## Advantages of Modularization

1. **Better Organization**: Each module has a single, clear responsibility
2. **Easier Maintenance**: Smaller, focused files are easier to understand and modify
3. **Better Reusability**: Modules can be imported and used independently
4. **Easier Testing**: Individual components can be tested in isolation
5. **Clearer Dependencies**: Module imports clearly show what each component needs
6. **Scalability**: Easy to add new prompts, formatters, or validation strategies

## Import Examples

```python
# Import specific functions
from refraction.code_rewriter import rewrite_file_with_validation
from refraction.error_formatter import format_errors_for_llm
from refraction.prompts import get_prompt_config

# Import from package __init__
from refraction import (
    SYSTEM_PROMPTS,
    rewrite_file,
    verify_syntax
)
```

## Future Enhancements

Possible improvements enabled by this modular structure:
- Add new prompt levels without modifying core logic
- Create alternative error formatters for different output formats
- Develop specialized validation strategies
- Support multiple LLM backends in config.py
- Create advanced CLI with subcommands using main.py as base
- Generate documentation from prompt configurations
