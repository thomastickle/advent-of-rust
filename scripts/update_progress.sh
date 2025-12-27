#!/bin/bash

README_FILE="readme.md"

# Loop through days 01 to 25
for i in {1..25}; do
    DAY=$(printf "%02d" $i)
    FILE="src/day${DAY}.rs"
    
    if [ -f "$FILE" ]; then
        echo "Testing Day ${DAY}..."
        if cargo test "day${DAY}" --quiet > /dev/null 2>&1 || cargo.exe test "day${DAY}" --quiet > /dev/null 2>&1; then
            STATUS="✅"
        else
            STATUS="❌"
        fi
    else
        STATUS="⬜"
    fi

    # Update the status in readme.md
    # Pattern matches "- [anything] Day X" and captures the rest of the line
    sed -i "s/- .* Day ${i}\b\(.*\)/- ${STATUS} Day ${i}\1/" "$README_FILE"
done

echo "Progress updated in $README_FILE"
