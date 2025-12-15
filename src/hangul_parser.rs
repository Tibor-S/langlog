use crate::{
    ext::Tree,
    hangul::{Hangul, HangulResult},
    jamo::Jamo,
};

pub struct HangulParser(Tree<char, Jamo>);
impl HangulParser {
    pub fn new() -> Self {
        let mut tree = Tree::<char, Jamo>::default();
        tree.insert_str("g", Jamo::G);
        tree.insert_str("gg", Jamo::Gg);
        tree.insert_str("gs", Jamo::Gs);
        tree.insert_str("n", Jamo::N);
        tree.insert_str("nc", Jamo::Nc);
        tree.insert_str("nch", Jamo::Nch);
        tree.insert_str("d", Jamo::D);
        tree.insert_str("dd", Jamo::Dd);
        tree.insert_str("r", Jamo::R);
        tree.insert_str("lg", Jamo::Lg);
        tree.insert_str("lm", Jamo::Lm);
        tree.insert_str("lb", Jamo::Lb);
        tree.insert_str("ls", Jamo::Ls);
        tree.insert_str("lt", Jamo::Lt);
        tree.insert_str("lph", Jamo::Lph);
        tree.insert_str("lh", Jamo::Lh);
        tree.insert_str("m", Jamo::M);
        tree.insert_str("b", Jamo::B);
        tree.insert_str("bb", Jamo::Bb);
        tree.insert_str("bs", Jamo::Bs);
        tree.insert_str("s", Jamo::S);
        tree.insert_str("ss", Jamo::Ss);
        tree.insert_str("ng", Jamo::Silent);
        tree.insert_str("j", Jamo::J);
        tree.insert_str("jj", Jamo::Jj);
        tree.insert_str("ch", Jamo::Ch);
        tree.insert_str("k", Jamo::K);
        tree.insert_str("t", Jamo::T);
        tree.insert_str("p", Jamo::P);
        tree.insert_str("h", Jamo::H);
        tree.insert_str("a", Jamo::A);
        tree.insert_str("ae", Jamo::Ae);
        tree.insert_str("ya", Jamo::Ya);
        tree.insert_str("yae", Jamo::Yae);
        tree.insert_str("eo", Jamo::Eo);
        tree.insert_str("e", Jamo::E);
        tree.insert_str("yeo", Jamo::Yeo);
        tree.insert_str("ye", Jamo::Ye);
        tree.insert_str("o", Jamo::O);
        tree.insert_str("wa", Jamo::Wa);
        tree.insert_str("wae", Jamo::Wae);
        tree.insert_str("oe", Jamo::Oe);
        tree.insert_str("yo", Jamo::Yo);
        tree.insert_str("u", Jamo::U);
        tree.insert_str("wo", Jamo::Wo);
        tree.insert_str("we", Jamo::We);
        tree.insert_str("wi", Jamo::Wi);
        tree.insert_str("yu", Jamo::Yu);
        tree.insert_str("eu", Jamo::Eu);
        tree.insert_str("ui", Jamo::Ui);
        tree.insert_str("i", Jamo::I);
        Self(tree)
    }

    pub fn parse<'a>(
        &self,
        hangul: &mut Hangul,
        input: &'a str,
    ) -> HangulResult<&'a str> {
        let mut pre = "";
        let mut cur = input;
        while pre != cur {
            pre = cur;
            cur = self.parse_token(hangul, cur)?;
        }
        Ok(cur)
    }

    pub fn parse_token<'a>(
        &self,
        hangul: &mut Hangul,
        input: &'a str,
    ) -> HangulResult<&'a str> {
        let mut input = input;
        let mut is_break = false;
        while matches!(input.chars().nth(0).unwrap_or(' '), ' ' | '-') {
            is_break = true;
            if input.is_empty() {
                return Ok(input);
            }
            input = input.split_at(1).1;
        }
        for len in (1..=3).rev() {
            let end = input
                .char_indices()
                .nth(len)
                .map(|(i, _)| i)
                .unwrap_or(input.len());
            let (s, ret) = input.split_at(end);

            match self.0.get_str(s) {
                Some(j) => {
                    if is_break {
                        hangul.break_with(j)?;
                    } else {
                        hangul.push_back(j)?;
                    }
                    return Ok(ret);
                }
                None => continue,
            };
        }
        Ok(input)
    }

    pub fn with_prefix(&self, token: &str) -> Vec<Jamo> {
        self.0.with_prefix(token)
    }
}
