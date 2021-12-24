#[macro_use]
extern crate log;

use env_logger::Env;

mod client;


fn main() {
    let env = Env::default()
        .filter_or("FRIGATE_LOG_LEVEL", "trace")
        .write_style_or("FRIGATE_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    client::start();
}
