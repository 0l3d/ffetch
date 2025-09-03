# Maintainer: 0l3d

pkgname=ffetch-bin
pkgver=0.2.19
pkgrel=1
pkgdesc='CLI tool to fetch system info.'
url='https://github.com/0l3d/ffetch'
license=('GPL3')
arch=('x86_64')
depends=('pciutils' 'xorg-xprop' 'coreutils')
source=("$url/releases/download/v$pkgver/ffetch-v$pkgver-linux-x86_64.tar.gz")
sha256sums=('57bc7c6dbcf9eb7e16283d75c6dfe91e6e7405370bc68d0ae02e4631222ab0f3')

package() {
  tar -xzf "$srcdir/ffetch-v$pkgver-linux-x86_64.tar.gz" -C "$srcdir"
  install -Dm755 "$srcdir/ffetch" "$pkgdir/usr/bin/ffetch"
  install -Dm644 "$srcdir/LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
  install -Dm644 "$srcdir/README.md" "$pkgdir/usr/share/doc/$pkgname/README.md"
  install -Dm644 "$srcdir/CHANGELOG.md" "$pkgdir/usr/share/doc/$pkgname/CHANGELOG.md"
}
