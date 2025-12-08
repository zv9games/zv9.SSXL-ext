# Set the root of your workspace
$workspace = "C:\zv9\zv9.SSXL-ext\rust"

# Define regex patterns for #[func] and #[signal]
$funcPattern   = '#\s*\[func\]'
$signalPattern = '#\s*\[signal\]'

# Recursively search all .rs files
Get-ChildItem -Path $workspace -Recurse -Filter *.rs | ForEach-Object {
    $file = $_.FullName
    $lines = Get-Content $file

    for ($i = 0; $i -lt $lines.Length; $i++) {
        $line = $lines[$i]

        # Detect #[func] annotated methods
        if ($line -match $funcPattern) {
            $sig = ($lines | Select-Object -Skip ($i+1) | Where-Object { $_.Trim() -ne "" } | Select-Object -First 1).Trim()
            Write-Output "FUNC: $sig [$file]"
        }

        # Detect #[signal] annotated broadcasts
        if ($line -match $signalPattern) {
            $sig = ($lines | Select-Object -Skip ($i+1) | Where-Object { $_.Trim() -ne "" } | Select-Object -First 1).Trim()
            Write-Output "SIGNAL: $sig [$file]"
        }
    }
}
