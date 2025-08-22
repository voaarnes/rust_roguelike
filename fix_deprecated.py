#!/usr/bin/env python3
import os
import re

def fix_file(filepath):
    try:
        with open(filepath, 'r') as f:
            content = f.read()
        
        # Fix deprecated methods
        replacements = [
            (r'\.get_single\(\)', '.get_single()'),
            (r'\.get_single_mut\(\)', '.get_single_mut()'),
            (r'\.send\(', '.write('),
            (r'\.despawn_recursive\(\)', '.despawn()'),
            (r'time\.delta_seconds\(\)', 'time.delta_secs()'),
            (r'time\.elapsed_seconds\(\)', 'time.elapsed_secs()'),
            (r'\.percent_left\(\)', '.fraction_remaining()'),
        ]
        
        for pattern, replacement in replacements:
            content = re.sub(pattern, replacement, content)
        
        with open(filepath, 'w') as f:
            f.write(content)
        return True
    except Exception as e:
        print(f"Error fixing {filepath}: {e}")
        return False

# Walk through src directory
fixed_count = 0
for root, dirs, files in os.walk('src'):
    for file in files:
        if file.endswith('.rs'):
            filepath = os.path.join(root, file)
            if fix_file(filepath):
                fixed_count += 1
                print(f"Fixed: {filepath}")

print(f"\nTotal files fixed: {fixed_count}")
