extern crate config;

use config::Config;

pub struct Node {
    name: String,
    alive: bool,
    me: bool,
    size: u64,
    free: u64
}

pub struct Network {
    dummy: u16
}

impl Network {
    pub fn new<'a>(config: &Config ) -> Result<Network, String> {
         let h: String = config.get("hostname").unwrap();
         let dummy = 1;
         Ok( Network{dummy} )
    }
}

