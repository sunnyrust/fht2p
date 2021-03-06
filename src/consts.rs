// crate's info
pub const NAME: &str = env!("CARGO_PKG_NAME");
// pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/fht2p.txt"));
pub const DESC: &str = env!("CARGO_PKG_DESCRIPTION");
pub const URL: &str = "https://github.com/biluohc/fht2p";

// config file
pub const CONFIG_FILE_NAME: &str = "fht2p.json";
pub const CONFIG_STRING: &str = include_str!("../config/fht2p.json");

pub const CHARSET: &str = "charset=utf-8";
pub const CONTENT_TYPE_HTML: &str = "text/html; charset=utf-8";
pub const COMPRESS_LEVEL: u32 = 5;

use std::cell::UnsafeCell;
use std::net::{Ipv4Addr, SocketAddr};

lazy_static! {
    //  10485760 = 10M
    pub static ref MAGIC_LIMIT: MutStatic<u64> = MutStatic::new(1024*1024*10);
    pub static ref SERVER_ADDR: MutStatic<SocketAddr> = MutStatic::new(SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8000));
}

/// it's unsync, but only modify it before read it concurrent, lazy to use `RwLock`..
pub struct MutStatic<T>(UnsafeCell<T>);

impl<T> MutStatic<T> {
    pub fn new(data: T) -> MutStatic<T> {
        MutStatic(UnsafeCell::new(data))
    }
    pub fn get(&self) -> &T {
        unsafe { self.0.get().as_ref().unwrap() }
    }
    // tls
    #[warn(clippy::mut_from_ref)]
    pub fn get_mut(&self) -> &mut T {
        unsafe { self.0.get().as_mut().unwrap() }
    }
    // modify it before read it concurrent
    pub fn set(&self, new: T) {
        unsafe { self.0.get().as_mut().map(|d| *d = new).unwrap() }
    }
}

unsafe impl<T> Send for MutStatic<T> {}
unsafe impl<T> Sync for MutStatic<T> {}

#[test]
fn consts_test() {
    assert_eq!(NAME, "fht2p");
}
