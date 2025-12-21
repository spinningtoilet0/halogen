use winnow::{Parser, Result, token::take_while};

// weird quirk with this, an identifier can't have a *
// but, this is a play build. TODO
pub fn identifier<'a>(input: &mut &'a str) -> Result<&'a str> {
    take_while(1.., ('a'..='z', 'A'..='Z', '0'..='9', ':', '_', '*')).parse_next(input)
}
