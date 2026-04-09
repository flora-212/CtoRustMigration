"""
Rust code refactoring assistant - modular refactor package.
"""

from .prompts import SYSTEM_PROMPTS, PROMPT_CONFIGS, get_prompt_config, FIXING_PROMPT
from .config import MODEL, MAX_RETRIES, RETRY_DELAY, VALIDATION_STRATEGY
from .code_rewriter import rewrite_file, rewrite_file_with_validation, extract_code
from .error_formatter import format_errors_for_llm, format_errors_for_display, extract_errors_for_storage
from .utils import verify_syntax

__all__ = [
    'SYSTEM_PROMPTS',
    'PROMPT_CONFIGS',
    'get_prompt_config',
    'FIXING_PROMPT',
    'MODEL',
    'MAX_RETRIES',
    'RETRY_DELAY',
    'VALIDATION_STRATEGY',
    'rewrite_file',
    'rewrite_file_with_validation',
    'extract_code',
    'format_errors_for_llm',
    'format_errors_for_display',
    'extract_errors_for_storage',
    'verify_syntax',
]
