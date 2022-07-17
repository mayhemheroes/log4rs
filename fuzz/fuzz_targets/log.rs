#![no_main]
use libfuzzer_sys::fuzz_target;

use log::{error, info, warn, debug};
use log4rs;

fuzz_target!(|data: &[u8]| {
    let _ = log4rs::init_file("log4rs.yaml", Default::default());
    match std::str::from_utf8(data) {
        Ok(s) => {
            info!("{}", s);
            error!("{}", s);
            warn!("{}", s);
            debug!("{}", s);
        },
        _ => {},
    }
});
