class Commitgenius < Formula
  desc "AI-powered CLI tool that generates conventional commit messages using local LLMs via Ollama"
  homepage "https://github.com/bannawandoor27/Commitgenius"
  version "0.2.1"
  
  depends_on "rust" => :build
  depends_on "ollama"

  def install
    system "cargo", "install", "--root", prefix, "commitgenius", "--version", "0.2.1"
  end

  test do
    system "#{bin}/commitgenius", "--version"
  end
end
