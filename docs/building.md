### building
first clone the repo, [install git](https://git-scm.com/install/)
if you haven't already
```sh
git clone https://codeberg.org/paige/tf2-rpc.git && cd tf2-rpc
```
alternatively you can just
[download an archive](https://codeberg.org/paige/bot/archive/master.zip)
of this repo

> [!NOTE]
> **for arch users,** you can skip the next steps and just run:
> ```sh
> makepkg -si
> systemctl --user enable --now tf2-rpc
> ```

### grab dependencies
this will differ depending on what OS/distribution you're
using, to use direnv and nix (preferred, but optional):

```sh
direnv allow
```

otherwise, [install rust](https://rust-lang.org/tools/install/)

### build
```sh
cargo build --release
```

### installing
see [docs/installing.md](./installing.md)
