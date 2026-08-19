# Windows PowerShell Uninstallation Script for CFX

$ErrorActionPreference = "Stop"

$InstallDir = "$env:LOCALAPPDATA\cfx\bin"
$TargetExe = Join-Path $InstallDir "cfx.exe"
$BaseDir = "$env:LOCALAPPDATA\cfx"

if (Test-Path $TargetExe) {
    Remove-Item -Path $TargetExe -Force
    Write-Host "CFX binary successfully removed."
} else {
    Write-Host "CFX binary not found at $TargetExe. It might already be uninstalled."
}

if (Test-Path $BaseDir) {
    Remove-Item -Path $BaseDir -Recurse -Force
    Write-Host "CFX directories successfully removed."
}

# Remove from PATH
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -like "*$InstallDir*") {
    # We replace the specific string along with possible semicolons
    $CleanPath = $UserPath -replace [regex]::Escape(";$InstallDir"), ""
    $CleanPath = $CleanPath -replace [regex]::Escape("$InstallDir;"), ""
    $CleanPath = $CleanPath -replace [regex]::Escape("$InstallDir"), ""
    
    [Environment]::SetEnvironmentVariable("Path", $CleanPath, "User")
    Write-Host "Removed $InstallDir from your User PATH variable."
    Write-Host "Please restart your terminal to apply PATH changes globally."
}

Write-Host "CFX has been fully uninstalled."
