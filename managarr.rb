# Documentation: https://docs.brew.sh/Formula-Cookbook
#                https://rubydoc.brew.sh/Formula
class Managarr < Formula
  desc "A fast and simple dashboard for Kubernetes written in Rust"
  homepage "https://github.com/Dark-Alex-17/managarr"
  if OS.mac? and Hardware::CPU.arm?
    url "https://github.com/Dark-Alex-17/managarr/releases/download/0.4.1/managarr-macos-arm64.tar.gz"
    sha256 "754623934d5f6ffb631f3cc41c4aed29ce6a6c323f7f4022befab806ff4cbe4c"
  elsif OS.mac? and Hardware::CPU.intel?
    url "https://github.com/Dark-Alex-17/managarr/releases/download/0.4.1/managarr-macos.tar.gz"
    sha256 "ca55feb77e2bc0e03a2344e75c67fbfa14b7d3dc8e80a0ccfb1e0fea263ce4ed"
  else
    url "https://github.com/Dark-Alex-17/managarr/releases/download/0.4.1/managarr-linux-musl.tar.gz"
    sha256 "45cf1e06daf56bfc055876b51bd837716b2ea788898b5accc2f6847af4275011"
  end
  version "0.4.1"
  license "MIT"

  def install
    bin.install "managarr"
    ohai "You're done!  Run with \"managarr\""
    ohai "For runtime flags, see \"managarr --help\""
  end
end
