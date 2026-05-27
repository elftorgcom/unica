from __future__ import annotations

import subprocess
import os
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
SCRIPT = REPO_ROOT / "scripts" / "install-unica.sh"
PS_SCRIPT = REPO_ROOT / "scripts" / "install-unica.ps1"


def script_command(*args: str) -> list[str]:
    if os.name == "nt":
        return ["bash", "./scripts/install-unica.sh", *args]
    return [str(SCRIPT), *args]


def run_script(*args: str, env: dict[str, str] | None = None) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        script_command(*args),
        check=False,
        cwd=REPO_ROOT,
        env=env,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )


def run_ps_script(*args: str, env: dict[str, str] | None = None) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        ["pwsh", "-NoProfile", "-File", str(PS_SCRIPT), *args],
        check=False,
        cwd=REPO_ROOT,
        env=env,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )


class InstallUnicaVerificationNeedlesTests(unittest.TestCase):
    def test_installers_verify_current_skill_surface(self) -> None:
        for script in [SCRIPT, PS_SCRIPT]:
            with self.subTest(script=script.name):
                text = script.read_text(encoding="utf-8")
                self.assertIn("v8-runner", text)
                self.assertIn("meta-compile", text)
                self.assertIn("db-auth-check", text)
                self.assertNotIn("workspace-init", text)

    def test_windows_installer_repairs_mcp_launcher_to_powershell_launcher(self) -> None:
        text = PS_SCRIPT.read_text(encoding="utf-8")

        self.assertIn("Repair-WindowsMcpLauncher $marketplaceDir $target", text)
        self.assertIn('$mcp.mcpServers.unica.command = "pwsh"', text)
        self.assertIn("./plugins/unica/scripts/run-unica.ps1", text)
        self.assertIn("./scripts/run-unica.ps1", text)
        self.assertNotIn('$mcp.mcpServers.unica.command = "./plugins/unica/bin/win-x64/unica.exe"', text)


@unittest.skipIf(os.name == "nt", "install-unica.sh URL checks run on POSIX CI")
class InstallUnicaScriptTests(unittest.TestCase):
    def test_prints_latest_release_asset_url_for_target(self) -> None:
        result = run_script("--target", "darwin-arm64", "--print-download-url")

        self.assertEqual(result.returncode, 0, result.stderr)

        self.assertEqual(
            result.stdout.strip(),
            "https://github.com/elftorgcom/unica/releases/latest/download/"
            "unica-codex-marketplace-darwin-arm64.tar.gz",
        )

    def test_prints_pinned_release_asset_url_for_target(self) -> None:
        result = run_script("--target", "linux-x64", "--version", "v0.3.3", "--print-download-url")

        self.assertEqual(result.returncode, 0, result.stderr)

        self.assertEqual(
            result.stdout.strip(),
            "https://github.com/elftorgcom/unica/releases/download/v0.3.3/"
            "unica-codex-marketplace-linux-x64.tar.gz",
        )

    def test_print_download_url_does_not_require_codex_home(self) -> None:
        env = os.environ.copy()
        env.pop("CODEX_HOME", None)
        env.pop("HOME", None)
        env.pop("USERPROFILE", None)

        result = run_script("--target", "win-x64", "--print-download-url", env=env)

        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(
            result.stdout.strip(),
            "https://github.com/elftorgcom/unica/releases/latest/download/"
            "unica-codex-marketplace-win-x64.zip",
        )


@unittest.skipUnless(os.name == "nt", "install-unica.ps1 URL checks run on Windows CI")
class InstallUnicaPowerShellScriptTests(unittest.TestCase):
    def test_prints_latest_windows_release_asset_url_by_default(self) -> None:
        result = run_ps_script("-PrintDownloadUrl")

        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(
            result.stdout.strip(),
            "https://github.com/elftorgcom/unica/releases/latest/download/"
            "unica-codex-marketplace-win-x64.zip",
        )

    def test_prints_pinned_release_asset_url_for_target(self) -> None:
        result = run_ps_script(
            "-Target",
            "win-x64",
            "-Version",
            "v0.4.2",
            "-PrintDownloadUrl",
        )

        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(
            result.stdout.strip(),
            "https://github.com/elftorgcom/unica/releases/download/v0.4.2/"
            "unica-codex-marketplace-win-x64.zip",
        )

    def test_print_download_url_does_not_require_codex_home(self) -> None:
        env = os.environ.copy()
        env.pop("CODEX_HOME", None)
        env.pop("HOME", None)
        env.pop("USERPROFILE", None)

        result = run_ps_script("-PrintDownloadUrl", env=env)

        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("unica-codex-marketplace-win-x64.zip", result.stdout.strip())


if __name__ == "__main__":
    unittest.main()
