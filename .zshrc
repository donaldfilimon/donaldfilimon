# Completions
fpath+=~/.zfunc
autoload -Uz compinit
compinit
zstyle ':completion:*' menu select

# Keep PATH entries unique across interactive shell setup.
typeset -U path PATH

# Bun completions
if [ -s "$HOME/.bun/_bun" ]; then
  source "$HOME/.bun/_bun"
fi

# bun completions
[ -s "/Users/donaldfilimon/.bun/_bun" ] && source "/Users/donaldfilimon/.bun/_bun"

export PATH="$HOME/.local/bin:$PATH"

#" Added by LM Studio CLI tool (lms)
export PATH="$PATH:/Users/donaldfilimon/.lmstudio/bin"
export PATH="/opt/homebrew/opt/llvm/bin:$PATH"
