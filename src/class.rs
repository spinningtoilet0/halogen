use winnow::{
    Parser, Result,
    ascii::{multispace0, multispace1},
    combinator::{delimited, opt, preceded, separated},
    token::{take_until, take_while},
};

pub struct Class {
    pub name: String,
    pub superclasses: Vec<String>,
}

fn identifier<'a>(input: &mut &'a str) -> Result<&'a str> {
    take_while(1.., ('a'..='z', 'A'..='Z', '0'..='9', ':', '_')).parse_next(input)
}

fn opt_supers(input: &mut &str) -> Result<Option<Vec<String>>> {
    opt(preceded(
        (":", multispace0),
        separated(
            1..,
            preceded(multispace0, identifier.map(String::from)),
            (multispace0, ",", multispace0),
        ),
    ))
    .parse_next(input)
}

pub fn parse_class(input: &mut &str) -> Result<Class> {
    preceded(
        ("class", multispace1),
        (
            identifier.map(str::to_owned),
            multispace0,
            opt_supers,
            multispace0,
            delimited("{", take_until(0.., "}"), "}"),
        ),
    )
    .map(|(name, _, superclasses, _, _)| Class {
        name,
        superclasses: superclasses.unwrap_or_default(),
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
}
