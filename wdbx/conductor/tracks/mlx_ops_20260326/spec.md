# Specification: Implement basic MLX tensor operations and inference benchmark

## Overview
This track aims to establish the foundational machine learning capabilities of `wdbx` by implementing core tensor operations using `mlx-rs` and creating a simple benchmark to verify performance on Apple Silicon.

## Requirements
- Initialize MLX device and stream.
- Implement basic tensor creation and manipulation (add, multiply, matmul).
- Implement a simple linear layer or MLP forward pass.
- Create a CLI command to run an inference benchmark.
- Ensure all operations are asynchronous where applicable using Tokio.

## Technical Constraints
- Use `mlx-rs` for all ML operations.
- Target Apple Silicon (Metal) via MLX.
- CLI must follow `clap` conventions.