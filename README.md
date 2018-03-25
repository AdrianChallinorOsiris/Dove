# Dove

DOVE is the Distributed Osiris Vertex Edge database:
 * __Distributed__ - because it runs seemlessly over a network od different nodes, each with different characteristics.

 * __Osiris__ - Because we developed it!

* __Vertex / Edge__ - Beause it uses directed graph constructs under the covers.

## Status
This is very much in alpha development at present. It is only on github so we have a code store.

It is not working for any useful operation (apart from trashing a disk!)

## WANRNING
You are advised **NOT** to use DOVE at this time. Don't play. Don't tray and be smart. It won't end well and there will be tears before bedtime. Trust me on this.

*DOVE* uses raw disk. It detects if it has formatted the disk yet, and if it hadn't it will write all over your disk. It expects to use the *whole* disk. So it can quite easily write all over your partition table.

Don't - please just don't - start *DOVE* with `--disk /sda` if that is where your OS is located. That will be the last you see of your operating system, disk, files, etc.

> OSIRIS will take no responsibility if you ignore this warning. We won't be
> sorry, but we maj just laugh. Outloud.

**YOU HAVE BEEN WARNED**

## Concepts
The idea is that this will run in parallel over a large number of nodes. Data (vertices) are stored across the nodes according to a storage protocol. Eventually this will support fault tolerant shards, but not in this iteration.

Each node provides one disk for DOVE to work with. This is a raw disk with no file system on it. DOVE will take over the disk and handle all IO.

DOVE nodes are autonomous. There is no concept of a master node. They discover each other and work in harmony. The cluster can grow or shrink without loss of functionality.

## Build

*DOVE* is a RUST project. The TOML has all the dependencies in use. It can be built with `cargo build`.

## Configuration

*DOVE* is controlled by a configuration file. By default this called dove.toml. It is written in [TOML](https://github.com/toml-lang/toml) format, which any Rust programmer will know only too well.

## Command line parameters
Run DOVE with the --help to get the help text

```
$ cargo run -- --help
DOVE-DB 0.1.0
Adrian Challinor <adrian.challinor@osiris.co.uk>
Distributed Osiris Vertex Edge database

USAGE:
    dove [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -i, --init       Forces the disk to be initialized
    -V, --version    Prints version information
    -v, --verbose    Sets the level of verbosity

OPTIONS:
    -c, --config <FILE>    The config file [default: dove.toml]
    -d, --disk <disk>      Name of a disk to manage
```

Note the -v flag. This can be used multiple time:
0. Warnings, Errors and Info messages
1. Debug messages
2. Trace messages
3. Deeper trace it can get wordy!

This may change: it is planned to bump info to level 1, and all the others go up a level.







##  License

This is released under the [GNU GPL3 license](https://choosealicense.com/licenses/gpl-3.0/).

## Contact
Adrian Challinor
adrian dot challinor at osiris dot co dot uk