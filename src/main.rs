use crate::{
    hangul::Hangul,
    hangul_parser::HangulParser,
    jamo::{FinalJamo, InitialJamo, Jamo, MedialJamo},
    syllable::Syllable,
};

mod ext;
mod hangul;
mod hangul_parser;
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
    syllable.push(Jamo::G).unwrap();
    log::debug!("Syllable: {}", syllable);
    syllable.push(Jamo::A).unwrap();
    log::debug!("Syllable: {}", syllable);
    syllable.push(Jamo::G).unwrap();
    log::debug!("Syllable: {}", syllable);

    let mut syllable = Syllable::default();
    log::debug!("Syllable: {}", syllable);
    syllable.push(Jamo::A).unwrap();
    log::debug!("Syllable: {}", syllable);
    syllable.push(Jamo::O).unwrap();
    log::debug!("Syllable: {}", syllable);
    syllable.push(Jamo::G).unwrap();
    log::debug!("Syllable: {}", syllable);
    syllable.push(Jamo::S).unwrap();

    log::debug!("Syllable: {}", syllable);
    let mut syllable = Syllable::default();
    log::debug!("Syllable: {}", syllable);
    syllable.push(Jamo::A).unwrap();
    log::debug!("Syllable: {}", syllable);
    syllable.push(Jamo::O).unwrap();
    log::debug!("Syllable: {}", syllable);
    let overflow = syllable.push(Jamo::O).unwrap().unwrap();
    log::debug!("Syllables: {}{}", syllable, overflow);

    log::debug!("possible Jamo:");
    for j in syllable.possible() {
        log::debug!("\t{j}");
    }
    syllable.push(Jamo::R).unwrap();
    log::debug!("Syllable: {}", syllable);
    log::debug!("Possible Jamo:");
    for j in syllable.possible() {
        log::debug!("\t{j}");
    }

    let mut str = Hangul::default();
    str.push_back(Jamo::G).unwrap();
    log::debug!("Hangul: {str}");
    str.push_back(Jamo::Ae).unwrap();
    log::debug!("Hangul: {str}");
    str.push_back(Jamo::Ya).unwrap();
    log::debug!("Hangul: {str}");
    str.push_back(Jamo::B).unwrap();
    log::debug!("Hangul: {str}");
    str.push_back(Jamo::Gg).unwrap();
    log::debug!("Hangul: {str}");
    str.push_back(Jamo::I).unwrap();
    log::debug!("Hangul: {str}");
    str.push_back(Jamo::O).unwrap();
    log::debug!("Hangul: {str}");
    str.push_back(Jamo::M).unwrap();
    log::debug!("Hangul: {str}");
    str.push_back(Jamo::D).unwrap();
    log::debug!("Hangul: {str}");
    str.push_back(Jamo::Ae).unwrap();
    log::debug!("Hangul: {str}");
    str.push_back(Jamo::R).unwrap();
    log::debug!("Hangul: {str}");
    str.push_back(Jamo::S).unwrap();
    log::debug!("Hangul: {str}");
    let popped = str.pop_back().unwrap();
    log::debug!("Hangul: {str}\tpopped:{popped}");
    let popped = str.pop_back().unwrap();
    log::debug!("Hangul: {str}\tpopped:{popped}");
    let popped = str.pop_back().unwrap();
    log::debug!("Hangul: {str}\tpopped:{popped}");
    let popped = str.pop_back().unwrap();
    log::debug!("Hangul: {str}\tpopped:{popped}");
    let popped = str.pop_back().unwrap();
    log::debug!("Hangul: {str}\tpopped:{popped}");
    let popped = str.pop_back().unwrap();
    log::debug!("Hangul: {str}\tpopped:{popped}");

    let js = vec![
        Jamo::G,
        Jamo::Ae,
        Jamo::Ya,
        Jamo::B,
        Jamo::Gg,
        Jamo::I,
        Jamo::O,
        Jamo::M,
        Jamo::D,
        Jamo::Ae,
        Jamo::R,
        Jamo::S,
    ];
    let str = Hangul::try_from(js).unwrap();
    log::debug!("Hangul: {str}");

    let mut hangul = Hangul::default();
    let parser = HangulParser::new();
    let rr = parser.parse_token(&mut hangul, "ggwae").unwrap();
    log::debug!("{hangul}\t{rr}");
    let rr = parser.parse_token(&mut hangul, rr).unwrap();
    log::debug!("{hangul}\t{rr}");

    log::debug!("With prefix \"e\"");
    for j in parser.with_prefix("e") {
        log::debug!("\t{j}")
    }

    let mut hangul = Hangul::default();
    let rr = parser.parse(&mut hangul, "gyaeonhrui").unwrap();
    log::debug!("{hangul}\t{rr}");
    let mut hangul = Hangul::default();
    let rr = parser.parse(&mut hangul, "gyaed-danjja").unwrap();
    log::debug!("{hangul}\t{rr}");
}
