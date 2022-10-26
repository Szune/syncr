if (!([System.IO.DirectoryInfo]::new($PWD).Name.Contains('syncr', [System.StringComparison]::OrdinalIgnoreCase))) {
    throw 'Only executable in the root of the syncr dir'
}

$testDirRoot = 'synctest'
$testDirSource = 'syncsource'
$testDirSourceProject = 'syncrs'
$testDirDestination = 'syncdestination'

# setup test folders
Write-Host 'Setup test folders'
if (!(Test-Path $testDirRoot)) {
    New-Item $testDirRoot -ItemType Directory
}

Push-Location $testDirRoot
try
{
    if (!(Test-Path $testDirSource))
    {
        New-Item $testDirSource -ItemType Directory
    }

    Push-Location $testDirSource
    try {
        if (!(Test-Path $testDirSourceProject))
        {
            New-Item $testDirSourceProject -ItemType Directory
        }
    } finally {
        Pop-Location
    }

    if (!(Test-Path $testDirDestination))
    {
        New-Item $testDirDestination -ItemType Directory
    }
} finally {
    Pop-Location
}

$sourcePath = Join-Path ($PWD.Path) $testDirRoot $testDirSource $testDirSourceProject

# copy source code to test sync with
Write-Host '  Copy source code to test sync with'
Copy-Item './src' $sourcePath -Recurse -Force
Write-Host '  Copy Cargo.toml to test sync with'
Copy-Item './Cargo.toml' $sourcePath -Force

# create folder that is not supposed to be synced
$sourceRustTargetPath = Join-Path $sourcePath 'target'
$sourceRustTargetDebugPath = Join-Path $sourceRustTargetPath 'debug'
$sourceRustTargetDebugTestExePath = Join-Path $sourceRustTargetDebugPath 'fake.exe'

Write-Host '  Create fake exe file target/debug/fake.exe'
if (!(Test-Path $sourceRustTargetPath)) {
    New-Item $sourceRustTargetPath -ItemType Directory | Out-Null
}

if (!(Test-Path $sourceRustTargetDebugPath)) {
    New-Item $sourceRustTargetDebugPath -ItemType Directory | Out-Null
}

New-Item $sourceRustTargetDebugTestExePath -ItemType File -Value '### This is not an exe file, it is used for testing the syncing' -Force | Out-Null

# create a file in the root of the projects folder
Write-Host "  Create file to sync in $testDirRoot/$testDirSource/project-readme.md"
New-Item (Join-Path ($Pwd.Path) $testDirRoot $testDirSource 'project-readme.md') -ItemType File -Value '## README for something' -Force | Out-Null
Write-Host "  Create file to ignore in $testDirRoot/$testDirSource/thing.swp"
New-Item (Join-Path ($Pwd.Path) $testDirRoot $testDirSource 'thing.swp') -ItemType File -Value 'Do not sync' -Force | Out-Null

Write-Host 'Setup done'
