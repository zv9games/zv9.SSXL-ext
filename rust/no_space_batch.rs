# powershell command. removes no break space. 

(Get-Content -Path ssxl_sync/src/lib.rs) -replace [char]0x00A0, ' ' | Set-Content -Path ssxl_sync/src/lib.rs

# powershell command. removes new visibile corruption from no break space removal. 

# Step 2: Replace the corrupted 'Â' character (U+00C2) with nothing, and the NBSP (U+00A0) with a standard space.
(Get-Content -Path ssxl_sync/src/lib.rs) -replace [char]0x00C2, '' -replace [char]0x00A0, ' ' | Set-Content -Path ssxl_sync/src/lib.rs

#powershell command. cleans the whole directory. 

Get-ChildItem -Path . -Include *.rs -Recurse | ForEach-Object {
    (Get-Content $_.FullName) -replace [char]0x00C2, '' -replace [char]0x00A0, ' ' | Set-Content $_.FullName
}