> [!CAUTION]
> before you install, **you NEED to put `-condebug` in tf2's launch options**
>
> **debian/ubuntu, nixos, and windows are all untested and might not work**

make sure you [built from source](./building.md) first
(not applicable to nixos)

more detailed usage instructions can be found in the
[usage](./usage.md) section

### universal
```sh
sudo ./scripts/install.sh
```

### windows
```sh
cargo install cargo-packager
cargo packager --release --formats nsis
```
run the generated installer in `./dist`

### fedora
```sh
cargo install cargo-generate-rpm
cargo generate-rpm
sudo dnf install target/generate-rpm/*.rpm
systemctl enable --user --now tf2-rpc
```

### debian/ubuntu
```sh
cargo install cargo-deb
cargo deb
sudo dpkg -i target/debian/*.deb
systemctl enable --user --now tf2-rpc
```

### arch
```sh
makepkg -si
systemctl --user enable --now tf2-rpc
```

### nixos
```nix
inputs = {
	tf2-rpc.url = "git+https://codeberg.org/paige/tf2-rpc";
};
```
```nix
environment.systemPackages = [
	inputs.tf2-rpc.packages.${pkgs.stdenv.hostPlatform.system}.default
];
```
