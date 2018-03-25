// Copyright ⓒ 2018 Adrian Challinor
// Licensed under the MIT license
// (see LICENSE or <http://opensource.org/licenses/MIT>) All files in the project carrying such
// notice may not be copied, modified, or distributed except according to those terms.

//! DOVE - The Distributed Osiris Vertex Edge database
//!
//! `DOVE` is a directed graph database, written entirely in RUST
//! and using a distributed computing model This means that graph
//! objects are distributed over the available nodes. Each object can be
//! stored on a single node, or replicated over multiple nodes.
//! Qhen querying `DOVE` it does not matter which node you connect to,
//! as all nodes maintain a link to all other runing nodes.
//!
//! `DOVE` is config light. This means that one config file can be used
//! for many nodes. Only where there are differnces does the config need
//! to change. In that instance, config changes can be passed via environment
//! variables by prefixing any of the config values with DOVE_.
//!

extern crate chrono;
#[macro_use]
extern crate clap;
extern crate config;
extern crate fern;
extern crate hostname;
extern crate libc;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

use clap::{App, Arg};
use fern::colors::{Color, ColoredLevelConfig};
use hostname::get_hostname;

mod disk;
mod header;
mod network;
mod freelist;
mod graphlist;
mod goblist;

fn main() {
    // Parse the command line
    let cmd_arguments = App::new("DOVE-DB")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Distributed Osiris Vertex Edge database")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .takes_value(true)
                .help("The config file")
                .default_value("dove.toml"),
        )
        .arg(
            Arg::with_name("disk")
                .short("d")
                .long("disk")
                .help("Name of a disk to manage")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("init")
                .short("i")
                .long("init")
                .help("Forces the disk to be initialized"),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Sets the level of verbosity")
                .multiple(true),
        )
        .get_matches();

    // Set up logging

    let verbosity: u64 = cmd_arguments.occurrences_of("verbose");
    let colors = ColoredLevelConfig::new()
        .debug(Color::Magenta)
        .info(Color::Green)
        .trace(Color::BrightBlue);

    let level = match verbosity {
        0 => log::LevelFilter::Info,
        1 => log::LevelFilter::Debug,
        _2_or_more => log::LevelFilter::Trace,
    };

    fern::Dispatch::new()
        .chain(std::io::stdout())
        .level(level)
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} {}\t [{}] {}",
                // This will color the log level only, not the whole line. Just a touch.
                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
                colors.color(record.level()),
                record.target(),
                message
            ))
        })
        .apply()
        .unwrap();

    info!("Starting up - Version {}", crate_version!());
    debug!("Verbosity level is {}", verbosity);

    // Get the host name
    let hostname = match get_hostname() {
        Some(name) => name,
        None => panic!("No node name"),
    };
    trace!("Node name: {} ", hostname);

    // Load the config file

    let cfile = cmd_arguments.value_of("config").unwrap();
    debug!("Reading config from: {}", cfile);

    let mut config = config::Config::default();
    config
        .set_default("hostname", hostname)
        .expect("Enable to add error");

    config
        // Merge the TOML config file
        .merge(config::File::with_name(cfile)).unwrap()
        // Add in settings from the environment (with a prefix of DOVE)
        .merge(config::Environment::with_prefix("DOVE")).unwrap();

    info!("Configuration loaded");
    let n: String = config.get("hostname").unwrap();
    info!("Confirming Host={}", n);

    // Connect to network

    // Start the disk

    let disk = disk::Disk::new(&config).expect("Opening disk failed");

    debug!("Disk open: {}", disk.get_name());

    // Anounce we are loaded
}
