param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$Args
)

$ErrorActionPreference = "Stop"

if ($PSVersionTable.PSVersion.Major -lt 7) {
    [Console]::Error.WriteLine("Unica PowerShell launchers require PowerShell 7+ (pwsh). Current version: $($PSVersionTable.PSVersion).")
    exit 78
}

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$pluginRoot = Split-Path -Parent $scriptDir
$repoRoot = Split-Path -Parent (Split-Path -Parent $pluginRoot)

if ((Test-Path -LiteralPath (Join-Path $repoRoot "Cargo.toml") -PathType Leaf) -and (Get-Command cargo -ErrorAction SilentlyContinue)) {
    $env:UNICA_PLUGIN_ROOT = $pluginRoot
    cargo run --quiet --package unica-coder --bin unica --manifest-path (Join-Path $repoRoot "Cargo.toml") -- @Args
    exit $LASTEXITCODE
}

& (Join-Path $scriptDir "run-tool.ps1") "unica" @Args
exit $LASTEXITCODE
