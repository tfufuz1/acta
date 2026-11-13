#!/usr/bin/env python3
"""
Python wrapper entry-point for acta-mcp-server binary.
Enables running `acta-mcp` seamlessly via `uvx acta-mcp`.
"""

import os
import sys
import shutil
import subprocess
from pathlib import Path

from acta_mcp import __version__
from acta_mcp._download import get_cached_binary_path, download_release_binary

def find_repo_root() -> Path:
    """Recursively search upward from current file to find repository root containing Cargo.toml."""
    curr = Path(__file__).resolve().parent
    for p in [curr] + list(curr.parents):
        if (p / "Cargo.toml").exists() and (p / "crates" / "acta-mcp").exists():
            return p
    return curr

def is_executable(path: Path) -> bool:
    """Check whether a path exists, is a file, and is executable."""
    return path.is_file() and os.access(path, os.X_OK)

def find_mcp_binary() -> str:
    """
    Locate or download the acta-mcp-server binary following strict priority:
    1. ACTA_MCP_BINARY environment variable (explicit binary path).
    2. acta-mcp-server in system PATH.
    3. Cached release binary (~/.cache/acta-mcp/<version>/ or %LOCALAPPDATA%).
    4. Download GitHub release binary (SHA256 verified).
    5. Repo tree & Cargo build fallback (ONLY if ACTA_MCP_ALLOW_REPO_BUILD=1).
    """
    # 1. Check explicit environment override
    env_bin = os.environ.get("ACTA_MCP_BINARY")
    if env_bin:
        env_path = Path(env_bin)
        if is_executable(env_path):
            return str(env_path)
        sys.stderr.write(
            f"[acta-mcp] Warning: ACTA_MCP_BINARY set to '{env_bin}', but file is missing or not executable.\n"
        )
        sys.stderr.flush()

    # 2. Check system PATH
    which_bin = shutil.which("acta-mcp-server")
    if which_bin and is_executable(Path(which_bin)):
        return which_bin

    # 3. Check local cache directory
    cached_bin = get_cached_binary_path(__version__)
    if is_executable(cached_bin):
        return str(cached_bin)

    # 4. Download release binary from GitHub
    try:
        downloaded_bin = download_release_binary(__version__)
        if is_executable(downloaded_bin):
            return str(downloaded_bin)
    except Exception as exc:
        sys.stderr.write(f"[acta-mcp] Automatic binary download failed: {exc}\n")
        sys.stderr.flush()

    # 5. Repo tree / Cargo build fallback ONLY IF ACTA_MCP_ALLOW_REPO_BUILD=1
    if os.environ.get("ACTA_MCP_ALLOW_REPO_BUILD") == "1":
        repo_root = find_repo_root()
        candidates = [
            repo_root / "target" / "release" / "acta-mcp-server",
            repo_root / "target" / "debug" / "acta-mcp-server",
            Path.cwd() / "target" / "release" / "acta-mcp-server",
            Path.cwd() / "target" / "debug" / "acta-mcp-server",
        ]

        for candidate in candidates:
            if is_executable(candidate):
                return str(candidate)

        if (repo_root / "Cargo.toml").exists():
            sys.stderr.write(
                "[acta-mcp] ACTA_MCP_ALLOW_REPO_BUILD=1 is set. Building release binary via Cargo...\n"
            )
            sys.stderr.flush()
            cmd = ["cargo", "build", "--release", "-p", "acta-mcp", "--bin", "acta-mcp-server"]
            res = subprocess.run(cmd, cwd=str(repo_root))
            if res.returncode == 0:
                target_bin = repo_root / "target" / "release" / "acta-mcp-server"
                if is_executable(target_bin):
                    return str(target_bin)

    raise FileNotFoundError(
        "Could not locate or download `acta-mcp-server` executable.\n"
        "Options to resolve:\n"
        "1. Set ACTA_MCP_BINARY environment variable pointing to the precompiled binary.\n"
        "2. Ensure acta-mcp-server is available in your system PATH.\n"
        "3. Set ACTA_MCP_ALLOW_REPO_BUILD=1 to allow building from source via Cargo in developer repositories."
    )

def main():
    try:
        binary_path = find_mcp_binary()
    except Exception as e:
        sys.stderr.write(f"Error: {e}\n")
        sys.exit(1)

    args = [binary_path] + sys.argv[1:]

    # Use os.execv on POSIX for zero-overhead process replacement
    if hasattr(os, "execv"):
        try:
            os.execv(binary_path, args)
        except OSError:
            pass

    # Fallback to subprocess for platforms/environments where execv fails or isn't available
    res = subprocess.run(args)
    sys.exit(res.returncode)

if __name__ == "__main__":
    main()
