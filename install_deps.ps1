function Invoke-Refresh-Environment {
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")
    [Environment]::SetEnvironmentVariable("Path", $env:Path, [EnvironmentVariableTarget]::User)
    [Environment]::SetEnvironmentVariable("Path", $env:Path, [EnvironmentVariableTarget]::Machine)
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path")
}

$chocoInstalled = Get-Command choco -ErrorAction SilentlyContinue
if (-not $chocoInstalled) {
    Write-Host "Chocolatey is not installed. Installing..."
    Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
    if (Test-Path "$env:ProgramData\chocolatey\bin\choco.exe") {
        Write-Host "Chocolatey installed successfully."
        Invoke-Refresh-Environment
    }
    else {
        Write-Error "Chocolatey installation failed."
    }

}
Write-Host "Installing MinGW using Chocolatey..."
try {
    choco install mingw -y --no-progress --limit-output
    Write-Host "MinGW installed successfully."
    Invoke-Refresh-Environment
    Write-Host "Checking for gcc..."
    if (Get-Command gcc -ErrorAction SilentlyContinue) {
        Write-Host "gcc is installed."
    }
    else {
        Write-Error "gcc is not installed."
    }
}
catch {
    Write-Error "MinGW installation failed: $_"
}
Write-Host "Installing clangFormat using Chocolatey..."
try {
    choco install llvm -y --no-progress --limit-output
    Write-Host "LLVM installed successfully."
    Invoke-Refresh-Environment
    Write-Host "Checking for gcc..."
    if (Get-Command gcc -ErrorAction SilentlyContinue) {
        Write-Host "clangFormat is installed."
    }
    else {
        Write-Error "clangFormat is not installed."
    }
}
catch {
    Write-Error "LLLVM installation failed: $_"
}