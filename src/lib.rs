#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

extern crate querystring;
extern crate serde;
extern crate serde_json;
extern crate tap;
extern crate reqwest;
extern crate futures;
extern crate env_logger;

mod instance;
pub use instance::{InstanceOp, InstanceItem, InstanceItems, InstanceStatus};

mod instancegroupmanager;
pub use instancegroupmanager::InstanceGroupManagerOp;


mod client;
pub use client::Client;
