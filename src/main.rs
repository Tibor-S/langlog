use crate::{
    jamo::{FinalJamo, InitialJamo, Jamo, MedialJamo},
    syllable::Syllable,
};

mod ext;
mod jamo;
mod syllable;

fn main() {
    pretty_env_logger::init();
    log::debug!("{}", Jamo::D);
    log::debug!("{}", InitialJamo::Ch);
    log::debug!("{}{}", InitialJamo::Silent, MedialJamo::Oe);
    log::debug!("{}{}", InitialJamo::Silent, FinalJamo::Lg);

    let mut syllable = Syllable::default();
    log::debug!("Syllable: {}", syllable);
    syllable = syllable.append(Jamo::G).unwrap();
    log::debug!("Syllable: {}", syllable);
    syllable = syllable.append(Jamo::A).unwrap();
    log::debug!("Syllable: {}", syllable);
    syllable = syllable.append(Jamo::G).unwrap();
    log::debug!("Syllable: {}", syllable);

    let mut syllable = Syllable::default();
    log::debug!("Syllable: {}", syllable);
    syllable = syllable.append(Jamo::A).unwrap();
    log::debug!("Syllable: {}", syllable);
    syllable = syllable.append(Jamo::O).unwrap();
    log::debug!("Syllable: {}", syllable);
    syllable = syllable.append(Jamo::G).unwrap();
    log::debug!("Syllable: {}", syllable);
    syllable = syllable.append(Jamo::S).unwrap();

    log::debug!("Syllable: {}", syllable);
    let mut syllable = Syllable::default();
    log::debug!("Syllable: {}", syllable);
    syllable = syllable.append(Jamo::A).unwrap();
    log::debug!("Syllable: {}", syllable);
    syllable = syllable.append(Jamo::O).unwrap();
    log::debug!("Syllable: {}", syllable);
    match syllable.append(Jamo::O) {
        Ok(v) => log::error!("Got {v:?}, should get an error!"),
        Err(e) => log::debug!("Got error {e} as expected"),
    };

    log::debug!("Actual possible Jamo:");
    for j in syllable.possible() {
        log::debug!("\t{j}");
    }
    syllable = syllable.append(Jamo::R).unwrap();
    log::debug!("Syllable: {}", syllable);
    log::debug!("Possible Jamo:");
    for j in syllable.possible() {
        log::debug!("\t{j}");
    }
}
