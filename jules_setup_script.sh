#!/usr/bin/env bash
# =============================================================================
# jules_setup_script.sh — Jules VM Environment Bootstrap
# Runs once during VM snapshot creation. Installs native dependencies,
# caches heavy build artifacts, and injects peer agent context.
# =============================================================================

set -euo pipefail

echo ">>> [setup] Starting Jules VM bootstrap..."

# -----------------------------------------------------------------------------
# 1. System Dependencies
# -----------------------------------------------------------------------------
echo ">>> [setup] Installing system packages..."
sudo apt-get update -qq
sudo apt-get install -y --no-install-recommends \
    build-essential \
    curl \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    libwebkit2gtk-4.1-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

# -----------------------------------------------------------------------------
# 2. Rust Toolchain
# -----------------------------------------------------------------------------
echo ">>> [setup] Installing Rust toolchain..."
if ! command -v rustup &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
fi
source "$HOME/.cargo/env"
rustup update stable

# Pre-compile heavy Tauri crates to warm the build cache.
echo ">>> [setup] Pre-warming Rust build cache (tauri v2)..."
cd /repo/src-tauri && cargo check --quiet || true

# -----------------------------------------------------------------------------
# 3. Node / pnpm
# -----------------------------------------------------------------------------
echo ">>> [setup] Installing Node.js and pnpm..."
if ! command -v node &> /dev/null; then
    curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
    sudo apt-get install -y nodejs
fi
npm install -g pnpm bun --quiet

# Install project Node dependencies
echo ">>> [setup] Running pnpm install..."
cd /repo && pnpm install --frozen-lockfile

# -----------------------------------------------------------------------------
# 4. Peer Agent Context — GEMINI.md (Antigravity Architect Profile)
#
#    Jules operates alongside Antigravity, a Senior Staff Architect agent.
#    The content below is Antigravity's binding contract. Jules must read and
#    respect these constraints when interpreting task plans and review feedback.
#
#    ▼▼▼  PASTE THE FULL CONTENTS OF GEMINI.md BETWEEN THE HEREDOC MARKERS  ▼▼▼
# -----------------------------------------------------------------------------
echo ">>> [setup] Writing Antigravity peer context to /repo/GEMINI.md..."
cat > /repo/GEMINI.md << 'GEMINI_EOF'
# ============================================================
# INSERT GEMINI.md CONTENTS HERE
# Copy the full text of GEMINI.md and replace this comment.
# ============================================================
GEMINI_EOF

echo ">>> [setup] Antigravity context written. Verifying..."
if grep -q "INSERT GEMINI.md CONTENTS HERE" /repo/GEMINI.md; then
    echo "!!! [WARN] GEMINI.md placeholder was not replaced. Jules will lack peer context."
else
    echo ">>> [setup] GEMINI.md injection confirmed."
fi

# -----------------------------------------------------------------------------
# 5. Final Verification
# -----------------------------------------------------------------------------
echo ">>> [setup] Environment summary:"
echo "    Rust:  $(rustc --version)"
echo "    Node:  $(node --version)"
echo "    pnpm:  $(pnpm --version)"
echo "    bun:   $(bun --version)"
echo ""
echo ">>> [setup] Bootstrap complete. VM ready for Jules task execution."
