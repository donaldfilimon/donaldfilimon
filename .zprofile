# Homebrew base
eval "$(/opt/homebrew/bin/brew shellenv zsh)"

# Keep PATH entries unique across login shell setup.
typeset -U path PATH

# Homebrew opt paths
export PATH="/opt/homebrew/opt/llvm/bin:$PATH"
export PATH="/opt/homebrew/opt/rustup/bin:$PATH"
export PATH="/opt/homebrew/opt/swift/bin:$PATH"
export PATH="/opt/homebrew/opt/cython/bin:$PATH"

# Cargo / Rust
if [ -f "$HOME/.cargo/env" ]; then
  . "$HOME/.cargo/env"
fi

# pipx and local bin
export PATH="$PATH:$HOME/.local/bin"
if [ -f "$HOME/.local/bin/env" ]; then
  . "$HOME/.local/bin/env"
fi

# ZVM (Zig Version Manager)
export ZVM_INSTALL="$HOME/.zvm/self"
export PATH="$HOME/.zvm/bin:$ZVM_INSTALL:$PATH"

# Bun
export BUN_INSTALL="$HOME/.bun"
export PATH="$BUN_INSTALL/bin:$PATH"

# Swiftly
if [ -f "$HOME/.swiftly/env.sh" ]; then
  . "$HOME/.swiftly/env.sh"
fi

# LM Studio CLI
export PATH="$PATH:$HOME/.lmstudio/bin"
