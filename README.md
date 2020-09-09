[![fate-i]][fate-a] [![travis-i]][travis-a]
========
Standalone Fate/Another Project
```sh
# client
git submodule init && git submodule update --depth=1
cd client; cargo run

# server
cd server; cargo run

# server tester
cd util; cargo run
```

![diagram]

### Requirements
* **[Rust]**
* Developement headers for XFree86 video mode extension \
  (`libXxf86vm-dev` on apt, `libXxf86vm-devel` on rpm, `libxxf86vm` on pacman)

[fate-i]: https://raw.githubusercontent.com/simnalamburt/i/master/fate/logo.png
[fate-a]: https://cafe.naver.com/ufw
[travis-i]: https://travis-ci.org/simnalamburt/fate.svg?branch=master
[travis-a]: https://travis-ci.org/simnalamburt/fate
[diagram]: https://raw.githubusercontent.com/simnalamburt/i/master/fate/diagram.png
[Rust]: https://rust-lang.org
