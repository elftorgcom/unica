param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$Args
)

$ErrorActionPreference = "Stop"
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$sourceDir = if ($env:UNICA_BSL_SOURCE_DIR) { $env:UNICA_BSL_SOURCE_DIR } else { "." }
& (Join-Path $scriptDir "run-bsl-analyzer.ps1") "mcp" "serve" "--profile" "workspace" "--source-dir" $sourceDir @Args
exit $LASTEXITCODE
