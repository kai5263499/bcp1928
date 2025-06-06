#!/bin/bash

shopt -s globstar

# Check if the markdown directory exists, if not, create it
if [ -d "markdown" ]; then
    rm -f markdown
fi

mkdir markdown

# Loop over all HTML files in the html directory
for html_file in html/**/*.html; do
    # Get the base name of the file (without directories and extension)
    base_name=$(basename "$html_file" .html)
    
    # Use pandoc to convert the HTML file to Markdown
    pandoc "$html_file" -f html -t markdown -o "markdown/$base_name.md"
done
