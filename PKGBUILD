pkgname=flas-timer
pkgver=1.0.0
pkgrel=1
pkgdesc="Timer application"
arch=('x86_64')
url="https://github.com/FLAS-Forum/timer"
license=('MIT')
depends=('glibc')
makedepends=('cargo' 'rust')
source=("git+https://github.com/FLAS-Forum/timer.git")
sha256sums=('SKIP')

build() {
  cd "$srcdir/timer"   # Passe an, falls der Ordner anders heißt
  cargo build --release
}

package() {
  cd "$srcdir/timer"
  install -Dm755 "target/release/flas-timer" "$pkgdir/usr/bin/flas-timer"
}
