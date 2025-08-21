#!/usr/bin/env python3
import os
import re

def fix_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()
    
    # Fix deprecated methods
    content = re.sub(r'\.get_single\(\)', '.get_single()', content)
    content = re.sub(r'\.get_single_mut\(\)', '.get_single_mut()', content)
    content = re.sub(r'\.send\(', '.write(', content)
    content = re.sub(r'\.despawn_recursive\(\)', '.despawn()', content)
    content = re.sub(r'time\.delta_seconds\(\)', 'time.delta_secs()', content)
    content = re.sub(r'time\.elapsed_seconds\(\)', 'time.elapsed_secs()', content)
    content = re.sub(r'\.percent_left\(\)', '.fraction_remaining()', content)
    
    with open(filepath, 'w') as f:
        f.write(content)

# Walk through src directory
for root, dirs, files in os.walk('src'):
    for file in files:
        if file.endswith('.rs'):
            filepath = os.path.join(root, file)
            fix_file(filepath)
            print(f"Fixed: {filepath}")

print("All files updated!")
