#!/usr/bin/env python3
"""
Cleanup script - Delete generated main_rewritten_*.rs files

Usage:
    python3 cleanup.py              # Interactive prompt
    python3 cleanup.py --help       # Show help
    python3 cleanup.py --all        # Delete all main_rewritten_*
    python3 cleanup.py 0            # Delete main_rewritten_0.rs
    python3 cleanup.py 0 1 2        # Delete main_rewritten_0/1/2.rs
    python3 cleanup.py --pattern "*.rs"  # Custom pattern
    python3 cleanup.py --dry-run    # Preview files to delete (don't actually delete)
"""

import os
import glob
import sys
import argparse


def find_files(examples_dir, pattern):
    """Find files matching pattern"""
    search_path = os.path.join(examples_dir, "*/", pattern)
    files = sorted(glob.glob(search_path))
    return files


def count_by_type(files):
    """Count files by type"""
    counts = {}
    for f in files:
        basename = os.path.basename(f)
        if basename.startswith("main_rewritten_"):
            # Extract prompt_idx
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
    """Format file size"""
    for unit in ["B", "KB", "MB"]:
        if size_bytes < 1024:
            return f"{size_bytes:.1f}{unit}"
        size_bytes /= 1024
    return f"{size_bytes:.1f}GB"


def get_total_size(files):
    """Calculate total size"""
    return sum(os.path.getsize(f) for f in files if os.path.exists(f))


def preview_files(files, limit=10):
    """Preview file list"""
    if not files:
        print("  (No files match)")
        return
    
    for i, f in enumerate(files[:limit], 1):
        example = os.path.basename(os.path.dirname(f))
        filename = os.path.basename(f)
        size = format_size(os.path.getsize(f)) if os.path.exists(f) else "?"
        print(f"  {i:3}. [{example:20}] {filename:30} ({size})")
    
    if len(files) > limit:
        print(f"  ... and {len(files) - limit} more files")


