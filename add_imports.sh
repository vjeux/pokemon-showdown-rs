#!/bin/bash

# Update all add_volatile calls to use Pokemon::add_volatile

# Function to update a file
update_file() {
    local file="$1"

    # Skip if already uses Pokemon::add_volatile
    if grep -q "Pokemon::add_volatile" "$file"; then
        return
    fi

    # Skip if no add_volatile calls
    if ! grep -q "\.add_volatile(" "$file"; then
        return
    fi

    echo "Processing: $file"

    # Add Pokemon import if missing
    if grep -q "use crate::event::EventResult;" "$file" && ! grep -q "use crate::pokemon::Pokemon;" "$file"; then
        sed -i '' '/use crate::event::EventResult;/a\
use crate::pokemon::Pokemon;
' "$file"
    elif grep -q "use crate::dex_data::ID;" "$file" && ! grep -q "use crate::pokemon::Pokemon;" "$file"; then
        sed -i '' '/use crate::dex_data::ID;/a\
use crate::pokemon::Pokemon;
' "$file"
    fi
}

# Find all files with add_volatile
find src -name "*.rs" -type f | while read file; do
    if grep -q "\.add_volatile(" "$file" && ! grep -q "Pokemon::add_volatile" "$file"; then
        update_file "$file"
    fi
done

echo "Done!"
