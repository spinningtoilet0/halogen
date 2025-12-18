use winnow::{Parser, Result, ascii::multispace1, combinator::separated_pair};

use crate::util::identifier;

pub struct Member {
    pub ty: String,
    pub name: String,
}

pub fn parse_member(input: &mut &str) -> Result<Member> {
    separated_pair(identifier, multispace1, identifier)
        .map(|(ty, name)| Member {
            ty: ty.to_owned(),
            name: name.to_owned(),
        })
        .parse_next(input)
}

#[cfg(test)]
mod test {
    #[test]
    fn parse() {
        let mut data = "int hi";

        let member = super::parse_member(&mut data).expect("failed to parse");

        assert_eq!(member.ty, "int");
        assert_eq!(member.name, "hi");
    }

    #[test]
    fn capitals() {
        let mut data = "cocos2d::SomethingSomething m_complexVariableName42";

        let member = super::parse_member(&mut data).expect("failed to parse");

        assert_eq!(member.ty, "cocos2d::SomethingSomething");
        assert_eq!(member.name, "m_complexVariableName42");
    }

    #[test]
    fn whitespace() {
        let mut data = "cocos2d::SomethingSomething \t\nm_complexVariableName42";

        let member = super::parse_member(&mut data).expect("failed to parse");

        assert_eq!(member.ty, "cocos2d::SomethingSomething");
        assert_eq!(member.name, "m_complexVariableName42");
    }
}
