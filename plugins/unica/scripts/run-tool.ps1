param(
    [Parameter(Mandatory = $true, Position = 0)]
    [string]$ToolName,

    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$ToolArgs
)

$ErrorActionPreference = "Stop"

function Exit-WithError {
    param(
        [Parameter(Mandatory = $true)]
        [int]$Code,

        [Parameter(Mandatory = $true)]
        [string]$Message
    )

    [Console]::Error.WriteLine($Message)
    exit $Code
}

if ($PSVersionTable.PSVersion.Major -lt 7) {
    Exit-WithError 78 "Unica PowerShell launchers require PowerShell 7+ (pwsh). Current version: $($PSVersionTable.PSVersion)."
}

function Get-CurrentTriple {
    $platform = [System.Environment]::OSVersion.Platform
    $arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString()

    if ($platform -eq [System.PlatformID]::Win32NT) {
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

    if ($IsMacOS) {
        switch ($arch) {
            "Arm64" { return "aarch64-apple-darwin" }
            "X64" { return "x86_64-apple-darwin" }
        }
    }

    return "$([System.Runtime.InteropServices.RuntimeInformation]::OSDescription)-$arch"
}

function Get-CurrentTargetId {
    $platform = [System.Environment]::OSVersion.Platform
    $arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString()

    if ($platform -eq [System.PlatformID]::Win32NT -and $arch -eq "X64") {
        return "win-x64"
    }

    if ($IsLinux -and $arch -eq "X64") {
        return "linux-x64"
    }

    if ($IsMacOS -and $arch -eq "Arm64") {
        return "darwin-arm64"
    }

    Exit-WithError 78 "Unica does not ship binaries for $platform-$arch."
}

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$pluginRoot = Split-Path -Parent $scriptDir
$manifestPath = Join-Path $pluginRoot "third-party/manifest.json"

if (-not (Test-Path -LiteralPath $manifestPath -PathType Leaf)) {
    Exit-WithError 66 "Unica third-party manifest not found: $manifestPath"
}

$manifest = Get-Content -LiteralPath $manifestPath -Raw -Encoding UTF8 | ConvertFrom-Json
$currentTriple = Get-CurrentTriple
$currentTargetId = Get-CurrentTargetId

$tool = $manifest.tools | Where-Object { $_.name -eq $ToolName } | Select-Object -First 1
if (-not $tool) {
    Exit-WithError 64 "tool not found in manifest: $ToolName"
}

if ($tool.binaries) {
    $binary = $tool.binaries.PSObject.Properties[$currentTargetId].Value
    if (-not $binary) {
        $supported = ($tool.binaries.PSObject.Properties.Name | Sort-Object) -join ", "
        Exit-WithError 78 "tool $ToolName is not packaged for $currentTargetId; supported: $supported"
    }
} else {
    if ($manifest.targetTriple -and $manifest.targetTriple -ne $currentTriple) {
        Exit-WithError 78 "Unica ships binaries for $($manifest.targetTriple); current host is $currentTriple."
    }
    $binary = $tool
}

$binaryPath = Join-Path $pluginRoot $binary.binaryPath
if (-not (Test-Path -LiteralPath $binaryPath -PathType Leaf)) {
    Exit-WithError 66 "Unica binary is missing: $binaryPath"
}

$actualSha = (Get-FileHash -LiteralPath $binaryPath -Algorithm SHA256).Hash.ToLowerInvariant()
$expectedSha = [string]$binary.sha256
if ($actualSha -ne $expectedSha.ToLowerInvariant()) {
    Exit-WithError 65 "Unica binary checksum mismatch for $ToolName.`nexpected: $expectedSha`nactual:   $actualSha"
}

& $binaryPath @ToolArgs
exit $LASTEXITCODE
