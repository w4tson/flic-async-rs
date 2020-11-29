# Flicfun 

[![Master](https://github.com/w4tson/flic-async-rs/workflows/Rust/badge.svg)](https://github.com/w4tson/flic-async-rs/actions)

Tinkering with Rust + Raspberry Pi + Flic + Hue


build for the pi with `cross build --target armv7-unknown-linux-gnueabihf`

starting flicd on the server 

`./flicd -f flic.sqlite3 -d -l /var/log/flicd/flicd.log -s 0.0.0.0`
