# 🚀 Commitgenius

A powerful CLI tool that generates conventional commit messages using local LLMs via Ollama. Say goodbye to writing commit messages manually!

[![Crates.io](https://img.shields.io/crates/v/commitgenius.svg)](https://crates.io/crates/commitgenius)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## ✨ Features

- 🤖 Generates smart, conventional commit messages using local LLMs
- 🔄 Automatically manages Ollama service
- 🎯 Supports multiple Ollama models
- 🚀 Fast and efficient
- 💻 Works offline with local models
- 📝 Automatic git staging with smart file selection

## 📦 Installation

### Using Homebrew (macOS)

```bash
brew tap bannawandoor27/commitgenius
brew install commitgenius
```

### Using APT (Debian/Ubuntu)

```bash
# Add our repository
echo "deb [trusted=yes] https://apt.commitgenius.dev/ /" | sudo tee /etc/apt/sources.list.d/commitgenius.list

# Update package list and install
sudo apt update
sudo apt install commitgenius
```

### Using Cargo (Recommended)

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

Available models:
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
