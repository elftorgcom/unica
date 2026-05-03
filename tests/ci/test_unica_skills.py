from __future__ import annotations

import unittest
from pathlib import Path


IN_SCOPE_TOOLS = {
    "cf-edit": "unica.cf.edit",
    "cf-info": "unica.cf.info",
    "cf-init": "unica.cf.init",
    "cf-validate": "unica.cf.validate",
    "cfe-borrow": "unica.cfe.borrow",
    "cfe-diff": "unica.cfe.diff",
    "cfe-init": "unica.cfe.init",
    "cfe-patch-method": "unica.cfe.patch_method",
    "cfe-validate": "unica.cfe.validate",
    "meta-compile": "unica.meta.compile",
    "meta-edit": "unica.meta.edit",
    "meta-info": "unica.meta.info",
    "meta-remove": "unica.meta.remove",
    "meta-validate": "unica.meta.validate",
    "form-add": "unica.form.add",
    "form-compile": "unica.form.compile",
    "form-edit": "unica.form.edit",
    "form-info": "unica.form.info",
    "form-remove": "unica.form.remove",
    "form-validate": "unica.form.validate",
    "interface-edit": "unica.interface.edit",
    "interface-validate": "unica.interface.validate",
    "subsystem-compile": "unica.subsystem.compile",
    "subsystem-edit": "unica.subsystem.edit",
    "subsystem-info": "unica.subsystem.info",
    "subsystem-validate": "unica.subsystem.validate",
    "template-add": "unica.template.add",
    "template-remove": "unica.template.remove",
    "skd-compile": "unica.skd.compile",
    "skd-edit": "unica.skd.edit",
    "skd-info": "unica.skd.info",
    "skd-validate": "unica.skd.validate",
    "mxl-compile": "unica.mxl.compile",
    "mxl-decompile": "unica.mxl.decompile",
    "mxl-info": "unica.mxl.info",
    "mxl-validate": "unica.mxl.validate",
    "role-compile": "unica.role.compile",
    "role-info": "unica.role.info",
    "role-validate": "unica.role.validate",
}

OUT_OF_SCOPE = [
    "db-create",
    "db-dump-cf",
    "workspace-init",
    "epf-build",
    "erf-init",
    "web-test",
    "help-add",
    "img-grid",
]


class UnicaSkillRoutingTests(unittest.TestCase):
    def skill_root(self) -> Path:
        return Path(__file__).resolve().parents[2] / "plugins" / "unica" / "skills"

    def test_in_scope_skills_route_to_single_unica_mcp(self) -> None:
        for skill, tool_name in IN_SCOPE_TOOLS.items():
            with self.subTest(skill=skill):
                text = (self.skill_root() / skill / "SKILL.md").read_text(encoding="utf-8")
                self.assertIn("## MCP routing", text)
                self.assertIn("MCP `unica`", text)
                self.assertIn(tool_name, text)
                self.assertNotIn("unica-coder", text)
                self.assertNotIn("unica-v8-runner", text)
                self.assertNotIn("unica-bsl-workspace", text)
                self.assertNotIn("unica-rlm-tools-bsl", text)
                self.assertNotIn("unica-v8std", text)

    def test_all_skills_do_not_expose_internal_mcp_names(self) -> None:
        forbidden = [
            "unica-coder",
            "unica-v8-runner",
            "unica-bsl-reference",
            "unica-bsl-workspace",
            "unica-rlm-tools-bsl",
            "unica-v8std",
        ]
        for skill_path in self.skill_root().glob("*/SKILL.md"):
            with self.subTest(skill=skill_path.parent.name):
                text = skill_path.read_text(encoding="utf-8")
                for name in forbidden:
                    self.assertNotIn(name, text)


if __name__ == "__main__":
    unittest.main()
