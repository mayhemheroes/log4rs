#![no_main]
use libfuzzer_sys::fuzz_target;

use log::{error, info, warn};
use log4rs;

fuzz_target!(|data: &[u8]| {
    match std::str::from_utf8(data) {
        Ok(s) => {
            log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
            info!("{}", s);
            error!("{}", s);
            warn!("{}", s);
        },
        _ => {},
    }
});
