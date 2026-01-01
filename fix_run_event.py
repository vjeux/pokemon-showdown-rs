#!/usr/bin/env python3
import re
import os
import sys

def fix_run_event_calls(file_path):
    """Add 'false' parameter to run_event calls that don't have it."""
    with open(file_path, 'r') as f:
        content = f.read()

    # Pattern to match .run_event(...) calls
    # We need to handle multi-line calls
    pattern = r'\.run_event\s*\('

    modified = False
    new_content = []
    i = 0

    while i < len(content):
        match = re.match(r'\.run_event\s*\(', content[i:])
        if match:
            # Found .run_event(
            start = i
            i += len(match.group(0))

            # Find the matching closing parenthesis
            paren_count = 1
            in_string = False
            string_char = None
            escaped = False

            while i < len(content) and paren_count > 0:
                char = content[i]

                if escaped:
                    escaped = False
                elif char == '\\':
                    escaped = True
                elif char in ['"', "'"] and not in_string:
                    in_string = True
                    string_char = char
                elif in_string and char == string_char:
                    in_string = False
                    string_char = None
                elif not in_string:
                    if char == '(':
                        paren_count += 1
                    elif char == ')':
                        paren_count -= 1

                i += 1

            # Extract the call
            call = content[start:i]

            # Check if it already has 'true' or 'false' as last parameter
            # Look for the pattern: , true) or , false) at the end
            if not re.search(r',\s*(true|false)\s*\)\s*$', call):
                # Need to add ', false' before the closing )
                # Find the last )
                last_paren = call.rfind(')')
                new_call = call[:last_paren] + ', false' + call[last_paren:]
                new_content.append(new_call)
                modified = True
            else:
                new_content.append(call)
        else:
            new_content.append(content[i])
            i += 1

    if modified:
        with open(file_path, 'w') as f:
            f.write(''.join(new_content))
        return True
    return False

def main():
    # Find all .rs files in src directory
    src_dir = '/Users/vjeux/random/showdown/pokemon-showdown-rs/src'

    count = 0
    for root, dirs, files in os.walk(src_dir):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                if fix_run_event_calls(file_path):
                    count += 1
                    print(f"Fixed: {file_path}")

    print(f"\nTotal files modified: {count}")

if __name__ == '__main__':
    main()
