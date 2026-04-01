### usage
after installing, all you need to do is make
tf2-rpc run. it will handle everything else
for you.

unlike other tf2 rich presence projects, this
one simply checks what programs are running to
find your tf2 installation, so you don't need
to specify what drive it's on or anything.

### linux
after you built and installed from source, you
can enable the systemd service:

```sh
systemctl enable --now --user tf2-rpc
# see if it's working, if you want
systemctl status --user tf2-rpc
```

### windows
after you built from source, find the installer
inside `.\dist\` and run it. then, you should
be able to run `tf2-rpc` in a terminal, or from
the start menu. you can setup windows to
automatically open `tf2-rpc` on boot
