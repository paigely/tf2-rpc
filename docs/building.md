### clone the repo
```sh
git clone https://codeberg.org/paige/tf2-rpc.git && cd tf2-rpc
```

### grab dependencies
this will differ depending on what OS/distribution you're
using, to use direnv and nix (preferred, but optional):

```sh
direnv allow
```

for everything else, use your package manager to install
rust and cargo, eg. for fedora:

```sh
sudo dnf install cargo rust
```

### build
```sh
cargo build --release
```
