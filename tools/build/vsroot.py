"""Where Visual Studio 2022 lives: any edition (Community, Professional, Enterprise, Build Tools).

VSROOT in the environment wins, then vswhere (the newest install with the x86/x64 C++ tools), then the
Community path the scripts used to hard-code. tools/build/vsroot.cmd does the same for the .cmd scripts.
"""
import os
import subprocess

LEGACY = r"C:\Program Files\Microsoft Visual Studio\2022\Community"
VSWHERE = os.path.join(os.environ.get("ProgramFiles(x86)", r"C:\Program Files (x86)"),
                       r"Microsoft Visual Studio\Installer\vswhere.exe")


def find() -> str:
    env = os.environ.get("VSROOT")
    if env:
        return env
    if os.path.exists(VSWHERE):
        try:
            p = subprocess.run([VSWHERE, "-latest", "-products", "*", "-requires",
                                "Microsoft.VisualStudio.Component.VC.Tools.x86.x64", "-property", "installationPath"],
                               capture_output=True, text=True, timeout=30)
            path = p.stdout.strip().splitlines()[0] if p.stdout.strip() else ""
            if path:
                return path
        except (OSError, subprocess.SubprocessError):
            pass
    return LEGACY


if __name__ == "__main__":
    print(find())
