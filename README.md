# doryani

A simple Path of Exile 2 filter list manager

## Features

- Downloads a set of good default filters (Currently only NeverSinks Litefilter)
- Updates all filters managed by doryani
- Allows downloading and updating custom filters (soon)

## Configuration

**This is currently not implemented**

For custom filters you need to create/edit the file: **~/.config/doryani/config.toml**

Like this:

```toml
[[filter]]
name = "My custom filter"
url = "https://raw.githubusercontent.com/atomicptr/doryani/refs/heads/master/my-custom-filter.filter"

[[filter]]
name = "My other custom filter"
url = "https://raw.githubusercontent.com/atomicptr/doryani/refs/heads/master/my-other-custom-filter.filter"
```

## Usage

Just run the tool and it will automatically do all the things

## License

GNU General Public License v3

![](https://www.gnu.org/graphics/gplv3-127x51.png)
