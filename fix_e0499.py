#!/usr/bin/env python3
import re
import sys
import os

def fix_borrow_error(content):
    """Fix E0499 borrow errors where pokemon_at_mut is followed by Pokemon::remove_volatile"""

    # Universal pattern:
    # let VAR = match battle.pokemon_at_mut(POS_VAR.0, POS_VAR.1) {
    #     Some(p) => p,
    #     None => return EventResult::Continue,
    # };
    # Pokemon::remove_volatile(battle, (VAR.side_index, VAR.position), &...);
    #
    # Should become:
    # Pokemon::remove_volatile(battle, POS_VAR, &...);

    # Pattern that captures the position variable name
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

    content = pattern.sub(replacer, content)

    # Pattern for cases with extra indentation or inside if blocks
    pattern2 = re.compile(
        r'(\s+)let (\w+) = match battle\.pokemon_at_mut\((\w+)\.0, \3\.1\) \{\s*'
        r'Some\(p\) => p,\s*'
        r'None => return EventResult::Continue,\s*'
        r'\};\s*'
        r'Pokemon::remove_volatile\(battle, \(\2\.side_index, \2\.position\), (&[^)]+)\);',
        re.MULTILINE | re.DOTALL
    )

    def replacer2(match):
        indent = match.group(1)
        var_name = match.group(2)
        pos_var = match.group(3)
        volatile_id = match.group(4)
        return f'{indent}Pokemon::remove_volatile(battle, {pos_var}, {volatile_id});'

    content = pattern2.sub(replacer2, content)

    # Pattern for wrapped in braces
    # {
    #     let VAR = match battle.pokemon_at_mut(POS.0, POS.1) { ... };
    #     Pokemon::remove_volatile(battle, (VAR.side_index, VAR.position), &...);
    # }
    pattern3 = re.compile(
        r'\{\s*'
        r'let (\w+) = match battle\.pokemon_at_mut\((\w+)\.0, \2\.1\) \{\s*'
        r'Some\(p\) => p,\s*'
        r'None => return EventResult::Continue,\s*'
        r'\};\s*'
        r'Pokemon::remove_volatile\(battle, \(\1\.side_index, \1\.position\), (&[^)]+)\);\s*'
        r'\}',
        re.MULTILINE | re.DOTALL
    )

    def replacer3(match):
        var_name = match.group(1)
        pos_var = match.group(2)
        volatile_id = match.group(3)
        return f'Pokemon::remove_volatile(battle, {pos_var}, {volatile_id});'

    content = pattern3.sub(replacer3, content)

    return content

def process_file(filepath):
    """Process a single file"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()

        original = content
        content = fix_borrow_error(content)

        if content != original:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Fixed: {filepath}")
            return True
        return False
    except Exception as e:
        print(f"Error processing {filepath}: {e}", file=sys.stderr)
        return False

def main():
    if len(sys.argv) < 2:
        print("Usage: fix_e0499.py <file1> [file2] ...")
        sys.exit(1)

    fixed_count = 0
    for filepath in sys.argv[1:]:
        if os.path.isfile(filepath):
            if process_file(filepath):
                fixed_count += 1

    print(f"\nFixed {fixed_count} files")

if __name__ == "__main__":
    main()
