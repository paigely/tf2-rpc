### paige/tf2-rpc
Discord RPC for Team Fortress 2

### why
vesktop wasn't picking up TF2 as my presence and that annoyed me,
also existing solutions for this sucked and/or didn't even work

### building
see [docs/building.md](./docs/building.md)

```sh
git clone https://codeberg.org/paige/tf2-rpc.git && cd tf2-rpc
direnv allow
cargo build --release
```

### installing
see [docs/installing.md](./docs/installing.md)

```sh
sudo ./scripts/install.sh
```

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
