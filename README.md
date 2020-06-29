# feembox [![TravisCI build status](https://travis-ci.com/nabijaczleweli/feembox.svg?branch=trunk)](https://travis-ci.com/nabijaczleweli/feembox) [![AppVeyorCI build status](https://ci.appveyor.com/api/projects/status/hricwh1vkkj0wiwo?svg=true)](https://ci.appveyor.com/project/nabijaczleweli/feembox/branch/trunk) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE) [![Crates.io version](https://meritbadge.herokuapp.com/feembox)](https://crates.io/crates/feembox)
What if a feed, but it's a mailbox?

## [Documentation](https://rawcdn.githack.com/nabijaczleweli/feembox/doc/feembox/index.html)
## [Manpage](https://rawcdn.githack.com/nabijaczleweli/feembox/man/feembox.1.html)

### Installation

#### From Crates.io

Start by obtaining Rust from https://rustup.rs.
Afterwards, run

```sh
cargo install feembox
```

After the installation process finishes,
  move onto the [manpage](https://rawcdn.githack.com/nabijaczleweli/feembox/man/feembox.1.html) or example cronjob/subscription in `example/` to set up feed acquisition.

If you've encountered a problem during the installation or configuration, do not hesitate to open an issue [here](https://github.com/nabijaczleweli/feembox/issues/new).

#### From Debian repository

The following line in `/etc/apt/sources.list` or equivalent:
```apt
deb https://debian.nabijaczleweli.xyz stable main
```

With [my PGP key](https://nabijaczleweli.xyz/pgp.txt) (the two URLs are interchangeable):
```sh
wget -O- https://debian.nabijaczleweli.xyz/nabijaczleweli.gpg.key | sudo apt-key add
# or
sudo wget -O/etc/apt/trusted.gpg.d/nabijaczleweli.asc https://keybase.io/nabijaczleweli/pgp_keys.asc
```

Then the usual
```sh
sudo apt update
sudo apt install feembox
```
will work on x86_64 and i686.

See the [repository README](https://debian.nabijaczleweli.xyz/README) for more information.

#### From pre-built executables

Alternatively, have a glance over at the [releases page](https://github.com/nabijaczleweli/feembox/releases), which hosts Windows and Linux x86_64 binaries.

Installation should be a matter of downloading and unpacking them, and copying somewhere to your `$PATH`.

## Special thanks

To all who support further development on Patreon, in particular:

  * ThePhD
  * Embark Studios
