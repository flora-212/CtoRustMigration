#!/usr/bin/env python3
"""
清理脚本 - 删除生成的 main_rewritten_*.rs 文件

使用方式:
    python3 cleanup.py              # 交互式询问
    python3 cleanup.py --help       # 显示帮助
    python3 cleanup.py --all        # 删除所有 main_rewritten_*
    python3 cleanup.py 0            # 删除 main_rewritten_0.rs
    python3 cleanup.py 0 1 2        # 删除 main_rewritten_0/1/2.rs
    python3 cleanup.py --pattern "*.rs"  # 自定义模式
    python3 cleanup.py --dry-run    # 预览要删除的文件（不实际删除）
"""

import os
import glob
import sys
import argparse


def find_files(examples_dir, pattern):
    """找到匹配模式的文件"""
    search_path = os.path.join(examples_dir, "*/", pattern)
    files = sorted(glob.glob(search_path))
    return files


def count_by_type(files):
    """按类型统计文件"""
    counts = {}
    for f in files:
        basename = os.path.basename(f)
        if basename.startswith("main_rewritten_"):
            # 提取 prompt_idx
            parts = basename.replace("main_rewritten_", "").replace(".rs", "")
            try:
                idx = int(parts)
                counts[idx] = counts.get(idx, 0) + 1
            except:
                counts["unknown"] = counts.get("unknown", 0) + 1
        else:
            counts["other"] = counts.get("other", 0) + 1
    return counts


def format_size(size_bytes):
    """格式化文件大小"""
    for unit in ["B", "KB", "MB"]:
        if size_bytes < 1024:
            return f"{size_bytes:.1f}{unit}"
        size_bytes /= 1024
    return f"{size_bytes:.1f}GB"


def get_total_size(files):
    """计算总大小"""
    return sum(os.path.getsize(f) for f in files if os.path.exists(f))


def preview_files(files, limit=10):
    """预览文件列表"""
    if not files:
        print("  (没有文件匹配)")
        return
    
    for i, f in enumerate(files[:limit], 1):
        example = os.path.basename(os.path.dirname(f))
        filename = os.path.basename(f)
        size = format_size(os.path.getsize(f)) if os.path.exists(f) else "?"
        print(f"  {i:3}. [{example:20}] {filename:30} ({size})")
    
    if len(files) > limit:
        print(f"  ... 和 {len(files) - limit} 个更多文件")


