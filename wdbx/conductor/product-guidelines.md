# Product Guidelines

## Core UX Principles
- **Fast & Responsive:** Emphasize low latency and quick feedback, especially for large computational or ML tasks. Provide immediate feedback even when underlying jobs take time.
- **Rich Outputs:** Use colors, progress bars, and formatted text to ensure high readability and a pleasant developer experience.
- **Scriptable:** Offer JSON or structured text output modes to support easy piping into other Unix tools or CI/CD pipelines.

## Error Handling
- **Contextual Output:** Show concise, user-actionable error messages by default to avoid overwhelming the user. Allow detailed stack traces and debugging context via a verbose (`-v` or `--verbose`) flag.

## Brand & Voice
- **Friendly & Helpful:** Use a warm tone and incorporate well-placed emojis. When errors occur, try to guide the user towards a solution and explain clearly why an action failed. Avoid overly terse or obscure error codes.