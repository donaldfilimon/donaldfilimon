export PATH="/opt/homebrew/opt/rustup/bin:$PATH"
export PATH="/opt/homebrew/opt/swift/bin:$PATH"
export PATH="/opt/homebrew/opt/llvm/bin:$PATH"

# === ABI & Lilex Development ===
alias abi='cd ~/abi'
alias lilex='cd ~/lilex'
alias abt='cd ~/abi && zig build test --summary all'
alias abf='cd ~/abi && zig build feature-tests --summary all'
alias abc='cd ~/abi && zig build full-check'
alias abl='cd ~/abi && zig build lint'
alias abx='cd ~/abi && zig build fix'
alias lbt='cd ~/lilex && cargo test --no-default-features'
alias lbc='cd ~/lilex && cargo clippy --all-targets --no-default-features'
alias lbf='cd ~/lilex && cargo fmt --all'
alias lbb='cd ~/lilex && cargo build --no-default-features'
