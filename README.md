# 🚀 Commitgenius

A powerful CLI tool that generates conventional commit messages and pull request descriptions using local LLMs via Ollama. Say goodbye to writing commit messages manually!

[![Crates.io](https://img.shields.io/crates/v/commitgenius.svg)](https://crates.io/crates/commitgenius)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## ✨ Features

- 🤖 Generates smart, conventional commit messages using local LLMs
- 📝 Creates pull requests with AI-generated descriptions
- 🔄 Automatically manages Ollama service
- 🎯 Supports multiple Ollama models
- 🚀 Fast and efficient
- 💻 Works offline with local models
- 📈 GitHub integration for PR creation
- 📝 Automatic git staging with smart file selection

## 📦 Installation

### Using Cargo (Recommended)

```bash
cargo install commitgenius
```

### Using Homebrew (macOS and Linux)

```bash
brew install bannawandoor27/commitgenius/commitgenius
```

### Using APT (Debian/Ubuntu)

```bash
cargo install commitgenius
```

### Building from Source

1. Clone the repository:
```bash
git clone https://github.com/bannawandoor27/Commitgenius.git
cd Commitgenius
```

2. Build and install:
```bash
cargo install --path .
```

### Prerequisites

- [Ollama](https://ollama.ai/) must be installed on your system
- Rust and Cargo (if building from source)
- Git installed and configured
- GitHub account and personal access token (for PR creation)

## 🚀 Usage

### Basic Usage

1. Stage and commit all changes:
```bash
cmgenius .
```

2. Stage and commit specific files:
```bash
cmgenius file1.rs file2.rs
```

3. Commit already staged changes:
```bash
cmgenius
```

### Advanced Usage

Use a different model:
```bash
# Stage and commit all changes with a specific model
cmgenius . --model codellama

# Stage and commit specific files with a specific model
cmgenius file1.rs file2.rs --model codellama

# Commit staged changes with a specific model
cmgenius --model codellama
```

### Create Pull Request

```bash
# Create a PR after committing (uses 'develop' as default base branch)
cmgenius . --pull-request

# Specify a different base branch
cmgenius . --pull-request --base main
```

### Environment Setup

For pull request creation functionality, set up the following environment variables:

```bash
# Create a .env file in the project root
GITHUB_TOKEN=your_github_token
GITHUB_REPO_OWNER=your_github_username
GITHUB_REPO_NAME=your_repository_name
```

### Available Models

- qwen2.5:7b (default)
- codellama
- llama2
- mistral
- neural-chat
- And any other model available in Ollama

### Examples

```bash
# Stage and commit all changes
cmgenius .

# Stage and commit specific files
cmgenius src/main.rs Cargo.toml

# Commit already staged changes with a specific model
cmgenius --model codellama

# Create a PR after committing
cmgenius . --pull-request

# View available options
cmgenius --help
```

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes using Commitgenius! (`cmgenius .`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Ollama](https://ollama.ai/) for providing the local LLM infrastructure
- The Rust community for amazing crates and tools

## 📧 Contact

Hasanul Banna - [@bannawandoor27](https://github.com/bannawandoor27)

Project Link: [https://github.com/bannawandoor27/Commitgenius](https://github.com/bannawandoor27/Commitgenius)
