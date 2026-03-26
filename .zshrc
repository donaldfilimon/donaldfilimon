# eval "$(~/.local/bin/agent shell-integration zsh)"
# Completions
fpath+=~/.zfunc
autoload -Uz compinit
compinit
zstyle ':completion:*' menu select

# Bun completions
if [ -s "$HOME/.bun/_bun" ]; then
  source "$HOME/.bun/_bun"
fi

# >>> conda initialize >>>
# !! Contents within this block are managed by 'conda init' !!
# __conda_setup="$('/Users/donaldfilimon/miniconda3/bin/conda' 'shell.zsh' 'hook' 2> /dev/null)"
# if [ $? -eq 0 ]; then
#     eval "$__conda_setup"
# else
#     if [ -f "/Users/donaldfilimon/miniconda3/etc/profile.d/conda.sh" ]; then
#         . "/Users/donaldfilimon/miniconda3/etc/profile.d/conda.sh"
#     else
#         export PATH="/Users/donaldfilimon/miniconda3/bin:$PATH"
#     fi
# fi
# unset __conda_setup
# <<< conda initialize <<<

# Zig + ZLS (managed by tools/zigup.sh --link)
export PATH="$HOME/.local/bin:$PATH"

# bun completions
[ -s "/Users/donaldfilimon/.bun/_bun" ] && source "/Users/donaldfilimon/.bun/_bun"
export PATH="/opt/homebrew/opt/llvm/bin:$PATH"
export PATH="$HOME/.zvm/bin:$PATH"
export PATH="$HOME/.zvm/self:$PATH"
export PATH="/opt/homebrew/opt/util-linux/bin:$PATH"
export PATH="/opt/homebrew/opt/util-linux/sbin:$PATH"
