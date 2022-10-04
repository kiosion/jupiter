# Maintainer: Kiosion <hi@kio.dev>

pkgname=jupiter-git
pkgver=0.1.0
_pkgver=0.1.0
pkgrel=1
pkgdesc='Simple, markdown-based note application.'
arch=('x86_64')
url='https://github.com/kiosion/jupiter'
license=('MIT')
depends=('gcc-libs')
makedepends=('git' 'cargo')
provides=("${pkgname%-git}")
source=("git+https://github.com/kiosion/jupiter.git#tag=v${_pkgver}")
sha256sums=('SKIP')

pkgver() {
  cd "$srcdir/${pkgname%-git}"

  git describe --long --tags | sed 's/^v//;s/\([^-]*-g\)/r\1/;s/-/./g'
}

build() {
  cd "$srcdir/${pkgname%-git}"

  make release
}

package() {
  install -Dm755 "$srcdir/${pkgname%-git}/target/release/jupiter" "$pkgdir/usr/bin/jupiter"

  install -Dm644 "README.md" "$pkgdir/usr/share/doc/${pkgname}/README.md"
}
