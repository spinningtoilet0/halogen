use winnow::{
    Parser, Result,
    ascii::{multispace0, multispace1},
    combinator::{alt, delimited, opt, preceded, repeat, separated, separated_pair, terminated},
    token::none_of,
};

use crate::{
    bind::{self, Bind},
    util::identifier,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TypedParameter {
    pub ty: String,
    pub name: String,
}

impl TypedParameter {
    fn new(ty: &str, name: &str) -> Self {
        Self {
            ty: ty.to_owned(),
            name: name.to_owned(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Method {
    pub name: String,
    pub return_type: String,
    pub params: Vec<TypedParameter>,
    pub bind: Bind,
    /// Denotes if the function is virtual
    pub virt: bool,
    /// Denotes if the function is static
    pub stat: bool,
}

fn opt_parameters(input: &mut &str) -> Result<Vec<TypedParameter>> {
    let params: Vec<(&str, &str)> = separated(
        0..,
        separated_pair(identifier, multispace1, identifier),
        (multispace0, ",", multispace0),
    )
    .parse_next(input)?;

    Ok(params
        .into_iter()
        .map(|(ty, name)| TypedParameter::new(ty, name))
        .collect())
}

#[derive(Clone)]
enum VirtualOrStatic {
    Virtual,
    Static,
}

fn balance_braces<'a>(input: &mut &'a str) -> Result<()> {
    delimited(
        "{",
        repeat(
            0..,
            alt((balance_braces.void(), none_of(['{', '}']).void())),
        ),
        "}",
    )
    .parse_next(input)
}

pub fn parse_method(input: &mut &str) -> Result<Method> {
    let _ = multispace0.parse_next(input)?;

    let virtual_or_static = opt(terminated(
        alt((
            "virtual".value(VirtualOrStatic::Virtual),
            "static".value(VirtualOrStatic::Static),
        )),
        multispace1,
    ))
    .parse_next(input)?;

    let (return_type, name) =
        separated_pair(identifier, multispace1, identifier).parse_next(input)?;

    let params = delimited("(", opt_parameters, ")").parse_next(input)?;

    let bind = preceded(
        (multispace0, "=", multispace0),
        terminated(
            bind::parse_bind,
            alt((
                (multispace0, balance_braces).void(),
                (multispace0, ";").void(),
            )),
        ),
    )
    .parse_next(input)?;

    let virt = matches!(virtual_or_static, Some(VirtualOrStatic::Virtual));
    let stat = matches!(virtual_or_static, Some(VirtualOrStatic::Static));

    Ok(Method {
        return_type: return_type.to_owned(),
        name: name.to_owned(),
        params,
        bind,
        virt,
        stat,
    })
}

#[cfg(test)]
mod test {
    use std::num::NonZeroU64;

    use crate::method::TypedParameter;

    #[test]
    fn parse() {
        let mut data = "int someFunc() = ios 0x17;";

        let method = super::parse_method(&mut data).expect("failed to parse");

        assert_eq!(method.return_type, "int");
        assert_eq!(method.name, "someFunc");
        assert!(method.params.is_empty());
        assert_eq!(method.bind.ios, NonZeroU64::new(0x17));
    }

    #[test]
    fn parameters() {
        let mut data = "int someFunc(int hi, double somethingFLoat) = win 0x67, android32 0x99;";

        let method = super::parse_method(&mut data).expect("failed to parse");

        assert_eq!(method.return_type, "int");
        assert_eq!(method.name, "someFunc");
        assert_eq!(
            method.params,
            vec![
                TypedParameter::new("int", "hi"),
                TypedParameter::new("double", "somethingFLoat")
            ]
        );
        assert_eq!(method.bind.win, NonZeroU64::new(0x67));
        assert_eq!(method.bind.android32, NonZeroU64::new(0x99));
    }

    #[test]
    fn method_body() {
        let mut data = "int someFunc(int hi, double somethingFLoat) = win 0x67, android32 0x99 {lalalalalala im ignored and these are some braces {{{{}}}} lalalala}";

        let method = super::parse_method(&mut data).expect("failed to parse");

        assert_eq!(method.return_type, "int");
        assert_eq!(method.name, "someFunc");
        assert_eq!(
            method.params,
            vec![
                TypedParameter::new("int", "hi"),
                TypedParameter::new("double", "somethingFLoat")
            ]
        );
        assert_eq!(method.bind.win, NonZeroU64::new(0x67));
        assert_eq!(method.bind.android32, NonZeroU64::new(0x99));
        assert!(data.is_empty());
    }
}
