if (!([System.IO.DirectoryInfo]::new($PWD).Name.Contains('syncr', [System.StringComparison]::OrdinalIgnoreCase))) {
    throw 'Only executable in the root of the syncr dir'
}

Push-Location 'synctest'
try {
    Write-Host 'Removing destination dir'
    Remove-Item 'syncdestination' -Recurse
    Write-Host 'Creating empty destination dir'
    New-Item 'syncdestination' -ItemType Directory | Out-Null
} finally {
    Pop-Location
}