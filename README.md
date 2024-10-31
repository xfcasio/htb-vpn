<h1 align='center'> htb-vpn </h1>

<p align="center">
  <h4 align="center">
    A little wrapper around openvpn that makes connecting to the HTB network fun! </br>
    Written in rust, however it currently depends on the following system binaries </br>
    { sudo, openvpn, killall } </br>
  </h4>
</p>

<p align="center">
    <img src="https://i.imgur.com/08NPiuI.png" />
</p>

<h2>Usage:</h2>
```
usage: htb-vpn [OPTION] [CONFIG_PATH]
 OPTIONS:
    -c, --connect          Connect to the HTB network with your configuration file.
    -d, --disconnect       Disconnect from the HTB network.
```

<h2>Compile & Install:</h2>

```
$ make            # requires rust and cargo to build
$ make install    # installs executable to /usr/local/bin
```
