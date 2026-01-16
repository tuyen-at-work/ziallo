#!/bin/bash

# Script to replace giallo/Giallo/GIALLO with zalo/Zalo/ZALO
# in both file/folder names and file contents
# Excludes .git and target folders

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Starting replacement process...${NC}\n"

# Step 1: Replace content in files (before renaming to avoid path issues)
echo -e "${GREEN}Step 1: Replacing content in files...${NC}"

find . -type f \
    -not -path '*/.git/*' \
    -not -path '*/target/*' \
    -not -path '*/.git' \
    -not -path '*/target' \
    -not -name 'replacement.sh' \
    | while read -r file; do
    
    # Skip binary files
    if file "$file" | grep -q "text"; then
        # Check if file contains any of the patterns
        if grep -qE 'giallo|Giallo|GIALLO' "$file" 2>/dev/null; then
            echo "  Updating content: $file"
            
            # Create temporary file and replace all variations
            sed -e 's/giallo/zalo/g' \
                -e 's/Giallo/Zalo/g' \
                -e 's/GIALLO/ZALO/g' \
                "$file" > "${file}.tmp"
            
            # Replace original file
            mv "${file}.tmp" "$file"
        fi
    fi
done

echo -e "\n${GREEN}Step 2: Renaming files and directories...${NC}"

# Step 2: Rename files and directories (depth-first to handle nested items)
find . -depth \
    -not -path '*/.git/*' \
    -not -path '*/target/*' \
    -not -path '*/.git' \
    -not -path '*/target' \
    -not -name 'replacement.sh' \
    | while read -r path; do
    
    # Skip if it's just the current directory
    if [ "$path" = "." ]; then
        continue
    fi
    
    dir=$(dirname "$path")
    base=$(basename "$path")
    
    # Replace all variations in the name
    new_base="$base"
    new_base="${new_base//giallo/zalo}"
    new_base="${new_base//Giallo/Zalo}"
    new_base="${new_base//GIALLO/ZALO}"
    
    new_path="$dir/$new_base"
    
    # Rename if the new name is different
    if [ "$path" != "$new_path" ]; then
        echo "  Renaming: $path -> $new_path"
        mv "$path" "$new_path"
    fi
done

echo -e "\n${BLUE}Replacement complete!${NC}"
echo -e "${GREEN}Summary:${NC}"
echo "  - Replaced 'giallo' with 'zalo'"
echo "  - Replaced 'Giallo' with 'Zalo'"
echo "  - Replaced 'GIALLO' with 'ZALO'"
echo "  - In both file/folder names and file contents"
echo "  - Excluded .git and target folders"
echo "  - Excluded replace_script.sh file"