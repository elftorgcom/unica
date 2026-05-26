param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$Args
)

$ErrorActionPreference = "Stop"

if (-not $env:V8TR_CONFIG -and -not (Test-Path -LiteralPath "v8project.yaml" -PathType Leaf)) {
    [Console]::Error.WriteLine("v8-runner MCP requires v8project.yaml in the current directory or V8TR_CONFIG pointing to a config file.")
    [Console]::Error.WriteLine("Run 'plugins/unica/scripts/run-v8-runner.ps1 config init' in a 1C project before starting this MCP server.")
    exit 66
}

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
& (Join-Path $scriptDir "run-v8-runner.ps1") "mcp" "serve" "stdio" @Args
exit $LASTEXITCODE
