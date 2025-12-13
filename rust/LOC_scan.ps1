function Generate-LOCReports {
    param(
        [string[]]$RustExts = @("rs", "toml"),
        [string]$OutputBaseDir = "loc_reports",
        [string]$SummaryFile = $("loc_summary_{0}.txt" -f (Get-Random))
    )

    # ✅ Auto-detect the directory where this script lives (never null)
    $ScriptRoot = $PSScriptRoot
    Set-Location $ScriptRoot

    $DateString = Get-Date -UFormat '%Y-%m-%d %H:%M:%S'
    $Timestamp = [long](Get-Date -UFormat %s)
    $ProjectRoot = [System.Text.RegularExpressions.Regex]::Escape($ScriptRoot + '\')

    # ✅ Output directory
    $OutputDirPath = Join-Path -Path $ScriptRoot -ChildPath $OutputBaseDir
    if (-not (Test-Path $OutputDirPath)) {
        New-Item -ItemType Directory -Force -Path $OutputDirPath | Out-Null
    }

    $Summary = [System.Collections.ArrayList]::new()
    $TotalLoc = 0

    $Summary.Add("Rust LOC Summary Report")
    $Summary.Add("Generated (UTC): $DateString")
    $Summary.Add("Generated (Epoch Seconds): $Timestamp")
    $Summary.Add("Root Directory: $ScriptRoot")
    $Summary.Add("Target Extensions: $($RustExts -join ', ')`n")
    $Summary.Add("------------------------------------------------------")
    $Summary.Add(" FILE LOC | Relative File Path")
    $Summary.Add("------------------------------------------------------")

    # ✅ Treat script root as a module + its subdirectories
    $ModuleDirs = @(
    @{ Name = (Split-Path $ScriptRoot -Leaf); FullName = $ScriptRoot }
) + (Get-ChildItem -Path $ScriptRoot -Directory -ErrorAction SilentlyContinue | Where-Object {
    $_.Name -ne "target"
} | ForEach-Object {
    @{ Name = $_.Name; FullName = $_.FullName }
})


    foreach ($Module in $ModuleDirs) {
        $ModuleName = $Module.Name
        $ModulePath = $Module.FullName
        $ModuleReportPath = Join-Path $OutputDirPath "$ModuleName.txt"
        Remove-Item $ModuleReportPath -ErrorAction SilentlyContinue

        # ✅ Gather Rust/TOML files
        $ModuleFiles = Get-ChildItem -Path $ModulePath -Recurse -File -ErrorAction SilentlyContinue | Where-Object {
            $RelPath = $_.FullName -replace $ProjectRoot, ''
            if ($RelPath -like "*\target\*" -or $RelPath -like "*\iteration5\*") { return $false }
            $ext = $_.Extension.TrimStart('.')
            return $RustExts -contains $ext
        }

        foreach ($File in $ModuleFiles) {
            $Loc = (Get-Content -Path $File.FullName -ErrorAction SilentlyContinue | Measure-Object -Line).Lines
            $RelPath = $File.FullName -replace $ProjectRoot, ''

            $Summary.Add(("{0,9} LOC | {1}" -f $Loc, $RelPath))
            $TotalLoc += $Loc

            $Content = Get-Content -Path $File.FullName -Raw -Encoding UTF8 -ErrorAction SilentlyContinue
            $FileHeader = ">>> FILE START: $RelPath ($Loc LOC) <<<"
            $FileFooter = "<<< FILE END: $RelPath >>>"

            ($FileHeader + "`n" + $Content + "`n" + $FileFooter + "`n") |
                Out-File -FilePath $ModuleReportPath -Encoding UTF8 -Append
        }
    }

    $Summary.Add("------------------------------------------------------")
    $Summary.Add(("{0,9} LOC | TOTAL" -f $TotalLoc))
    $Summary.Add("------------------------------------------------------")

    $FullSummaryPath = Join-Path $OutputDirPath $SummaryFile
    $Summary -join "`n" | Out-File -FilePath $FullSummaryPath -Encoding UTF8

    Write-Host "Rust module reports saved in: $OutputDirPath"
    Write-Host "LOC summary saved: $FullSummaryPath"
}

# ✅ Call the function (no TargetDirs needed)
Generate-LOCReports
