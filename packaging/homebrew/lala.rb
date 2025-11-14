class Lala < Formula
  desc "Modern text editor with multi-format preview support"
  homepage "https://github.com/yourusername/lala"
  url "https://github.com/yourusername/lala/archive/v0.1.0.tar.gz"
  sha256 "REPLACE_WITH_ACTUAL_SHA256"  # Calculate: sha256sum v0.1.0.tar.gz
  license "MIT OR Apache-2.0"
  head "https://github.com/yourusername/lala.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    # Test version output
    assert_match "0.1.0", shell_output("#{bin}/lala --version")

    # Test help output
    assert_match "Modern text editor", shell_output("#{bin}/lala --help")

    # Test Markdown preview (with test file)
    (testpath/"test.md").write("# Test\nHello World")
    system "#{bin}/lala", "markdown", "test.md", "--no-color"
  end
end
