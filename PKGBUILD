pkgname=gh-cal
pkgver=1
pkgrel=1

pkgdesc="View your github contribution calander in unicode."
arch=('any')
url='https://github.com/mrshmllow/gh-cal'

license=('MIT')

source=('https://github.com/mrshmllow/gh-cal/gh-cal.tar.xz')
sha256sums=('6440a60f947b3fa4326a5111cb799eb82bb942818c0a395c04229e45f97b336b')

package() {
    install -D --mode 755 'gh-cal' --target-directory "$pkgdir/usr/bin/gh-cal"
}
