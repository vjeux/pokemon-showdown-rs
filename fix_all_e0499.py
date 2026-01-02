#!/usr/bin/env python3
"""
Comprehensive E0499 fixer for remove_volatile refactoring.
Handles multiple patterns of borrow checker errors.
"""
import re
import sys
import os

def fix_pattern_1(content):
    """
    Pattern 1: let removed = { pokemon_at_mut ... remove_volatile }; if removed {...}
    Replace with: check has_volatile, then remove if true
    """
    pattern = re.compile(
        r'let removed = \{\s*'
        r'let (\w+) = match battle\.pokemon_at_mut\((\w+)\.0, \2\.1\) \{\s*'
        r'Some\(p\) => p,\s*'
        r'None => return EventResult::Continue,\s*'
        r'\};\s*'
        r'Pokemon::remove_volatile\(battle, \(\1\.side_index, \1\.position\), (&[^)]+)\)\s*'
        r'\};\s*'
        r'if removed \{\s*'
        r'((?:.*?\n)*?)'  # capture body
        r'return EventResult::Continue;\s*'
        r'\}',
        re.MULTILINE | re.DOTALL
    )

    def replacer(match):
        var_name = match.group(1)
        pos_var = match.group(2)
        volatile_id = match.group(3)
        body = match.group(4)

        return f"""let has_volatile = {{
        let {var_name} = match battle.pokemon_at({pos_var}.0, {pos_var}.1) {{
            Some(p) => p,
            None => return EventResult::Continue,
        }};
        {var_name}.has_volatile({volatile_id})
    }};

    if has_volatile {{
        Pokemon::remove_volatile(battle, {pos_var}, {volatile_id});
        {body}return EventResult::Continue;
    }}"""

    return pattern.sub(replacer, content)

def fix_pattern_2(content):
    """
    Pattern 2: Simple pokemon_at_mut followed by remove_volatile (outside braces)
    """
    pattern = re.compile(
        r'let (\w+) = match battle\.pokemon_at_mut\((\w+)\.0, \2\.1\) \{\s*'
        r'Some\(p\) => p,\s*'
        r'None => return EventResult::Continue,\s*'
        r'\};\s*'
        r'Pokemon::remove_volatile\(battle, \(\1\.side_index, \1\.position\), (&[^)]+)\);',
        re.MULTILINE | re.DOTALL
    )

    def replacer(match):
        var_name = match.group(1)
        pos_var = match.group(2)
        volatile_id = match.group(3)
        return f'Pokemon::remove_volatile(battle, {pos_var}, {volatile_id});'

    return pattern.sub(replacer, content)

def fix_pattern_3(content):
    """
    Pattern 3: Inside if block with pokemon_at_mut followed by remove_volatile
    """
    pattern = re.compile(
        r'(\s+)let (\w+) = match battle\.pokemon_at_mut\((\w+)\.0, \3\.1\) \{\s*'
        r'Some\(p\) => p,\s*'
        r'None => return EventResult::Continue,\s*'
        r'\};\s*'
        r'Pokemon::remove_volatile\(battle, \(\2\.side_index, \2\.position\), (&[^)]+)\);',
        re.MULTILINE | re.DOTALL
    )

    def replacer(match):
        indent = match.group(1)
        var_name = match.group(2)
        pos_var = match.group(3)
        volatile_id = match.group(4)
        return f'{indent}Pokemon::remove_volatile(battle, {pos_var}, {volatile_id});'

    return pattern.sub(replacer, content)

def process_file(filepath):
    """Process a single file with all patterns"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()

        original = content

        # Apply all patterns
        content = fix_pattern_1(content)
        content = fix_pattern_2(content)
        content = fix_pattern_3(content)

        if content != original:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Fixed: {filepath}")
            return True
        return False
    except Exception as e:
        print(f"Error processing {filepath}: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        return False

def main():
    files = []

    # Collect all Rust files from arguments or default directories
    if len(sys.argv) > 1:
        files = sys.argv[1:]
    else:
        # Default: all callback files
        import glob
        files = (
            glob.glob("src/data/move_callbacks/*.rs") +
            glob.glob("src/data/ability_callbacks/*.rs") +
            glob.glob("src/data/item_callbacks/*.rs")
        )

    fixed_count = 0
    for filepath in files:
        if os.path.isfile(filepath):
            if process_file(filepath):
                fixed_count += 1

    print(f"\nProcessed {len(files)} files, fixed {fixed_count}")

if __name__ == "__main__":
    main()
