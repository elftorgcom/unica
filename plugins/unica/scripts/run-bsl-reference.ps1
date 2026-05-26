param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$Args
)

$ErrorActionPreference = "Stop"
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
& (Join-Path $scriptDir "run-bsl-analyzer.ps1") "mcp" "serve" "--profile" "reference" @Args
exit $LASTEXITCODE
