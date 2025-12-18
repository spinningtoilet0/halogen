use winnow::{
    Parser, Result,
    ascii::{multispace0, multispace1},
    combinator::{alt, delimited, opt, preceded, repeat, separated},
};

use crate::{
    member::{Member, parse_member},
    method::{Method, parse_method},
    util::identifier,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Class {
    pub name: String,
    pub superclasses: Vec<String>,
    pub methods: Vec<Method>,
    pub members: Vec<Member>,
}

fn opt_supers(input: &mut &str) -> Result<Option<Vec<String>>> {
    opt(preceded(
        (":", multispace0),
        separated(
            1..,
            preceded(multispace0, identifier.map(str::to_owned)),
            (multispace0, ",", multispace0),
        ),
    ))
    .parse_next(input)
}

#[derive(Clone)]
enum Child {
    Method(Method),
    Member(Member),
}

pub fn parse_class(input: &mut &str) -> Result<Class> {
    preceded(
        ("class", multispace1),
        (
            identifier.map(str::to_owned),
            multispace0,
            opt_supers,
            multispace0,
            delimited(
                "{",
                alt((
                    repeat(
                        0..,
                        preceded(
                            multispace0,
                            alt((
                                parse_method.map(|x| Child::Method(x)),
                                parse_member.map(|x| Child::Member(x)),
                            )),
                        ),
                    ),
                    multispace0.value(Vec::new()),
                )),
                "}",
            ),
        ),
    )
    .map(|parsed: (String, _, Option<Vec<String>>, _, Vec<Child>)| {
        let mut members = Vec::new();
        let mut methods = Vec::new();

        for child in parsed.4 {
            match child {
                Child::Member(x) => members.push(x),
                Child::Method(x) => methods.push(x),
            }
        }

        Class {
            name: parsed.0,
            superclasses: parsed.2.unwrap_or_default(),
            members,
            methods,
        }
    })
    .parse_next(input)
}

#[cfg(test)]
mod test {
    #[test]
    fn parse() {
        let mut data = "class Hi {}";

        let class = super::parse_class(&mut data).expect("failed to parse");

        assert_eq!(class.name, "Hi");
        assert_eq!(class.superclasses, Vec::<String>::new());
    }

    #[test]
    fn superclass() {
        let mut data = "class asdfjadsf::Hi : Whatup {}";

        let class = super::parse_class(&mut data).expect("failed to parse");

        assert_eq!(class.name, "asdfjadsf::Hi");
        assert_eq!(class.superclasses, vec!["Whatup"]);
    }

    #[test]
    fn multiple_superclass() {
        let mut data = "class asdfjadsf::Hi : Whatup, Whatup2, Test {}";

        let class = super::parse_class(&mut data).expect("failed to parse");

        assert_eq!(class.name, "asdfjadsf::Hi");
        assert_eq!(class.superclasses, vec!["Whatup", "Whatup2", "Test"]);
    }

    #[test]
    fn invalid_name() {
        let mut data = "class hi-im-an-invalid-name : a {}";

        let class = super::parse_class(&mut data);

        assert!(class.is_err());
    }

    #[test]
    fn whitespace() {
        let mut data = "class \tTest \t\t\n\n:\n super\t, \nSuper2  \n{}";

        let class = super::parse_class(&mut data).expect("failed to parse");

        assert_eq!(class.name, "Test");
        assert_eq!(class.superclasses, vec!["super", "Super2"]);
    }

    // TODO
    #[test]
    fn member() {
        let mut data = "class Test { }";

        let class = super::parse_class(&mut data).expect("failed to parse");

        assert_eq!(class.name, "Test");
    }
}
