# Maintainer: tangowithfoxtrot <4d7orvu7@anonaddy.me>

pkgname=managarr-bin
pkgdesc="A TUI and CLI to manage your Servarrs"
url="https://github.com/Dark-Alex-17/managarr"
pkgrel=1
pkgver="$(curl -s "https://api.github.com/repos/Dark-Alex-17/managarr/releases/latest" | grep 'tag_name": ' | awk '{print $2}' | grep -oP '\d+\.\d+\.\d+')" # 0.5.1
arch=("aarch64" "x86_64")
license=("custom:Managarr License")
provides=("${pkgname%-bin}")
conflicts=("${pkgname%-bin}")
download_base_url=("$url/releases/download/v${pkgver//_/-}")
_get_license="$(mkdir -p src && curl -sL "https://raw.githubusercontent.com/Dark-Alex-17/managarr/refs/heads/main/LICENSE" -o src/LICENSE)"
source_aarch64=("$download_base_url/managarr-aarch64-musl.tar.gz")
source_x86_64=("$download_base_url/managarr-linux-musl.tar.gz")
sha256sums_aarch64=($(curl -sL "$download_base_url/managarr-aarch64-musl.sha256" | awk '{print $1}'))
sha256sums_x86_64=($(curl -sL "$download_base_url/managarr-linux-musl.sha256" | awk '{print $1}'))

package() {
    install -Dm755 managarr "${pkgdir}/usr/bin/managarr"
    install -Dm644 LICENSE "${pkgdir}/usr/share/licenses/${pkgname}/LICENSE"
}
