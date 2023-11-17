pub mod config;

pub use crate::config as other_config;
fn main() {
    println!("{}", config::get_ascii());
}
