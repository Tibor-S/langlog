use crate::{
    ext::Tree,
    hangul::{Hangul, HangulResult},
    jamo::Jamo,
};

pub struct Parser(Tree<char, Jamo>);
impl Parser {
    pub fn new() -> Self {
        let mut tree = Tree::<char, Jamo>::default();
        Self::tree_insert(&mut tree, "g", Jamo::G);
        Self::tree_insert(&mut tree, "gg", Jamo::Gg);
        Self::tree_insert(&mut tree, "gs", Jamo::Gs);
        Self::tree_insert(&mut tree, "n", Jamo::N);
        Self::tree_insert(&mut tree, "nc", Jamo::Nc);
        Self::tree_insert(&mut tree, "nch", Jamo::Nch);
        Self::tree_insert(&mut tree, "d", Jamo::D);
        Self::tree_insert(&mut tree, "dd", Jamo::Dd);
        Self::tree_insert(&mut tree, "r", Jamo::R);
        Self::tree_insert(&mut tree, "lg", Jamo::Lg);
        Self::tree_insert(&mut tree, "lm", Jamo::Lm);
        Self::tree_insert(&mut tree, "lb", Jamo::Lb);
        Self::tree_insert(&mut tree, "ls", Jamo::Ls);
        Self::tree_insert(&mut tree, "lt", Jamo::Lt);
        Self::tree_insert(&mut tree, "lph", Jamo::Lph);
        Self::tree_insert(&mut tree, "lh", Jamo::Lh);
        Self::tree_insert(&mut tree, "m", Jamo::M);
        Self::tree_insert(&mut tree, "b", Jamo::B);
        Self::tree_insert(&mut tree, "bb", Jamo::Bb);
        Self::tree_insert(&mut tree, "bs", Jamo::Bs);
        Self::tree_insert(&mut tree, "s", Jamo::S);
        Self::tree_insert(&mut tree, "ss", Jamo::Ss);
        Self::tree_insert(&mut tree, "ng", Jamo::Silent);
        Self::tree_insert(&mut tree, "j", Jamo::J);
        Self::tree_insert(&mut tree, "jj", Jamo::Jj);
        Self::tree_insert(&mut tree, "ch", Jamo::Ch);
        Self::tree_insert(&mut tree, "k", Jamo::K);
        Self::tree_insert(&mut tree, "t", Jamo::T);
        Self::tree_insert(&mut tree, "p", Jamo::P);
        Self::tree_insert(&mut tree, "h", Jamo::H);
        Self::tree_insert(&mut tree, "a", Jamo::A);
        Self::tree_insert(&mut tree, "ae", Jamo::Ae);
        Self::tree_insert(&mut tree, "ya", Jamo::Ya);
        Self::tree_insert(&mut tree, "yae", Jamo::Yae);
        Self::tree_insert(&mut tree, "eo", Jamo::Eo);
        Self::tree_insert(&mut tree, "e", Jamo::E);
        Self::tree_insert(&mut tree, "yeo", Jamo::Yeo);
        Self::tree_insert(&mut tree, "ye", Jamo::Ye);
        Self::tree_insert(&mut tree, "o", Jamo::O);
        Self::tree_insert(&mut tree, "wa", Jamo::Wa);
        Self::tree_insert(&mut tree, "wae", Jamo::Wae);
        Self::tree_insert(&mut tree, "oe", Jamo::Oe);
        Self::tree_insert(&mut tree, "yo", Jamo::Yo);
        Self::tree_insert(&mut tree, "u", Jamo::U);
        Self::tree_insert(&mut tree, "wo", Jamo::Wo);
        Self::tree_insert(&mut tree, "we", Jamo::We);
        Self::tree_insert(&mut tree, "wi", Jamo::Wi);
        Self::tree_insert(&mut tree, "yu", Jamo::Yu);
        Self::tree_insert(&mut tree, "eu", Jamo::Eu);
        Self::tree_insert(&mut tree, "ui", Jamo::Ui);
        Self::tree_insert(&mut tree, "i", Jamo::I);
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

            match self.get(s) {
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
        let tree = &self.0;
        let temp = token.chars().collect::<Vec<_>>();
        let key = &mut temp.iter();
        let sub = match tree.get_tree(key) {
            Some(t) => t,
            None => return vec![],
        };

        sub.all().iter().map(|j| **j).collect()
    }

    fn get(&self, token: &str) -> Option<Jamo> {
        let tree = &self.0;
        let temp = token.chars().collect::<Vec<_>>();
        let key = &mut temp.iter();
        tree.get(key).cloned()
    }

    fn tree_insert(tree: &mut Tree<char, Jamo>, token: &str, value: Jamo) {
        let temp = token.chars().collect::<Vec<_>>();
        let key = &mut temp.iter();
        tree.insert(key, value);
    }
}
