function Generate-LOCReportsPerLanguage {
    param(
        [string[]]$TargetDirs,
        [string[]]$RustExts = @("rs", "toml"),
        [string]$OutputBaseDir = "loc_reports",
        [string]$SummaryFile = $("loc_summary_{0}.txt" -f (Get-Random))
    )

    $DateString = Get-Date -UFormat '%Y-%m-%d %H:%M:%S'
    $Timestamp = [long](Get-Date -UFormat %s)
    $ProjectRoot = [System.Text.RegularExpressions.Regex]::Escape((Get-Item .).FullName + '\')

    $OutputDirPath = Join-Path -Path (Get-Location) -ChildPath $OutputBaseDir
    if (-not (Test-Path $OutputDirPath)) {
        New-Item -ItemType Directory -Force -Path $OutputDirPath | Out-Null
    }

    $Summary = [System.Collections.ArrayList]::new()
    $TotalLoc = 0
    $TargetExts = $RustExts + $GdScriptExts + $OtherExts

    $Summary.Add("SSXL-ext LOC Summary Report")
    $Summary.Add("Generated (UTC): $DateString")
    $Summary.Add("Generated (Epoch Seconds): $Timestamp")
    $Summary.Add("Root Directories: $($TargetDirs -join ', ')")
    $Summary.Add("Target Extensions: $($TargetExts -join ', ')`n")
    $Summary.Add("------------------------------------------------------")
    $Summary.Add(" FILE LOC | Relative File Path")
    $Summary.Add("------------------------------------------------------")

    # Clear any old Godot report
    $GodotReportPath = Join-Path $OutputDirPath "godot_source.txt"
    Remove-Item $GodotReportPath -ErrorAction SilentlyContinue

    foreach ($Dir in $TargetDirs) {
        $ModuleDirs = Get-ChildItem -Path $Dir -Directory -ErrorAction SilentlyContinue

        foreach ($Module in $ModuleDirs) {
            $ModuleName = $Module.Name
            $ModulePath = $Module.FullName
            $ModuleReportPath = Join-Path $OutputDirPath "$ModuleName.txt"
            Remove-Item $ModuleReportPath -ErrorAction SilentlyContinue

            $ModuleFiles = Get-ChildItem -Path $ModulePath -Recurse -File -ErrorAction SilentlyContinue | Where-Object {
                $RelPath = $_.FullName -replace $ProjectRoot, ''
                if ($RelPath -like "*\target\*" -or $RelPath -like "*\iteration5\*") { return $false }
                $ext = $_.Extension.TrimStart('.')
                return $RustExts -contains $ext
            }

            foreach ($File in $ModuleFiles) {
                $Loc = (Get-Content -Path $File.FullName -ErrorAction SilentlyContinue | Measure-Object -Line).Lines
                $RelPath = $File.FullName -replace $ProjectRoot, ''
                $ext = $File.Extension.TrimStart('.')

                $Summary.Add(("{0,9} LOC | {1}" -f $Loc, $RelPath))
                $TotalLoc += $Loc

                $Content = Get-Content -Path $File.FullName -Raw -Encoding UTF8 -ErrorAction SilentlyContinue
                $FileHeader = ">>> FILE START: $RelPath ($Loc LOC) <<<"
                $FileFooter = "<<< FILE END: $RelPath >>>"

                ($FileHeader + "`n" + $Content + "`n" + $FileFooter + "`n") | Out-File -FilePath $ModuleReportPath -Encoding UTF8 -Append
            }
        }

        # Handle Godot files separately
        $GodotFiles = Get-ChildItem -Path $Dir -Recurse -File -ErrorAction SilentlyContinue | Where-Object {
            $RelPath = $_.FullName -replace $ProjectRoot, ''
            if ($RelPath -like "*\target\*" -or $RelPath -like "*\iteration5\*") { return $false }
            $ext = $_.Extension.TrimStart('.')
            return $GdScriptExts -contains $ext
        }

        foreach ($File in $GodotFiles) {
            $Loc = (Get-Content -Path $File.FullName -ErrorAction SilentlyContinue | Measure-Object -Line).Lines
            $RelPath = $File.FullName -replace $ProjectRoot, ''
            $Content = Get-Content -Path $File.FullName -Raw -Encoding UTF8 -ErrorAction SilentlyContinue
            $FileHeader = ">>> FILE START: $RelPath ($Loc LOC) <<<"
            $FileFooter = "<<< FILE END: $RelPath >>>"

            $Summary.Add(("{0,9} LOC | {1}" -f $Loc, $RelPath))
            $TotalLoc += $Loc

            ($FileHeader + "`n" + $Content + "`n" + $FileFooter + "`n") | Out-File -FilePath $GodotReportPath -Encoding UTF8 -Append
        }
    }

    $Summary.Add("------------------------------------------------------")
    $Summary.Add(("{0,9} LOC | TOTAL" -f $TotalLoc))
    $Summary.Add("------------------------------------------------------")

    $FullSummaryPath = Join-Path $OutputDirPath $SummaryFile
    $Summary -join "`n" | Out-File -FilePath $FullSummaryPath -Encoding UTF8

    Write-Host "Rust module reports saved in: $OutputDirPath"
    Write-Host "Godot combined report: $GodotReportPath"
    Write-Host "LOC summary saved: $FullSummaryPath"
}

# Call the function so it actually runs
Generate-LOCReportsPerLanguage `
    -TargetDirs @(".\rust", ".\SSXL_engine_tester") `
    -RustExts @("rs", "toml") `
    -GdScriptExts @("gd", "gdc") `
    -OtherExts @("md") `
    -OutputBaseDir "loc_reports"
