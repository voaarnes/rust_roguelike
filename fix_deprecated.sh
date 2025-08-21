#!/bin/bash

# Fix deprecated get_single() -> get_single()
find src -name "*.rs" -exec sed -i 's/\.get_single()/\.get_single()/g' {} \;
find src -name "*.rs" -exec sed -i 's/\.get_single_mut()/\.get_single_mut()/g' {} \;

# Fix deprecated send() -> write()
find src -name "*.rs" -exec sed -i 's/\.send(/\.write(/g' {} \;

# Fix deprecated despawn_recursive() -> despawn()
find src -name "*.rs" -exec sed -i 's/\.despawn_recursive()/\.despawn()/g' {} \;

# Fix Time methods
find src -name "*.rs" -exec sed -i 's/time\.delta_seconds()/time.delta_seconds()/g' {} \;
find src -name "*.rs" -exec sed -i 's/time\.elapsed_seconds()/time.elapsed_seconds()/g' {} \;

# Fix Timer methods
find src -name "*.rs" -exec sed -i 's/\.percent_left()/\.fraction_remaining()/g' {} \;

echo "Deprecated methods fixed!"