def main():
    parser = argparse.ArgumentParser(
        description="清理生成的 Rust 文件",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
示例:
  python3 cleanup.py --all                    # 删除所有 main_rewritten_*
  python3 cleanup.py 0 1 2                    # 删除 main_rewritten_0/1/2.rs
  python3 cleanup.py 1                        # 删除 main_rewritten_1.rs
  python3 cleanup.py --dry-run                # 预览所有要删除的文件
  python3 cleanup.py 0 --dry-run              # 预览 main_rewritten_0.rs
  python3 cleanup.py --pattern "main.rs"      # 删除 main.rs (原始文件)
  python3 cleanup.py --keep 1                 # 删除所有除了 main_rewritten_1.rs 以外的
        """
    )
    
    parser.add_argument(
        "indices",
        nargs="*",
        type=int,
        help="要删除的 prompt_idx (例: 0 1 2)"
    )
    parser.add_argument(
        "--all",
        action="store_true",
        help="删除所有 main_rewritten_*.rs 文件"
    )
    parser.add_argument(
        "--pattern",
        default="main_rewritten_*.rs",
        help="文件匹配模式 (默认: main_rewritten_*.rs)"
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="只预览，不实际删除"
    )
    parser.add_argument(
        "--keep",
        type=int,
        help="保留指定 prompt_idx，删除其他所有 main_rewritten_*"
    )
    parser.add_argument(
        "--yes",
        action="store_true",
        help="跳过确认提示，直接删除"
    )
    parser.add_argument(
        "--examples-dir",
        default="/home/guoxy/concrat/examples",
        help="examples 目录路径"
    )
    
    args = parser.parse_args()
    
    # 验证目录存在
    if not os.path.exists(args.examples_dir):
        print(f"❌ 错误: 目录不存在: {args.examples_dir}")
        sys.exit(1)
    
    # 确定要删除的文件
    files_to_delete = []
    
    if args.keep is not None:
        # 删除除了 keep 之外的所有 main_rewritten_*
        all_rewritten = find_files(args.examples_dir, "main_rewritten_*.rs")
        for f in all_rewritten:
            basename = os.path.basename(f)
            parts = basename.replace("main_rewritten_", "").replace(".rs", "")
            try:
                idx = int(parts)
                if idx != args.keep:
                    files_to_delete.append(f)
            except:
                files_to_delete.append(f)  # 无效格式的也删除
        
        action_desc = f"删除所有 main_rewritten_* 文件 (除了 main_rewritten_{args.keep}.rs)"
    
    elif args.all:
        # 删除所有 main_rewritten_*
        files_to_delete = find_files(args.examples_dir, args.pattern)
        action_desc = f"删除所有匹配 '{args.pattern}' 的文件"
    
    elif args.indices:
        # 删除指定 indices 的文件
        for idx in args.indices:
            pattern = f"main_rewritten_{idx}.rs"
            found = find_files(args.examples_dir, pattern)
            files_to_delete.extend(found)
        indices_str = ", ".join(str(i) for i in args.indices)
        action_desc = f"删除 main_rewritten_{{{indices_str}}}.rs 文件"
    
    else:
        # 交互式询问
        print("\n" + "=" * 70)
        print("清理生成的 Rust 文件")
        print("=" * 70)
        print("\n选项:")
        print("  1. 删除所有 main_rewritten_*.rs")
        print("  2. 删除指定 prompt_idx 的文件 (例如只删 main_rewritten_0)")
        print("  3. 保留指定 prompt_idx，删除其他所有")
        print("  4. 预览所有 main_rewritten_* 文件")
        print("  0. 退出")
        
        choice = input("\n请选择 [0-4]: ").strip()
        
        if choice == "0":
            print("已退出")
            return
        
        elif choice == "1":
            files_to_delete = find_files(args.examples_dir, "main_rewritten_*.rs")
            action_desc = "删除所有 main_rewritten_*.rs 文件"
        
        elif choice == "2":
            indices_input = input("输入要删除的 prompt_idx (空格分隔, 例: 0 1 2): ").strip()
            try:
                indices = [int(x) for x in indices_input.split()]
                for idx in indices:
                    pattern = f"main_rewritten_{idx}.rs"
                    found = find_files(args.examples_dir, pattern)
                    files_to_delete.extend(found)
                indices_str = ", ".join(str(i) for i in indices)
                action_desc = f"删除 main_rewritten_{{{indices_str}}}.rs 文件"
            except ValueError:
                print("❌ 输入格式错误")
                return
        
        elif choice == "3":
            keep_idx = input("输入要保留的 prompt_idx: ").strip()
            try:
                keep_idx = int(keep_idx)
                all_rewritten = find_files(args.examples_dir, "main_rewritten_*.rs")
                for f in all_rewritten:
                    basename = os.path.basename(f)
                    parts = basename.replace("main_rewritten_", "").replace(".rs", "")
                    try:
                        idx = int(parts)
                        if idx != keep_idx:
                            files_to_delete.append(f)
                    except:
                        files_to_delete.append(f)
                action_desc = f"删除所有 main_rewritten_* 文件 (除了 main_rewritten_{keep_idx}.rs)"
            except ValueError:
                print("❌ 输入格式错误")
                return
        
        elif choice == "4":
            files_to_preview = find_files(args.examples_dir, "main_rewritten_*.rs")
            print("\n所有 main_rewritten_* 文件:")
            preview_files(files_to_preview)
            counts = count_by_type(files_to_preview)
            print(f"\n统计:")
            for idx, count in sorted(counts.items()):
                print(f"  main_rewritten_{idx}.rs: {count} 个文件")
            print(f"\n总共: {len(files_to_preview)} 个文件, 大小: {format_size(get_total_size(files_to_preview))}")
            return
        
        else:
            print("❌ 无效的选择")
            return
    
    # 显示预览
    print("\n" + "=" * 70)
    print(action_desc)
    print("=" * 70)
    
    if not files_to_delete:
        print("✅ 没有找到要删除的文件")
        return
    
    counts = count_by_type(files_to_delete)
    total_size = get_total_size(files_to_delete)
    
    print(f"\n找到 {len(files_to_delete)} 个文件，总大小 {format_size(total_size)}")
    print("\n统计:")
    for idx, count in sorted(counts.items()):
        print(f"  main_rewritten_{idx}.rs: {count} 个文件")
    
    print("\n文件列表 (前 20 个):")
    preview_files(files_to_delete, limit=20)
    
    # 干运行模式
    if args.dry_run:
        print("\n✅ 预览完成 (--dry-run 模式，未实际删除)")
        return
    
    # 确认删除
    if not args.yes:
        confirm = input("\n确认删除? (yes/no): ").strip().lower()
        if confirm not in ("yes", "y"):
            print("❌ 已取消")
            return
    
    # 删除文件
    print("\n删除文件中...")
    deleted_count = 0
    failed_count = 0
    
    for f in files_to_delete:
        try:
            os.remove(f)
            deleted_count += 1
        except Exception as e:
            print(f"⚠️  删除失败: {f} - {e}")
            failed_count += 1
    
    # 结果统计
    print("\n" + "=" * 70)
    print("删除完成")
    print("=" * 70)
    print(f"✅ 成功删除: {deleted_count} 个文件")
    if failed_count > 0:
        print(f"❌ 删除失败: {failed_count} 个文件")
    print(f"📊 释放空间: {format_size(total_size)}")


if __name__ == "__main__":
    main()
