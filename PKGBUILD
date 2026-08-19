# Maintainer: Shihab Mahamud <mt.shihab26@gmail.com>
pkgname=orivo
pkgver=0.3.1
pkgrel=1
pkgdesc="A terminal-based (TUI) Todos + Pomodoro timer written in Rust"
arch=('x86_64' 'aarch64')
url="https://github.com/mt-shihab26/orivo"
license=('MIT')
depends=('sqlite' 'alsa-lib' 'gcc-libs')
options=('!strip' '!debug')

_base="https://github.com/mt-shihab26/orivo/releases/download/v$pkgver"

# Prebuilt binaries from the GitHub release
source_x86_64=("orivo-v$pkgver-linux-x86_64::$_base/orivo-v$pkgver-linux-x86_64")
source_aarch64=("orivo-v$pkgver-linux-aarch64::$_base/orivo-v$pkgver-linux-aarch64")

# fetches — no compiling. Run `updpkgsums` after bumping pkgver.
sha256sums_x86_64=('5f609c5acb46ccada8d641952b379029469125077aca67caa7500b6dd7f2ed40')
sha256sums_aarch64=('fddfe4bbe59fea0b6039fd1f521d3047b01bd892cead8f7933c558fdf4e1f9eb')

source=("LICENSE::https://raw.githubusercontent.com/mt-shihab26/orivo/v$pkgver/LICENSE"
        "orivo-omarchy.desktop::$_base/orivo-omarchy.desktop"
        "orivo.svg::$_base/orivo.svg")

sha256sums=('4a3d287c9308636fb243c9785a195a800c577b49f44ef658ff4c61143be313a1'
            'fc4896283c369009fa10a5d7d6f43e7bb406b7ec471db7cff509669ba61ff1d0'
            '27174eda039a68a18fa6e8df9981b44e381fbd85d88d2c6c62505766eff60671')

package() {
    install -Dm0755 "orivo-v$pkgver-linux-$CARCH" "$pkgdir/usr/bin/orivo"
    install -Dm0644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    install -Dm0644 orivo.svg "$pkgdir/usr/share/icons/hicolor/scalable/apps/orivo.svg"
    install -Dm0644 orivo-omarchy.desktop "$pkgdir/usr/share/applications/orivo.desktop"
}
