#!/usr/bin/env pwsh
Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Show-Usage {
    @"
Usage: install-unica.ps1 [options]

Download the Unica win-x64 package from GitHub Releases, install it into Codex,
and verify fresh-session visibility.

Options:
  --version VERSION       Release tag to install, for example v0.4.2 (default: latest)
  --target TARGET         Override detected target: win-x64
  --marketplace-name NAME Codex marketplace name (default: unica-local)
  --codex-home DIR        Codex home directory (default: `$env:CODEX_HOME or `$HOME/.codex)
  --skip-verify           Do not run codex debug prompt-input verification
  --print-download-url    Print the resolved release asset URL and exit
  -h, --help              Show this help
"@
}

function Read-OptionValue {
    param(
        [string[]] $Items,
        [int] $Index,
        [string] $Name
    )
    if ($Index + 1 -ge $Items.Count) {
        throw "missing value for $Name"
    }
    return $Items[$Index + 1]
}

function Detect-Target {
    if ($IsWindows -and [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture -eq [System.Runtime.InteropServices.Architecture]::X64) {
        return "win-x64"
    }
    throw "Unsupported Unica PowerShell installer host. Use --target win-x64 on Windows x64."
}

function Get-ArchiveExtension {
    param([string] $Target)
    switch ($Target) {
        "win-x64" { return "zip" }
        default { throw "Unsupported Unica release target for install-unica.ps1: $Target" }
    }
}

function Get-DefaultCodexHome {
    if ($env:CODEX_HOME) {
        return $env:CODEX_HOME
    }
    if ($HOME) {
        return (Join-Path $HOME ".codex")
    }
    if ($env:USERPROFILE) {
        return (Join-Path $env:USERPROFILE ".codex")
    }
    throw "CODEX_HOME, HOME, or USERPROFILE is required to install Unica."
}

function Get-ReleaseAssetUrl {
    param(
        [string] $Repo,
        [string] $Target,
        [string] $Version
    )
    $ext = Get-ArchiveExtension $Target
    $asset = "unica-codex-marketplace-$Target.$ext"
    if ($Version -eq "latest") {
        return "https://github.com/$Repo/releases/latest/download/$asset"
    }
    return "https://github.com/$Repo/releases/download/$Version/$asset"
}

function Find-MarketplaceRoot {
    param([string] $Root)
    $marker = Get-ChildItem -LiteralPath $Root -Recurse -File -Filter "marketplace.json" |
        Where-Object { $_.FullName -like "*.agents*plugins*marketplace.json" } |
        Select-Object -First 1
    if (-not $marker) {
        throw "Downloaded archive does not contain .agents/plugins/marketplace.json"
    }
    return (Resolve-Path (Join-Path $marker.DirectoryName "..\..\..")).Path
}

function Read-PluginVersion {
    param([string] $PluginJson)
    $plugin = Get-Content -LiteralPath $PluginJson -Raw | ConvertFrom-Json
    if (-not $plugin.version) {
        throw "Cannot read plugin version from $PluginJson"
    }
    return [string] $plugin.version
}

function Enable-CodexPlugin {
    param(
        [string] $CodexHome,
        [string] $MarketplaceName
    )
    $config = Join-Path $CodexHome "config.toml"
    $configDir = Split-Path -Parent $config
    New-Item -ItemType Directory -Force -Path $configDir | Out-Null

    $table = "[plugins.`"unica@$MarketplaceName`"]"
    $lines = @()
    if (Test-Path -LiteralPath $config) {
        $skip = $false
        foreach ($line in Get-Content -LiteralPath $config) {
            if ($line -eq $table) {
                $skip = $true
                continue
            }
            if ($skip -and $line.StartsWith("[")) {
                $skip = $false
            }
            if (-not $skip) {
                $lines += $line
            }
        }
    }

    $lines += ""
    $lines += $table
    $lines += "enabled = true"
    Set-Content -LiteralPath $config -Value $lines -Encoding UTF8
}

$repo = if ($env:UNICA_REPO) { $env:UNICA_REPO } else { "IngvarConsulting/unica" }
$version = if ($env:UNICA_VERSION) { $env:UNICA_VERSION } else { "latest" }
$target = if ($env:UNICA_TARGET) { $env:UNICA_TARGET } else { "" }
$marketplaceName = if ($env:UNICA_CODEX_MARKETPLACE_NAME) { $env:UNICA_CODEX_MARKETPLACE_NAME } else { "unica-local" }
$codexHome = if ($env:CODEX_HOME) { $env:CODEX_HOME } else { "" }
$doVerify = $true
$printDownloadUrl = $false

$i = 0
while ($i -lt $args.Count) {
    switch ($args[$i]) {
        { $_ -in @("--version", "-Version") } {
            $version = Read-OptionValue $args $i $args[$i]
            $i += 2
            continue
        }
        { $_ -in @("--target", "-Target") } {
            $target = Read-OptionValue $args $i $args[$i]
            $i += 2
            continue
        }
        { $_ -in @("--marketplace-name", "-MarketplaceName") } {
            $marketplaceName = Read-OptionValue $args $i $args[$i]
            $i += 2
            continue
        }
        { $_ -in @("--codex-home", "-CodexHome") } {
            $codexHome = Read-OptionValue $args $i $args[$i]
            $i += 2
            continue
        }
        { $_ -in @("--skip-verify", "-SkipVerify") } {
            $doVerify = $false
            $i += 1
            continue
        }
        { $_ -in @("--print-download-url", "-PrintDownloadUrl") } {
            $printDownloadUrl = $true
            $i += 1
            continue
        }
        { $_ -in @("-h", "--help", "-Help") } {
            Show-Usage
            exit 0
        }
        default {
            [Console]::Error.WriteLine("Unknown argument: $($args[$i])")
            Show-Usage
            exit 64
        }
    }
}

if (-not $target) {
    $target = Detect-Target
}

$url = Get-ReleaseAssetUrl $repo $target $version
if ($printDownloadUrl) {
    Write-Output $url
    exit 0
}

if (-not $codexHome) {
    $codexHome = Get-DefaultCodexHome
}

if (-not (Get-Command codex -ErrorAction SilentlyContinue)) {
    throw "codex CLI is required to install Unica."
}

$tmpRoot = Join-Path ([System.IO.Path]::GetTempPath()) ("unica-install." + [System.Guid]::NewGuid().ToString("N"))
New-Item -ItemType Directory -Path $tmpRoot | Out-Null

try {
    $archive = Join-Path $tmpRoot "unica-codex-marketplace-$target.$(Get-ArchiveExtension $target)"
    $extractDir = Join-Path $tmpRoot "extract"
    New-Item -ItemType Directory -Path $extractDir | Out-Null

    Write-Output "==> Unica target: $target"
    Write-Output "==> Download: $url"
    Invoke-WebRequest -Uri $url -OutFile $archive
    Expand-Archive -LiteralPath $archive -DestinationPath $extractDir -Force

    $extractedMarketplaceDir = Find-MarketplaceRoot $extractDir
    $marketplaceDir = Join-Path $codexHome "marketplaces\$marketplaceName"
    Remove-Item -LiteralPath $marketplaceDir -Recurse -Force -ErrorAction SilentlyContinue
    New-Item -ItemType Directory -Force -Path (Split-Path -Parent $marketplaceDir) | Out-Null
    Copy-Item -LiteralPath $extractedMarketplaceDir -Destination $marketplaceDir -Recurse

    & pwsh -NoProfile -File (Join-Path $marketplaceDir "plugins\unica\scripts\run-v8-runner.ps1") config init --help | Out-Null
    & pwsh -NoProfile -File (Join-Path $marketplaceDir "plugins\unica\scripts\run-unica.ps1") --help | Out-Null

    $pluginVersion = Read-PluginVersion (Join-Path $marketplaceDir "plugins\unica\.codex-plugin\plugin.json")
    $pluginCacheDir = Join-Path $codexHome "plugins\cache\$marketplaceName\unica"
    $pluginCacheVersionDir = Join-Path $pluginCacheDir $pluginVersion

    & codex plugin marketplace remove $marketplaceName | Out-Null
    if (Test-Path -LiteralPath $pluginCacheDir) {
        Write-Output "==> Removing stale Codex plugin cache: $pluginCacheDir"
        Remove-Item -LiteralPath $pluginCacheDir -Recurse -Force
    }

    & codex plugin marketplace add $marketplaceDir
    New-Item -ItemType Directory -Force -Path $pluginCacheDir | Out-Null
    Copy-Item -LiteralPath (Join-Path $marketplaceDir "plugins\unica") -Destination $pluginCacheVersionDir -Recurse
    Enable-CodexPlugin $codexHome $marketplaceName

    if ($doVerify) {
        $tmpDir = Join-Path $codexHome "tmp"
        New-Item -ItemType Directory -Force -Path $tmpDir | Out-Null
        $promptProof = Join-Path $tmpDir "unica-install-prompt-input.json"
        & codex debug prompt-input "test" > $promptProof
        foreach ($needle in @("Unica", "workspace-init", "db-auth-check")) {
            if (-not (Select-String -LiteralPath $promptProof -Pattern $needle -Quiet)) {
                throw "Codex prompt verification did not contain '$needle'. Saved prompt proof: $promptProof"
            }
        }
        Write-Output "==> Fresh prompt proof: $promptProof"
    }

    Write-Output "==> Installed Unica $pluginVersion in Codex as marketplace '$marketplaceName'"
}
finally {
    Remove-Item -LiteralPath $tmpRoot -Recurse -Force -ErrorAction SilentlyContinue
}