def main():
    parser = argparse.ArgumentParser(
        description="Cleanup generated Rust files",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python3 cleanup.py --all                    # Delete all main_rewritten_*
  python3 cleanup.py 0 1 2                    # Delete main_rewritten_0/1/2.rs
  python3 cleanup.py 1                        # Delete main_rewritten_1.rs
  python3 cleanup.py --dry-run                # Preview all files to delete
  python3 cleanup.py 0 --dry-run              # Preview main_rewritten_0.rs
  python3 cleanup.py --pattern "main.rs"      # Delete main.rs (original files)
  python3 cleanup.py --keep 1                 # Delete all except main_rewritten_1.rs
        """
    )
    
    parser.add_argument(
        "indices",
        nargs="*",
        type=int,
        help="prompt_idx to delete (e.g.: 0 1 2)"
    )
    parser.add_argument(
        "--all",
        action="store_true",
        help="Delete all main_rewritten_*.rs files"
    )
    parser.add_argument(
        "--pattern",
        default=None,
        help="File matching pattern (e.g., main.rs, main_c2rust.rs, *.rs)"
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Preview only, do not actually delete"
    )
    parser.add_argument(
        "--keep",
        type=int,
        help="Keep specified prompt_idx, delete all other main_rewritten_*"
    )
    parser.add_argument(
        "--yes",
        action="store_true",
        help="Skip confirmation prompt, delete directly"
    )
    parser.add_argument(
        "--examples-dir",
        default="/home/guoxy/concrat/examples",
        help="Path to examples directory"
    )
    
    args = parser.parse_args()
    
    # Verify directory exists
    if not os.path.exists(args.examples_dir):
        print(f"❌ Error: Directory not found: {args.examples_dir}")
        sys.exit(1)
    
    # Determine files to delete
    files_to_delete = []
    
    # If pattern is specified (and not default), use it directly
    if args.pattern is not None:
        files_to_delete = find_files(args.examples_dir, args.pattern)
        action_desc = f"Delete all files matching '{args.pattern}'"
    
    elif args.keep is not None:
        # Delete all main_rewritten_* except keep
        all_rewritten = find_files(args.examples_dir, "main_rewritten_*.rs")
        for f in all_rewritten:
            basename = os.path.basename(f)
            parts = basename.replace("main_rewritten_", "").replace(".rs", "")
            try:
                idx = int(parts)
                if idx != args.keep:
                    files_to_delete.append(f)
            except:
                files_to_delete.append(f)  # Also delete invalid format
        
        action_desc = f"Delete all main_rewritten_* files (except main_rewritten_{args.keep}.rs)"
    
    elif args.all:
        # Delete all main_rewritten_*
        files_to_delete = find_files(args.examples_dir, "main_rewritten_*.rs")
        action_desc = f"Delete all files matching 'main_rewritten_*.rs'"
    
    elif args.indices:
        # Delete files for specified indices
        for idx in args.indices:
            pattern = f"main_rewritten_{idx}.rs"
            found = find_files(args.examples_dir, pattern)
            files_to_delete.extend(found)
        indices_str = ", ".join(str(i) for i in args.indices)
        action_desc = f"Delete main_rewritten_{{{indices_str}}}.rs files"
    
    else:
        # Interactive prompt
        print("\n" + "=" * 70)
        print("Cleanup generated Rust files")
        print("=" * 70)
        print("\nOptions:")
        print("  1. Delete all main_rewritten_*.rs")
        print("  2. Delete files for specific prompt_idx (e.g., main_rewritten_0 only)")
        print("  3. Keep specific prompt_idx, delete all others")
        print("  4. Preview all main_rewritten_* files")
        print("  0. Exit")
        
        choice = input("\nPlease select [0-4]: ").strip()
        
        if choice == "0":
            print("Exited")
            return
        
        elif choice == "1":
            files_to_delete = find_files(args.examples_dir, "main_rewritten_*.rs")
            action_desc = "Delete all main_rewritten_*.rs files"
        
        elif choice == "2":
            indices_input = input("Input prompt_idx to delete (space-separated, e.g.: 0 1 2): ").strip()
            try:
                indices = [int(x) for x in indices_input.split()]
                for idx in indices:
                    pattern = f"main_rewritten_{idx}.rs"
                    found = find_files(args.examples_dir, pattern)
                    files_to_delete.extend(found)
                indices_str = ", ".join(str(i) for i in indices)
                action_desc = f"Delete main_rewritten_{{{indices_str}}}.rs files"
            except ValueError:
                print("❌ Input format error")
                return
        
        elif choice == "3":
            keep_idx = input("Input prompt_idx to keep: ").strip()
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
                action_desc = f"Delete all main_rewritten_* files (except main_rewritten_{keep_idx}.rs)"
            except ValueError:
                print("❌ Input format error")
                return
        
        elif choice == "4":
            files_to_preview = find_files(args.examples_dir, "main_rewritten_*.rs")
            print("\nAll main_rewritten_* files:")
            preview_files(files_to_preview)
            counts = count_by_type(files_to_preview)
            print(f"\nStatistics:")
            for idx, count in sorted(counts.items()):
                print(f"  main_rewritten_{idx}.rs: {count} files")
            print(f"\nTotal: {len(files_to_preview)} files, Size: {format_size(get_total_size(files_to_preview))}")
            return
        
        else:
            print("❌ Invalid selection")
            return
    
    # Display preview
    print("\n" + "=" * 70)
    print(action_desc)
    print("=" * 70)
    
    if not files_to_delete:
        print("✅ No files found to delete")
        return
    
    counts = count_by_type(files_to_delete)
    total_size = get_total_size(files_to_delete)
    
    print(f"\nFound {len(files_to_delete)} files, Total size {format_size(total_size)}")
    print("\nStatistics:")
    for idx, count in sorted(counts.items()):
        print(f"  main_rewritten_{idx}.rs: {count} files")
    
    print("\nFile list (first 20):")
    preview_files(files_to_delete, limit=20)
    
    # Dry run mode
    if args.dry_run:
        print("\n✅ Preview complete (--dry-run mode, no files deleted)")
        return
    
    # Confirm deletion
    if not args.yes:
        confirm = input("\nConfirm deletion? (yes/no): ").strip().lower()
        if confirm not in ("yes", "y"):
            print("❌ Cancelled")
            return
    
    # Delete files
    print("\nDeleting files...")
    deleted_count = 0
    failed_count = 0
    
    for f in files_to_delete:
        try:
            os.remove(f)
            deleted_count += 1
        except Exception as e:
            print(f"⚠️  Deletion failed: {f} - {e}")
            failed_count += 1
    
    # Result summary
    print("\n" + "=" * 70)
    print("Deletion complete")
    print("=" * 70)
    print(f"✅ Successfully deleted: {deleted_count} files")
    if failed_count > 0:
        print(f"❌ Deletion failed: {failed_count} files")
    print(f"📊 Space freed: {format_size(total_size)}")


if __name__ == "__main__":
    main()
