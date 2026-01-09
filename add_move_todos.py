#!/usr/bin/env python3
"""
Script to add TODO comments to functions that have move-related parameters.

In JavaScript, ActiveMove is passed everywhere, but in Rust we use various types:
- move_id: &ID
- move_id: &str
- move_id: ID
- the_move: &Move
etc.

This script finds these functions and adds a TODO comment to verify type consistency.
"""

import os
import re
from pathlib import Path
from typing import List, Tuple

# Move-related parameter patterns to search for
MOVE_PARAM_PATTERNS = [
    r'\bmove_id:\s*&?ID\b',
    r'\bmove_id:\s*&?str\b',
    r'\bmove_id:\s*&?String\b',
    r'\bthe_move:\s*&',
    r'\bactive_move:\s*&',
    r'\bmove:\s*&Move\b',
    r'\bmove:\s*&ID\b',
    r'\bz_move:\s*Option<&str>',
    r'\bmax_move:\s*Option<&str>',
    r'\bmove_name:\s*&',
    r'\bsource_move:\s*&',
    r'\bbase_move:\s*&',
]

# Combined pattern
COMBINED_PATTERN = re.compile('|'.join(MOVE_PARAM_PATTERNS))

# Pattern to find function signatures
FN_PATTERN = re.compile(r'^(\s*)(pub\s+)?(async\s+)?fn\s+(\w+)')

# TODO comment to add
TODO_COMMENT = "// TODO: Verify move parameter type matches JavaScript's ActiveMove usage"

def find_rust_files(base_dir: str) -> List[Path]:
    """Find all .rs files in the directory tree."""
    rust_files = []
    for root, dirs, files in os.walk(base_dir):
        # Skip target directory
        if 'target' in dirs:
            dirs.remove('target')
        for file in files:
            if file.endswith('.rs'):
                rust_files.append(Path(root) / file)
    return rust_files

def find_functions_with_move_params(filepath: Path) -> List[Tuple[int, str, str]]:
    """
    Find functions that have move-related parameters.
    Returns list of (line_number, function_name, matched_param).
    """
    results = []

    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
            lines = content.split('\n')
    except Exception as e:
        print(f"Error reading {filepath}: {e}")
        return results

    i = 0
    while i < len(lines):
        line = lines[i]

        # Check if this line starts a function
        fn_match = FN_PATTERN.match(line)
        if fn_match:
            fn_name = fn_match.group(4)
            # Collect the full function signature (may span multiple lines)
            signature_lines = [line]
            j = i + 1
            paren_count = line.count('(') - line.count(')')

            # Keep reading until we close all parentheses or hit the function body
            while j < len(lines) and paren_count > 0 and '{' not in lines[j-1]:
                signature_lines.append(lines[j])
                paren_count += lines[j].count('(') - lines[j].count(')')
                j += 1

            full_signature = '\n'.join(signature_lines)

            # Check if any move-related parameter is in the signature
            match = COMBINED_PATTERN.search(full_signature)
            if match:
                results.append((i + 1, fn_name, match.group()))

        i += 1

    return results

def add_todo_to_function(filepath: Path, line_num: int) -> bool:
    """
    Add a TODO comment before the function at the given line.
    Returns True if the file was modified.
    """
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except Exception as e:
        print(f"Error reading {filepath}: {e}")
        return False

    # Line numbers are 1-indexed, convert to 0-indexed
    idx = line_num - 1

    if idx < 0 or idx >= len(lines):
        return False

    # Check if TODO is already present in the line above
    if idx > 0 and TODO_COMMENT in lines[idx - 1]:
        return False

    # Get the indentation of the function
    fn_line = lines[idx]
    indent_match = re.match(r'^(\s*)', fn_line)
    indent = indent_match.group(1) if indent_match else ''

    # Insert the TODO comment before the function
    todo_line = f"{indent}{TODO_COMMENT}\n"
    lines.insert(idx, todo_line)

    try:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.writelines(lines)
        return True
    except Exception as e:
        print(f"Error writing {filepath}: {e}")
        return False

def main():
    # Base directory for Rust source
    base_dir = Path(__file__).parent / 'src'

    if not base_dir.exists():
        print(f"Error: {base_dir} does not exist")
        return

    print(f"Scanning {base_dir} for move-related function parameters...")
    print("=" * 60)

    rust_files = find_rust_files(str(base_dir))
    print(f"Found {len(rust_files)} Rust files")

    total_functions = 0
    modified_files = 0
    all_matches = []

    for filepath in sorted(rust_files):
        matches = find_functions_with_move_params(filepath)
        if matches:
            all_matches.append((filepath, matches))
            total_functions += len(matches)

    print(f"Found {total_functions} functions with move-related parameters")
    print("=" * 60)

    # First pass: print what we found
    for filepath, matches in all_matches:
        rel_path = filepath.relative_to(Path(__file__).parent)
        print(f"\n{rel_path}:")
        for line_num, fn_name, param in matches:
            print(f"  Line {line_num}: fn {fn_name}() - matched: {param}")

    print("\n" + "=" * 60)
    print("Adding TODO comments...")
    print("=" * 60)

    # Second pass: add TODOs (in reverse order within each file to preserve line numbers)
    for filepath, matches in all_matches:
        # Sort by line number in reverse to avoid shifting issues
        sorted_matches = sorted(matches, key=lambda x: x[0], reverse=True)
        file_modified = False

        for line_num, fn_name, param in sorted_matches:
            if add_todo_to_function(filepath, line_num):
                rel_path = filepath.relative_to(Path(__file__).parent)
                print(f"Added TODO to {rel_path}:{line_num} ({fn_name})")
                file_modified = True
            else:
                rel_path = filepath.relative_to(Path(__file__).parent)
                print(f"Skipped {rel_path}:{line_num} ({fn_name}) - already has TODO")

        if file_modified:
            modified_files += 1

    print("\n" + "=" * 60)
    print(f"Summary:")
    print(f"  Files scanned: {len(rust_files)}")
    print(f"  Functions found: {total_functions}")
    print(f"  Files modified: {modified_files}")
    print("=" * 60)

if __name__ == '__main__':
    main()
