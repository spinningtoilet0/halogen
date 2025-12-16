pub struct Class {
    pub name: String,
    pub superclasses: Vec<String>,
}

peg::parser! {
    grammar class_parser() for str {
        rule _ = ([' ']*)

        rule classname() -> String
            = name:(['a'..='z' | 'A'..='Z']+) {
                name.into_iter().collect()
            }

        rule superclasses() -> Vec<String>
            = _ ":" _ names:(classname() ++ ("," _)) { names }

        pub rule class() -> Class
            = "class" _ name:classname() sc:superclasses()? _ "{}" {
                Class {
                    name,
                    superclasses: sc.unwrap_or_default()
                }
            }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_class() {
        let parsed = class_parser::class("class GJBaseGameLayer {}").expect("failed to parse");

        assert_eq!(parsed.name, "GJBaseGameLayer");
    }

    #[test]
    fn invalid_class_name() {
        let parsed = class_parser::class("class {}");

        assert!(parsed.is_err());
    }

    #[test]
    fn invalid_superclasses() {
        let parsed = class_parser::class("class {} : {}");

        assert!(parsed.is_err());
    }

    #[test]
    fn parse_class_subclass() {
        let parsed =
            class_parser::class("class PlayLayer : GJBaseGameLayer {}").expect("failed to parse");

        assert_eq!(parsed.name, "PlayLayer");
        assert_eq!(parsed.superclasses.len(), 1);
        assert_eq!(parsed.superclasses.first().unwrap(), "GJBaseGameLayer");
    }
}
