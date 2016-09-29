### dpkg-deb-rst
> :cyclone: dpkg-deb implementation in Rust.

[![Build Status](https://travis-ci.org/stpettersens/dpkg-deb-rst.png?branch=master)](https://travis-ci.org/stpettersens/dpkg-deb-rst)
<!--[![Build status](https://ci.appveyor.com/api/projects/status/cromct4qxt10o5yb?svg=true)](https://ci.appveyor.com/project/stpettersens/dpkg-deb-rst)
-->
<!-- TODO -->

##### Usage:

```
Usage: dpkg-deb-rst [<option> ...] <command>

Standard commands:
  -b|--build <directory> [<deb>]  Build an archive.
  -c|--contents <deb>             List contents.
  -I|--info <deb>                 Show info to stdout.

Extended commands:
  -s|--stage <pkg.json>           Stage package structure from JSON file.
  -b|--build <pkg.json>  [<deb>]  Build an archive from JSON file.
