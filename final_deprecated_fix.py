#!/usr/bin/env python3
import os
import re

def fix_file(filepath):
    try:
        with open(filepath, 'r') as f:
            content = f.read()
        
        original = content
        
        # Fix get_single() calls - this is already correct in Bevy 0.16
        # but we need to change it to single() for the newer API
        content = re.sub(r'\.get_single\(\)', '.single()', content)
        content = re.sub(r'\.get_single_mut\(\)', '.single_mut()', content)
        
        if content != original:
            with open(filepath, 'w') as f:
                f.write(content)
            return True
        return False
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
