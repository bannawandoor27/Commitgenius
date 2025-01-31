# Commitgenius 🤖

A Rust-based CLI tool that uses local LLMs (via Ollama) to generate conventional commit messages from your git diffs.

## Features

- 🚀 Uses local LLM inference through Ollama
- 📝 Generates conventional commit messages
- 🔄 Auto-starts Ollama if not running
- 🎯 Supports multiple Ollama models
- 🛠 Easy to use CLI interface

## Prerequisites

- Rust
- Git
- [Ollama](https://ollama.ai)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/bannawandoor27/commitgenius.git
cd commitgenius
```

2. Build and install:
```bash
cargo build --release
ln -s "$(pwd)/target/release/commitgenius" ~/bin/cmgenius
```

## Usage

1. Stage your changes:
```bash
git add .
```

2. Generate commit message and commit:
```bash
# Using default model (qwen2.5:7b)
cmgenius

# Using a specific model
cmgenius --model phi4
```

## Supported Models

Any model available in your Ollama installation can be used. Some recommended models:
- qwen2.5:7b
- phi4
- llama3.2
- codellama

## License

MIT License
