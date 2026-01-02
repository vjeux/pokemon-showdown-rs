#!/usr/bin/env python3
"""
This script fixes E0499 borrow errors by:
1. Finding Pokemon::remove_volatile(battle, (VAR.side_index, VAR.position), &...) calls
2. Tracing back to find the let VAR = match battle.pokemon_at_mut(POS.0, POS.1) pattern
3. Replacing pokemon_at_mut with pokemon_at
4. Replacing the remove_volatile call to use POS instead of (VAR.side_index, VAR.position)
"""
import re
import sys

def fix_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()

    original = content

    # Find all remove_volatile calls with the (VAR.side_index, VAR.position) pattern
    # and track what variables they use
    remove_volatile_pattern = r'Pokemon::remove_volatile\(battle, \((\w+)\.side_index, \1\.position\), (&[^)]+)\)'

    # For each match, find the corresponding pokemon_at_mut and fix both
    def fix_instance(match):
        var_name = match.group(1)
        volatile_id = match.group(2)

        # Look backward in the content to find the let var_name = match battle.pokemon_at_mut pattern
        # We'll need to process the whole file
        return match.group(0)  # Return unchanged for now, we'll fix in post-processing

    # Strategy: Process line by line, tracking variable assignments
    lines = content.split('\n')
    var_to_pos = {}  # Maps variable name to position variable
    new_lines = []

    i = 0
    while i < len(lines):
        line = lines[i]

        # Check if this is a pokemon_at_mut assignment
        match = re.search(r'let (\w+) = match battle\.pokemon_at_mut\((\w+)\.0, \2\.1\)', line)
        if match:
            var_name = match.group(1)
            pos_var = match.group(2)
            var_to_pos[var_name] = pos_var

            # Check if we should skip this whole block and replace with direct call
            # Look ahead to see if there's a remove_volatile call using this var
            j = i
            found_remove = False
            while j < min(i + 10, len(lines)):
                if f'Pokemon::remove_volatile(battle, ({var_name}.side_index, {var_name}.position)' in lines[j]:
                    found_remove = True
                    # Extract the volatile ID
                    remove_match = re.search(r'Pokemon::remove_volatile\(battle, \(' + var_name + r'\.side_index, ' + var_name + r'\.position\), (&[^)]+)\)', lines[j])
                    if remove_match:
                        volatile_id = remove_match.group(1)
                        # Replace the whole block
                        new_lines.append(f'Pokemon::remove_volatile(battle, {pos_var}, {volatile_id});')
                        # Skip lines until we find the remove_volatile call
                        while i < j:
                            i += 1
                        i += 1  # Skip the remove_volatile line too
                        found_remove = True
                        break
                j += 1

            if not found_remove:
                new_lines.append(line)
                i += 1
        else:
            new_lines.append(line)
            i += 1

    new_content = '\n'.join(new_lines)

    if new_content != original:
        with open(filepath, 'w') as f:
            f.write(new_content)
        print(f"Fixed: {filepath}")
        return True
    return False

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: fix_e0499_v2.py <files...>")
        sys.exit(1)

    for filepath in sys.argv[1:]:
        try:
            fix_file(filepath)
        except Exception as e:
            print(f"Error processing {filepath}: {e}")
