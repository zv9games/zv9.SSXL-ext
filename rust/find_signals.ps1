# Set the target string (the incorrect, old node name)
$SearchTerm = "AetherionSignals"

# Define the file extensions most likely to contain Godot node paths/names
$FileExtensions = "*.gd", "*.cs", "*.tscn", "*.tres", "*.cfg", "*.rs"

# --- CORE SEARCH COMMAND ---
Write-Host "🔍 Searching for '$SearchTerm' in current directory and all subdirectories..." -ForegroundColor Cyan

# Get all files with the specified extensions recursively
Get-ChildItem -Path . -Include $FileExtensions -Recurse -ErrorAction SilentlyContinue |
# Search the contents of each file for the search term
Select-String -Pattern $SearchTerm |
# Format the output clearly using the -f (format) operator
ForEach-Object {
    $FilePath = $_.Path
    $LineNumber = $_.LineNumber
    $LineContent = $_.Line.Trim()
    
    Write-Host "✅ FOUND: $FilePath" -ForegroundColor Green
    # Use -f operator for robust string formatting
    Write-Host ('   Line {0}: {1}' -f $LineNumber, $LineContent)
}

Write-Host "`n✅ Search complete. Review the files listed above." -ForegroundColor Cyan