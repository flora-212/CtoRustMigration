"""
Configuration and constants for the Rust code refactoring tool.
"""

# Model configuration
MODEL = "qwen2.5-coder:14b"

# Retry configuration
MAX_RETRIES = 3
RETRY_DELAY = 5  # seconds

# Validation configuration
VALIDATION_STRATEGY = ["compile"]  # Default validation tools (now a list)
