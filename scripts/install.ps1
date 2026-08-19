# Windows PowerShell Installation Script for CFX

$ErrorActionPreference = "Stop"

$InstallDir = "$env:LOCALAPPDATA\cfx\bin"
$AssetName = "cfx-windows-x86_64.exe"

# Get latest release from GitHub API
Write-Host "Fetching latest CFX release..."
$LatestRelease = Invoke-RestMethod -Uri "https://api.github.com/repos/raushan728/cfx/releases/latest" -ErrorAction SilentlyContinue

if ($null -eq $LatestRelease) {
    # Fallback to tags if no latest is marked
    $Tags = Invoke-RestMethod -Uri "https://api.github.com/repos/raushan728/cfx/tags"
    if ($Tags.Length -eq 0) {
        Write-Error "Could not find any releases."
        exit 1
    }
    $LatestTag = $Tags[0].name
} else {
    $LatestTag = $LatestRelease.tag_name
}

$DownloadUrl = "https://github.com/raushan728/cfx/releases/download/$LatestTag/$AssetName"

if (!(Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
}

$TargetExe = Join-Path $InstallDir "cfx.exe"

Write-Host "Downloading $DownloadUrl ..."
Invoke-WebRequest -Uri $DownloadUrl -OutFile $TargetExe

Write-Host "Successfully downloaded cfx.exe to $InstallDir"

# Add to PATH if not already there
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    $NewPath = $UserPath + ";$InstallDir"
    [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
    Write-Host "Added $InstallDir to your User PATH variable."
    Write-Host "Please restart your terminal to use the 'cfx' command globally."
} else {
    Write-Host "CFX is already in your PATH."
}

Write-Host "Installation complete! Run 'cfx --help' to get started."
