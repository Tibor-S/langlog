use crate::jamo::{FinalJamo, InitialJamo, Jamo, MedialJamo};

mod jamo;

fn main() {
    pretty_env_logger::init();
    log::debug!("{}", Jamo::D);
    log::debug!("{}", InitialJamo::Ch);
    log::debug!("{}{}", InitialJamo::Silent, MedialJamo::Oe);
    log::debug!("{}{}", InitialJamo::Silent, FinalJamo::Lg);
}
