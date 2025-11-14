# Requires -Version 3.0

param(
    # Configuration based on your ssxl_cli::scan::report_formatter.rs
    [string[]]$TargetDirs = @(".\rust", ".\SSXL_engine_tester"),
    [string[]]$TargetExts = @("rs", "gd", "gdc", "toml", "md"),
    # The output filename, now prepended with the directory structure.
    [string]$OutputBaseDir = "loc_reports",
    [string]$OutputFile = "ssxl_loc_report_live_{0}.txt" -f (Get-Random)
)

# --- Initialization & Setup ---
$TotalLoc = 0
$Report = [System.Collections.ArrayList]::new()
$SourceCodeDump = [System.Collections.ArrayList]::new()

# Get current date/time and project root
$DateString = Get-Date -UFormat '%Y-%m-%d %H:%M:%S'
$Timestamp = [long](Get-Date -UFormat %s)
$ProjectRoot = [System.Text.RegularExpressions.Regex]::Escape((Get-Item .).FullName + '\')

# Create the output directory if it doesn't exist
$OutputDirPath = Join-Path -Path (Get-Location) -ChildPath $OutputBaseDir
if (-not (Test-Path $OutputDirPath)) {
    Write-Host "Creating output directory: $OutputDirPath"
    New-Item -ItemType Directory -Force -Path $OutputDirPath | Out-Null
}
$FullOutputPath = Join-Path -Path $OutputDirPath -ChildPath $OutputFile

Write-Host "Running SSXL-ext LOC Scan on project root..."

# --- Report Header ---
$Report.Add("SSXL-ext Live LOC Report")
$Report.Add("Generated (UTC): $DateString")
$Report.Add("Generated (Epoch Seconds): $Timestamp")
$Report.Add("Root Directories: $($TargetDirs -join ', ')")
$Report.Add("Target Extensions: $($TargetExts -join ', ')`n")
$Report.Add("------------------------------------------------------")
$Report.Add(" FILE LOC | Relative File Path")
$Report.Add("------------------------------------------------------")

# --- Scanning Loop (LOC Calculation & Source Dumping) ---
foreach ($Dir in $TargetDirs) {
    
    # Get all files recursively, then filter them
    $Files = Get-ChildItem -Path $Dir -Recurse -File -ErrorAction SilentlyContinue | Where-Object {
        $RelPath = $_.FullName -replace $ProjectRoot, ''
        
        # Skip files in 'target' or 'iteration5' directories
        if ($RelPath -like "*\target\*" -or $RelPath -like "*\iteration5\*") {
            return $false
        }
        
        # Only include files that match one of the target extensions
        $ext = $_.Extension.TrimStart('.')
        return $TargetExts -contains $ext
    }

    foreach ($File in $Files) {
        # Calculate LOC using Measure-Object
        $Loc = (Get-Content -Path $File.FullName -ErrorAction SilentlyContinue | Measure-Object -Line).Lines
        
        # Calculate relative path
        $RelPath = $File.FullName -replace $ProjectRoot, ''

        # Add to report and total (LOC Summary)
        $Report.Add([string]::Format("{0,9} LOC | {1}", $Loc, $RelPath))
        $TotalLoc += $Loc

        # --- Source Code Dump Logic ---
        $SourceCodeDump.Add("======================================================")
        
        # CORRECTION 1: Wrap expression in parentheses
        $SourceCodeDump.Add((">>> FILE START: {0} ({1} LOC) <<<" -f $RelPath, $Loc))
        
        $SourceCodeDump.Add("======================================================")
        
        # Read file content and add to dump (using -Raw to preserve formatting/blank lines)
        $Content = Get-Content -Path $File.FullName -Raw -Encoding UTF8 -ErrorAction SilentlyContinue
        $SourceCodeDump.Add($Content)
        
        # CORRECTION 2: Wrap expression in parentheses
        $SourceCodeDump.Add((">>> FILE END: {0} <<<" -f $RelPath))
        
        $SourceCodeDump.Add("") # Add blank line for spacing
    }
}

# --- Final Summary ---
$Report.Add("------------------------------------------------------")
$Report.Add([string]::Format("{0,9} LOC | {1}", $TotalLoc, "TOTAL"))
$Report.Add("------------------------------------------------------")

# --- Append Source Code Dump to Main Report ---
$Report.Add("")
$Report.Add("")
$Report.Add("======================================================")
$Report.Add("          RAW SOURCE CODE DUMP (FOR CONTEXT)")
$Report.Add("======================================================")
$Report.AddRange($SourceCodeDump)


# Write the final report to the specified path
$Report -join "`n" | Out-File -FilePath $FullOutputPath -Encoding UTF8
Write-Host "LOC scan complete. Report saved to $FullOutputPath"