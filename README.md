# groovy2kts
Converter for Groovy based Gradle files into Kotlin scripts

This is an experimental project for converting Groovy gradle files into Kotlin scripts.


## Hello, DWoC!

<img src="https://imgur.com/TLx5rsv.png" width="325px" />

If you're here for a [Delta Winter of Code](https://dwoc.io) project, check out the [Contributor's Guide](https://github.com/ranaaditya/groovy2kts/wiki/DWoC-Contributor's-Guide) in the Wiki.


## Setup
### Linux (WIP) 

- Install **rust**  version - 2018 or later using :
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- Fork this **repo** and clone into local
- cd path_to_clone_repo

```
cargo build

cargo run <path_to_build.gradle>
```

## Cli (WIP)

```
cargo install groovy2kts

groovy2kts <path_to_build.gradle>
```
