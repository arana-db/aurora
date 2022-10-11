#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::type_complexity)]
#![allow(clippy::field_reassign_with_default)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]

#[macro_use]
extern crate slog;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate log;
#[macro_use]
extern crate anyhow;

#[cfg(test)]
mod test_aurora {
    use aurora_util::prelude::*;

    use super::*;

    fn init() {
        pretty_env_logger::init_timed();
    }

    #[test]
    fn test_hello() {
        init();
        let s = format!("{} AURORA {}!", "hello", Consts::VERSION);
        info!(">>>> {}", &s);
        assert!(!s.is_empty());
    }
}
