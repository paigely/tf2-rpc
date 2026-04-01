### paige/tf2-rpc
Discord RPC for Team Fortress 2

### why
vesktop wasn't picking up TF2 as my presence and that annoyed me,
also existing solutions for this sucked and/or didn't even work

### installing
**you NEED to put `-condebug` in tf2's launch options**

```sh
git clone git@codeberg.org:paige/tf2-rpc.git && cd tf2-rpc

# if you want to use direnv and nix, run the following
direnv allow

# otherwise, install cargo and other dependencies
# eg. `sudo dnf install rust cargo`

cargo build --release

mkdir -p ~/.local/bin/
cp target/release/tf2-rpc ~/.local/bin/tf2-rpc
```

### usage
**again, you NEED to put `-condebug` in tf2's launch options**

assuming ~/.local/bin is in your PATH, just run `tf2-rpc` in
a terminal

to automate this, you can make a systemd service if you want,
or if your distro/desktop environment has a feature for startup
apps/login items, you can register it there (eg. KDE autostart)

### how it works
this relies on source's `-condebug` launch option feature which
pipes the output of the developer console into `tf/console.log`

in a nutshell, this program literally just parses and watches
that file, and uses a discord rpc library to display what's going
on in game, based on the log. there is no things like hooking into
tf2 to see what's happening in game, therefore, it's completely
fine to use with VAC and will not trigger a ban, because it's just
using an engine feature as intended

since game info isn't directly printed into the console, we make
use of the connected server's IP in order to make an A2S info query
which provides things like the map name, server details etc

### what about other games?
i didn't write this to work with other games, however theoretically
you could adapt it to work with them. i only really play tf2 so i
have no interest in doing so, but if you can figure it out in a
clean way, feel free to contribute it!
