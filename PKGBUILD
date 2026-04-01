# Maintainer: paige <paigely@tuta.io>
pkgname=tf2-rpc-git
pkgver=1.0.0
pkgrel=1
pkgdesc="Discord RPC for Team Fortress 2"
arch=('x86_64')
url="https://codeberg.org/paige/tf2-rpc"
license=('GPL-3.0-only')
depends=('gcc-libs')
makedepends=('rust' 'cargo' 'git')
source=("${pkgname}::git+${url}.git"
        "tf2-rpc.service")
sha256sums=('SKIP'
            'SKIP')

build() {
  cd "$srcdir/$pkgname"
  cargo build --release --locked
}

package() {
  cd "$srcdir/$pkgname"
  install -Dm755 "target/release/tf2-rpc" "$pkgdir/usr/bin/tf2-rpc"
  install -Dm644 "$srcdir/tf2-rpc.service" "$pkgdir/usr/lib/systemd/user/tf2-rpc.service"
}
