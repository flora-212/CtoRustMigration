"""
Output management module for organizing results with timestamp-based directories.

Structure:
    result/
        {timestamp}_{prompt_idx}/
            config.json                    # Configuration file
            examples/
                example_001/
                    round1.rs              # First iteration result
                    round2.rs              # Second iteration result (if needed)
                    ...
                    final.rs               # Final version
                example_002/
                    round1.rs
                    ...
                    final.rs
                ...
            evaluation/
                comparison_report.json
                comparison_report.md
                clippy_concurrency_report.json
                clippy_concurrency_report.md
"""

import os
import json
import time
from datetime import datetime
from pathlib import Path
from typing import Dict, Any, Optional, List


class OutputManager:
    """Manages timestamped output directory structure"""
    
    def __init__(self, result_base_dir: str = "/home/guoxy/concrat/LLM/result"):
        self.result_base_dir = result_base_dir
        self.timestamp = None
        self.prompt_idx = None
        self.output_root = None
        self.config = {}
        
    def initialize(self, prompt_idx: int, validate: bool = False, 
                   strategy: str = "compile", max_iterations: int = 3,
                   force: bool = False, model: str = "qwen2.5-coder:14b"):
        """Initialize output directory with timestamp"""
        self.prompt_idx = prompt_idx
        self.timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        
        # Include validation strategy in directory name if validating
        dir_suffix = f"_{prompt_idx}"
        if validate:
            dir_suffix = f"_{prompt_idx}_{strategy}"
        else:
            dir_suffix = f"_{prompt_idx}_no_validation"
            
        self.output_root = os.path.join(
            self.result_base_dir,
            f"{self.timestamp}{dir_suffix}"
        )
        
        # Create main directory structure
        os.makedirs(self.output_root, exist_ok=True)
        os.makedirs(os.path.join(self.output_root, "examples"), exist_ok=True)
        os.makedirs(os.path.join(self.output_root, "evaluation"), exist_ok=True)
        
        # Initialize configuration
        self.config = {
            "timestamp": self.timestamp,
            "prompt_idx": prompt_idx,
            "validate": validate,
            "strategy": strategy,
            "max_iterations": max_iterations,
            "force": force,
            "model": model,
            "start_time": datetime.now().isoformat(),
            "end_time": None,
            "total_files": 0,
            "success_count": 0,
            "failed_count": 0,
            "failed_examples": []
        }
        
        self.save_config()
        return self.output_root
    
    def get_example_dir(self, example_name: str) -> str:
        """Get output directory for a specific example"""
        example_dir = os.path.join(self.output_root, "examples", example_name)
        os.makedirs(example_dir, exist_ok=True)
        return example_dir
    
    def get_evaluation_dir(self) -> str:
        """Get evaluation results directory"""
        return os.path.join(self.output_root, "evaluation")
    
    def save_example_round(self, example_name: str, round_num, 
                           content: str, filename: str = "round") -> str:
        """
        Save a round of results for a specific example
        
        Args:
            example_name: Name of the example
            round_num: Iteration number (1, 2, ...) or "final"
            content: File content
            filename: Base filename prefix (default "round")
            
        Returns:
            Path of saved file
        """
        example_dir = self.get_example_dir(example_name)
        
        if round_num == "final":
            filename = f"final.rs"
        else:
            filename = f"round{round_num}.rs"
        
        filepath = os.path.join(example_dir, filename)
        with open(filepath, "w") as f:
            f.write(content)
        
        return filepath
    
    def save_conversation_history(self, example_name: str, messages: List[Dict[str, Any]]) -> str:
        """
        Save conversation history with LLM for a specific example
        
        Args:
            example_name: Name of the example
            messages: List of message dicts with role and content
            
        Returns:
            Path of saved file
        """
        example_dir = self.get_example_dir(example_name)
        
        conversation_file = os.path.join(example_dir, "conversation_history.json")
        with open(conversation_file, "w") as f:
            json.dump(messages, f, indent=2, ensure_ascii=False)
        
        return conversation_file
    
    def save_round_metadata(self, example_name: str, round_num: int, 
                           passed: bool, compile_status: bool = None,
                           error_data: Dict[str, Any] = None) -> str:
        """
        Save metadata for a specific round (compilation status, errors, etc.)
        Accumulates all rounds in a single rounds_metadata.json file
        
        Args:
            example_name: Name of the example
            round_num: Iteration number
            passed: Whether validation passed
            compile_status: Whether code compiled (True/False/None)
            error_data: Dictionary with error information
            
        Returns:
            Path of metadata file
        """
        example_dir = self.get_example_dir(example_name)
        
        # Load existing metadata or start fresh
        metadata_file = os.path.join(example_dir, "rounds_metadata.json")
        if os.path.exists(metadata_file):
            with open(metadata_file, "r") as f:
                try:
                    all_rounds = json.load(f)
                except:
                    all_rounds = {}
        else:
            all_rounds = {}
        
        # Add/update this round's metadata
        round_meta = {
            "round": round_num,
            "passed": passed,
            "compile_status": compile_status,
            "timestamp": datetime.now().isoformat()
        }
        
        # Add error data if provided
        if error_data:
            round_meta["errors"] = error_data
        
        all_rounds[str(round_num)] = round_meta
        
        # Save consolidated metadata
        with open(metadata_file, "w") as f:
            json.dump(all_rounds, f, indent=2, ensure_ascii=False)
        
        return metadata_file
    
    def save_config(self):
        """Save configuration file"""
        config_path = os.path.join(self.output_root, "config.json")
        with open(config_path, "w") as f:
            json.dump(self.config, f, indent=2, ensure_ascii=False)
    
    def finalize(self, success_count: int, total_count: int, 
                failed_examples: List[str]):
        """Finalize output and update configuration file"""
        self.config["end_time"] = datetime.now().isoformat()
        self.config["total_files"] = total_count
        self.config["success_count"] = success_count
        self.config["failed_count"] = total_count - success_count
        self.config["failed_examples"] = failed_examples
        
        self.save_config()
    
    def finalize_with_round_info(self, examples_round_info: Dict[str, Any]):
        """
        Finalize output with round information for each example
        
        Args:
            examples_round_info: Dict mapping example names to their round info
                {
                    "example_name": {
                        "final_round": 3,
                        "best_compile_round": 2,
                        "final_passed": False,
                        "best_compile_passed": True
                    },
                    ...
                }
        """
        # Save round tracking information
        round_tracking_file = os.path.join(self.output_root, "round_tracking.json")
        with open(round_tracking_file, "w") as f:
            json.dump(examples_round_info, f, indent=2, ensure_ascii=False, default=str)
    
    def copy_evaluation_results(self, src_dir: str):
        """
        Copy evaluation results to output directory
        
        Args:
            src_dir: Source directory for evaluation results (typically result/{prompt_idx}/)
        """
        eval_dest = self.get_evaluation_dir()
        
        if os.path.exists(src_dir):
            for filename in os.listdir(src_dir):
                if filename.endswith(".json") or filename.endswith(".md"):
                    src_file = os.path.join(src_dir, filename)
                    dest_file = os.path.join(eval_dest, filename)
                    if os.path.isfile(src_file):
                        with open(src_file, "r") as f:
                            content = f.read()
                        with open(dest_file, "w") as f:
                            f.write(content)
    
    def get_summary(self) -> Dict[str, Any]:
        """Get output summary"""
        return {
            "output_dir": self.output_root,
            "timestamp": self.timestamp,
            "prompt_idx": self.prompt_idx,
            "config": self.config
        }
    
    def save_error_report(self, example_name: str, error_data: Dict[str, Any]) -> str:
        """
        Save detailed error report for specific example
        
        Args:
            example_name: Name of the example
            error_data: Dictionary containing all error details
                {
                    "total_iterations": int,
                    "failed_at_iteration": int,
                    "iterations": [
                        {
                            "iteration": int,
                            "passed": bool,
                            "errors": [...],
                            "details": {...}
                        },
                        ...
                    ]
                }
        
        Returns:
            Path of saved file
        """
        example_dir = self.get_example_dir(example_name)
        
        # Save error report in JSON format
        error_json_path = os.path.join(example_dir, "error_report.json")
        with open(error_json_path, "w") as f:
            json.dump(error_data, f, indent=2, ensure_ascii=False)
        
        # Generate readable Markdown report
        error_md_path = os.path.join(example_dir, "error_report.md")
        md_content = self._generate_error_report_md(example_name, error_data)
        with open(error_md_path, "w") as f:
            f.write(md_content)
        
        return error_json_path
    
    def _generate_error_report_md(self, example_name: str, error_data: Dict[str, Any]) -> str:
        """
        Generate readable Markdown error report
        """
        md = f"# Compilation Error Report - {example_name}\n\n"
        md += f"**Generated at**: {datetime.now().isoformat()}\n\n"
        md += f"**Total iterations**: {error_data.get('total_iterations', 0)}\n\n"
        
        iterations = error_data.get('iterations', [])
        md += f"## Iteration Details ({len(iterations)} failures total)\n\n"
        
        for iter_data in iterations:
            iteration_num = iter_data.get('iteration', '?')
            md += f"### Iteration {iteration_num}\n\n"
            md += f"**Status**: ❌ Failed\n\n"
            
            errors = iter_data.get('errors', [])
            if errors:
                md += "**Error List**:\n\n"
                for idx, error in enumerate(errors, 1):
                    md += f"{idx}. **{error.get('category', 'Unknown')}**\n"
                    md += f"   - {error.get('message', 'No message')}\n"
                    if error.get('details'):
                        md += f"   - Details: {error['details'][:200]}...\n"
                    md += "\n"
            
            details = iter_data.get('details', {})
            if details:
                md += "**Detailed Error Output**:\n\n"
                md += "```\n"
                for key, value in details.items():
                    if isinstance(value, str) and value.strip():
                        md += f"{key}:\n{value}\n\n"
                md += "```\n\n"
        
        md += "## Summary\n\n"
        md += f"Compilation failed after all {error_data.get('total_iterations', 0)} iterations.\n"
        md += f"The final code used has been saved as `final.rs`.\n"
        
        return md


def get_output_manager() -> OutputManager:
    """Get global OutputManager instance"""
    return OutputManager()
