from __future__ import annotations

import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
WORKFLOW = REPO_ROOT / ".github" / "workflows" / "unica-plugin-release.yml"
INSTALL_UNICA_PS1 = REPO_ROOT / "scripts" / "install-unica.ps1"


class UnicaWorkflowGuardrailTests(unittest.TestCase):
    def workflow_text(self) -> str:
        return WORKFLOW.read_text(encoding="utf-8")

    def test_pull_request_paths_cover_all_plugin_sources(self) -> None:
        text = self.workflow_text()
        required_paths = [
            ".agents/plugins/marketplace.json",
            ".github/workflows/unica-plugin-release.yml",
            "Cargo.toml",
            "Cargo.lock",
            "crates/unica-coder/**",
            "plugins/unica/**",
            "scripts/ci/**",
            "scripts/install-unica.sh",
            "scripts/install-unica.ps1",
            "tests/ci/**",
            "tests/fixtures/**",
            "spec/**",
        ]

        for path in required_paths:
            with self.subTest(path=path):
                self.assertIn(f'- "{path}"', text)

    def test_verify_source_job_runs_full_guardrail_suite(self) -> None:
        text = self.workflow_text()
        required_tokens = [
            "verify-source:",
            'branches:',
            '- "main"',
            '- "release/windows-first-0.4.2"',
            'tags:',
            '- "v*"',
            "Checkout repository",
            'git fetch --depth=1 origin "${GITHUB_REF}"',
            "Check Python",
            "python -m pip install -r tests/ci/requirements.txt",
            "Set up Rust",
            "rustup toolchain install stable --profile minimal",
            "rustup component add rustfmt clippy",
            "python -m unittest discover -s tests/ci",
            "python -m py_compile scripts/ci/*.py tests/ci/*.py",
            "python -m json.tool plugins/unica/.codex-plugin/plugin.json >/dev/null",
            "python -m json.tool plugins/unica/.mcp.json >/dev/null",
            "python -m json.tool plugins/unica/third-party/tools.lock.json >/dev/null",
            "python -m json.tool plugins/unica/third-party/manifest.json >/dev/null",
            "bash -n plugins/unica/scripts/*.sh",
            "Check PowerShell launchers",
            "shell: pwsh",
            "plugins/unica/scripts/*.ps1",
            "System.Management.Automation.Language.Parser]::ParseFile",
            "cargo fmt --all -- --check",
            "continue-on-error: true",
            "cargo clippy --package unica-coder --all-targets --all-features",
            "cargo test --package unica-coder",
        ]

        for token in required_tokens:
            with self.subTest(token=token):
                self.assertIn(token, text)

    def test_package_job_waits_for_source_verification_and_sets_up_rust(self) -> None:
        text = self.workflow_text()
        self.assertIn("package:", text)
        self.assertIn("needs: verify-source", text)
        self.assertIn("rustup toolchain install stable --profile minimal", text)
        self.assertIn("Build target bundle", text)
        self.assertIn("--out-dir \".build/tool-artifacts/unica-tools-${{ matrix.target }}\"", text)
        self.assertIn("Assemble marketplace package", text)
        self.assertIn("python scripts/ci/build-unica-tools.py", text)

    def test_release_workflow_publishes_both_installers_and_smokes_windows_package(self) -> None:
        text = self.workflow_text()
        required_tokens = [
            "cp scripts/install-unica.sh dist/install-unica.sh",
            "cp scripts/install-unica.ps1 dist/install-unica.ps1",
            "Smoke Windows package MCP launcher",
            "unica-codex-marketplace-win-x64.zip",
            'GH_TOKEN: ${{ github.token }}',
            "api.github.com/repos/${GITHUB_REPOSITORY}/releases",
            "releases/tags/${RELEASE_TAG}",
            "--data-binary @\"${asset}\"",
            "shell: pwsh",
            "run-unica.ps1",
        ]

        for token in required_tokens:
            with self.subTest(token=token):
                self.assertIn(token, text)

    def test_windows_installer_copies_marketplace_contents(self) -> None:
        text = INSTALL_UNICA_PS1.read_text(encoding="utf-8")
        required_tokens = [
            'Join-Path $marker.DirectoryName "..\\.."',
            "New-Item -ItemType Directory -Force -Path $marketplaceDir",
            "Get-ChildItem -LiteralPath $extractedMarketplaceDir -Force",
            "Copy-Item -Destination $marketplaceDir -Recurse -Force",
            'Join-Path $marketplaceDir "plugins\\unica\\scripts\\run-unica.ps1"',
        ]

        for token in required_tokens:
            with self.subTest(token=token):
                self.assertIn(token, text)


if __name__ == "__main__":
    unittest.main()
