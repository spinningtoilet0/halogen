pub struct Member {
    pub ty: String,
    pub name: String,
}

peg::parser! {
    grammar member_parser() for str {
        rule _ = ([' ']*)

        rule ty() -> String
            = ty:(['a'..='z' | 'A'..='Z' | '0'..='9' | '*']+) {
                ty.into_iter().collect()
            }

        rule name() -> String
            = name:(['a'..='z' | 'A'..='Z' | '0'..='9' | '_']+) {
                name.into_iter().collect()
            }

        pub rule member() -> Member
            = ty:ty() _ name:name() ";" {
                Member {
                    ty,
                    name
                }
            }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_member() {
        let parsed = member_parser::member("int hi;").expect("failed to parse");

        assert_eq!(&parsed.ty, "int");
        assert_eq!(&parsed.name, "hi");
    }

    #[test]
    fn pointer_type() {
        let parsed = member_parser::member("ASDF* hi;").expect("failed to parse");

        assert_eq!(&parsed.ty, "ASDF*");
        assert_eq!(&parsed.name, "hi");
    }

    #[test]
    fn invalid_type() {
        let parsed = member_parser::member("__ hi;");

        assert!(parsed.is_err());
    }

    #[test]
    fn invalid_name() {
        let parsed = member_parser::member("PlayerObject* **hi;");

        assert!(parsed.is_err());
    }

    #[test]
    fn no_semicolon() {
        let parsed = member_parser::member("int hi");

        assert!(parsed.is_err());
    }
}
