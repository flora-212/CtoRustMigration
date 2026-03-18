"""
Output management module for organizing results with timestamp-based directories.

Structure:
    result/
        {timestamp}_{prompt_idx}/
            config.json                    # 参数记录
            examples/
                example_001/
                    round1.rs              # 第一轮迭代结果
                    round2.rs              # 第二轮迭代结果（如需要）
                    ...
                    final.rs               # 最终版本
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
    """管理带时间戳的输出目录结构"""
    
    def __init__(self, result_base_dir: str = "/home/guoxy/concrat/LLM/result"):
        self.result_base_dir = result_base_dir
        self.timestamp = None
        self.prompt_idx = None
        self.output_root = None
        self.config = {}
        
    def initialize(self, prompt_idx: int, validate: bool = False, 
                   strategy: str = "compile", max_iterations: int = 3,
                   force: bool = False):
        """初始化输出目录，使用时间戳"""
        self.prompt_idx = prompt_idx
        self.timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        self.output_root = os.path.join(
            self.result_base_dir,
            f"{self.timestamp}_{prompt_idx}"
        )
        
        # 创建主目录结构
        os.makedirs(self.output_root, exist_ok=True)
        os.makedirs(os.path.join(self.output_root, "examples"), exist_ok=True)
        os.makedirs(os.path.join(self.output_root, "evaluation"), exist_ok=True)
        
        # 初始化配置
        self.config = {
            "timestamp": self.timestamp,
            "prompt_idx": prompt_idx,
            "validate": validate,
            "strategy": strategy,
            "max_iterations": max_iterations,
            "force": force,
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
        """为某个example获取输出目录"""
        example_dir = os.path.join(self.output_root, "examples", example_name)
        os.makedirs(example_dir, exist_ok=True)
        return example_dir
    
    def get_evaluation_dir(self) -> str:
        """获取evaluation结果目录"""
        return os.path.join(self.output_root, "evaluation")
    
    def save_example_round(self, example_name: str, round_num, 
                           content: str, filename: str = "round") -> str:
        """
        保存某个example的某一轮结果
        
        Args:
            example_name: 示例名称
            round_num: 轮次号（1, 2, ...）或 "final"
            content: 文件内容
            filename: 基础文件名前缀（默认 "round"）
            
        Returns:
            保存的文件路径
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
    
    def save_config(self):
        """保存配置文件"""
        config_path = os.path.join(self.output_root, "config.json")
        with open(config_path, "w") as f:
            json.dump(self.config, f, indent=2, ensure_ascii=False)
    
    def finalize(self, success_count: int, total_count: int, 
                failed_examples: List[str]):
        """完成输出，更新配置文件"""
        self.config["end_time"] = datetime.now().isoformat()
        self.config["total_files"] = total_count
        self.config["success_count"] = success_count
        self.config["failed_count"] = total_count - success_count
        self.config["failed_examples"] = failed_examples
        
        self.save_config()
    
    def copy_evaluation_results(self, src_dir: str):
        """
        将evaluation结果复制到输出目录
        
        Args:
            src_dir: evaluation结果源目录 (通常是 result/{prompt_idx}/)
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
        """获取输出摘要"""
        return {
            "output_dir": self.output_root,
            "timestamp": self.timestamp,
            "prompt_idx": self.prompt_idx,
            "config": self.config
        }


def get_output_manager() -> OutputManager:
    """获取全局OutputManager实例"""
    return OutputManager()
