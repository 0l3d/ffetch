# Maintainer: 0l3d

pkgname=ffetch-bin
pkgver=0.2.21
pkgrel=1
pkgdesc='CLI tool to fetch system info.'
url='https://github.com/0l3d/ffetch'
license=('GPL3')
arch=('x86_64')
depends=('pciutils' 'curl' 'coreutils')
source=("$url/releases/download/v$pkgver/ffetch-v$pkgver-linux-x86_64.tar.gz")
sha256sums=('4f62bd68dbc26e0279b97709ffa954faa9fca2c509d502750182ffd1bdf24ac5')

package() {
  tar -xzf "$srcdir/ffetch-v$pkgver-linux-x86_64.tar.gz" -C "$srcdir"
  install -Dm755 "$srcdir/ffetch" "$pkgdir/usr/bin/ffetch"
  install -Dm644 "$srcdir/LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
  install -Dm644 "$srcdir/README.md" "$pkgdir/usr/share/doc/$pkgname/README.md"
  install -Dm644 "$srcdir/CHANGELOG.md" "$pkgdir/usr/share/doc/$pkgname/CHANGELOG.md"
}
