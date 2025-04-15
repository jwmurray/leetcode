#!/bin/bash

# Check if the user has provided a file name
if [ -z "$1" ]; then
    filename="file.txt"
else
    filename="$1"
fi

# Read the file line by line
# store the line number in line_number
line_number=1
while IFS= read -r line; do
    # Print the line
    if [ "$line_number" -eq 10 ]; then
        echo "$line"
    fi
    line_number=$((line_number + 1))
done < "$filename"

