param(
    [Parameter(Mandatory = $true, Position = 0)]
    [string]$ToolName,

    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$ToolArgs
)

$ErrorActionPreference = "Stop"

function Get-CurrentTriple {
    $arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString()

    if ($IsMacOS) {
        switch ($arch) {
            "Arm64" { return "aarch64-apple-darwin" }
            "X64" { return "x86_64-apple-darwin" }
        }
    }

    if ($IsWindows) {
        switch ($arch) {
            "Arm64" { return "aarch64-pc-windows-msvc" }
            "X64" { return "x86_64-pc-windows-msvc" }
        }
    }

    if ($IsLinux) {
        switch ($arch) {
            "Arm64" { return "aarch64-unknown-linux-gnu" }
            "X64" { return "x86_64-unknown-linux-gnu" }
        }
    }

    return "$([System.Runtime.InteropServices.RuntimeInformation]::OSDescription)-$arch"
}

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$pluginRoot = Split-Path -Parent $scriptDir
$manifestPath = Join-Path $pluginRoot "third-party/manifest.json"

if (-not (Test-Path -LiteralPath $manifestPath -PathType Leaf)) {
    Write-Error "Unica third-party manifest not found: $manifestPath"
    exit 66
}

$manifest = Get-Content -LiteralPath $manifestPath -Raw -Encoding UTF8 | ConvertFrom-Json
$currentTriple = Get-CurrentTriple

if ($manifest.targetTriple -and $manifest.targetTriple -ne $currentTriple) {
    Write-Error "Unica ships binaries for $($manifest.targetTriple); current host is $currentTriple."
    exit 78
}

$tool = $manifest.tools | Where-Object { $_.name -eq $ToolName } | Select-Object -First 1
if (-not $tool) {
    Write-Error "tool not found in manifest: $ToolName"
    exit 64
}

$binaryPath = Join-Path $pluginRoot $tool.binaryPath
if (-not (Test-Path -LiteralPath $binaryPath -PathType Leaf)) {
    Write-Error "Unica binary is missing: $binaryPath"
    exit 66
}

$actualSha = (Get-FileHash -LiteralPath $binaryPath -Algorithm SHA256).Hash.ToLowerInvariant()
$expectedSha = [string]$tool.sha256
if ($actualSha -ne $expectedSha.ToLowerInvariant()) {
    Write-Error "Unica binary checksum mismatch for $ToolName.`nexpected: $expectedSha`nactual:   $actualSha"
    exit 65
}

& $binaryPath @ToolArgs
exit $LASTEXITCODE
