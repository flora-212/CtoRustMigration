#!/usr/bin/env python3
"""
Utilities for parsing metadata and extracting information from file paths and metadata.

This module provides functions for:
- Extracting round numbers from metadata files
- Parsing directory names to extract round information
- Pattern matching on file paths and metadata structures
"""

import json
import os
import re
from typing import Optional


def get_round_from_metadata(sample_name: str, examples_dir: str) -> str:
    """
    Extract round number from rounds_metadata.json for a sample.
    
    Returns the last successful round number, or "c2rust" if no round compiled successfully.
    
    Args:
        sample_name: Name of the sample example
        examples_dir: Path to the examples directory
    
    Returns:
        The last successful round number (e.g., "1", "2") or "c2rust"
    """
    metadata_path = os.path.join(examples_dir, sample_name, "rounds_metadata.json")
    if not os.path.exists(metadata_path):
        return "c2rust"
    
    try:
        with open(metadata_path) as f:
            data = json.load(f)
            # Find last successful round
            last_successful_round = None
            for round_key in sorted(data.keys(), key=lambda x: int(x)):
                if data[round_key].get("compile_status"):
                    last_successful_round = round_key
            
            # Return last successful round if found, otherwise "c2rust"
            return last_successful_round if last_successful_round else "c2rust"
    except (json.JSONDecodeError, ValueError, KeyError):
        pass
    
    return "c2rust"


def extract_round_from_dirname(dir_path: str) -> Optional[str]:
    """
    Extract round number from directory name.
    
    Expects format: YYYYmmdd_hhmmss_N[_optional_suffix]
    where N is the round number.
    
    Args:
        dir_path: Path to the directory
    
    Returns:
        The round number as a string, or None if pattern doesn't match
    
    Examples:
        "20260417_143022_3" -> "3"
        "20260417_143022_3_results" -> "3"
        "20260417_143022" -> None
    """
    # Normalize path to get directory name
    check_dir = dir_path.rstrip('/')
    if check_dir.endswith('/evaluation') or check_dir.endswith('evaluation'):
        # If in evaluation subdir, use parent
        check_dir = os.path.dirname(check_dir)
    
    dir_name = os.path.basename(check_dir)
    
    # Match format: YYYYmmdd_hhmmss_N[_optional_suffix]
    match = re.search(r'^\d{8}_\d{6}_(\d+)(?:_|$)', dir_name)
    if match:
        return match.group(1)
    
    return None


def find_examples_dir(base_output_dir: str) -> Optional[str]:
    """
    Find the examples directory within a base output directory.
    
    Handles both timestamped output directories and legacy paths.
    
    Args:
        base_output_dir: Path to search for examples directory
    
    Returns:
        Path to examples directory, or None if not found
    """
    # Check if examples is directly in this directory
    if os.path.isdir(os.path.join(base_output_dir, 'examples')):
        return os.path.join(base_output_dir, 'examples')
    
    # Check if evaluation/ subdir exists, and examples is in parent
    if base_output_dir.endswith('/evaluation') or base_output_dir.endswith('evaluation'):
        parent_dir = os.path.dirname(base_output_dir.rstrip('/'))
        if os.path.isdir(os.path.join(parent_dir, 'examples')):
            return os.path.join(parent_dir, 'examples')
    
    return None


def find_input_file(filename: str, search_dirs: list) -> tuple:
    """
    Find a file in multiple search directories.
    
    Args:
        filename: Name of the file to find
        search_dirs: List of directories to search in order
    
    Returns:
        Tuple of (file_path, parent_directory) if found, else (None, None)
    """
    for candidate_dir in search_dirs:
        candidate_path = os.path.join(candidate_dir, filename)
        if os.path.exists(candidate_path):
            return candidate_path, candidate_dir
    
    return None, None
