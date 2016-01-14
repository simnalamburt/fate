[![fate-i][]][fate-a] [![travis-i][]][travis-a]
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

### Requirements
* **[Rust][]**
* Developement headers for XFree86 video mode extension<br>
  (`libXxf86vm-dev` on apt, `libXxf86vm-devel` on rpm, `libxxf86vm` on pacman)

[fate-i]: http://cafefiles.naver.net/20120221_101/potechoi_1329792450679I3P6X_PNG/%B7%CE%B0%ED_20120221.png
[fate-a]: http://cafe.naver.com/ufw
[travis-i]: https://travis-ci.org/simnalamburt/fate.svg?branch=master
[travis-a]: https://travis-ci.org/simnalamburt/fate
[Rust]: http://rust-lang.org
