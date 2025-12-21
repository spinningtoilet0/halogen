use winnow::{
    Parser, Result,
    ascii::{multispace0, multispace1},
    combinator::{alt, delimited, opt, preceded, repeat, separated},
    token::take_until,
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
                repeat(
                    0..,
                    preceded(
                        opt((multispace0, "//", take_until(0.., "\n"), opt("\n"))),
                        alt((
                            parse_method.map(|x| Child::Method(x)),
                            parse_member.map(|x| Child::Member(x)),
                        )),
                    ),
                ),
                (multispace0, "}"),
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
    use crate::member::Member;

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

    #[test]
    fn member() {
        let mut data = "class Test { cocos2d::SomethingLayer* hi;\nint b;\nint a; }";

        let class = super::parse_class(&mut data).expect("failed to parse");

        assert_eq!(class.name, "Test");
        assert!(class.superclasses.is_empty());
        assert_eq!(
            class.members,
            vec![
                Member {
                    ty: "cocos2d::SomethingLayer*".to_owned(),
                    name: "hi".to_owned()
                },
                Member {
                    ty: "int".to_owned(),
                    name: "b".to_owned()
                },
                Member {
                    ty: "int".to_owned(),
                    name: "a".to_owned()
                }
            ]
        );
        assert!(class.methods.is_empty());
    }

    #[test]
    fn comments() {
        let mut data = "class Test { cocos2d::SomethingLayer* hi; int b;\n// hi j{}{{}int hi i = 013;\n int a; }";

        let class = super::parse_class(&mut data).expect("failed to parse");

        assert_eq!(class.name, "Test");
        assert!(class.superclasses.is_empty());
        assert_eq!(
            class.members,
            vec![
                Member {
                    ty: "cocos2d::SomethingLayer*".to_owned(),
                    name: "hi".to_owned()
                },
                Member {
                    ty: "int".to_owned(),
                    name: "b".to_owned()
                },
                Member {
                    ty: "int".to_owned(),
                    name: "a".to_owned()
                }
            ]
        );
        assert!(class.methods.is_empty());
    }

    // the real deal
    #[test]
    fn methods_and_members() {
        let mut data = r#"class AchievementsLayer : GJDropDownLayer {
            // virtual ~AchievementsLayer();
            AchievementsLayer() = m1 0x2f3300, imac 0x360260, ios 0x41a414 {
                m_currentPage = 0;
                m_nextPageButton = nullptr;
                m_prevPageButton = nullptr;
                m_pageLabel = nullptr;
            }

            static AchievementsLayer* create() = win inline, m1 0x2f2b5c, imac 0x35fa80, ios 0x419b94 {
                auto ret = new AchievementsLayer();
                if (ret->init("Achievements")) {
                    ret->autorelease();
                    return ret;
                }
                delete ret;
                return nullptr;
            }

            virtual void keyDown(cocos2d::enumKeyCodes key) = win 0x82260, imac 0x360030, m1 0x2f3078, ios 0x41a244;
            virtual void customSetup() = win 0x81fb0, m1 0x2f2c18, imac 0x35fb70, ios 0x419c40;

            void loadPage(int page) = win 0x82300, imac 0x35fe50, m1 0x2f2ea8, ios 0x419efc;
            void onNextPage(cocos2d::CCObject* sender) = win 0x824e0, imac 0x35fe30, m1 0x2f2e9c, ios 0x419ef0;
            void onPrevPage(cocos2d::CCObject* sender) = win 0x824f0, imac 0x35fe10, m1 0x2f2e90, ios 0x419ee4;
            void setupLevelBrowser(cocos2d::CCArray* arr) = win inline, m1 0x2f31dc, imac 0x360150, ios 0x41a2f0 {
                m_listLayer->removeChildByTag(9, true);
                auto* listView = CustomListView::create(arr, BoomListType::Default, 220.f, 356.f);
                listView->setTag(9);
                m_listLayer->addChild(listView, 6);
            }
            void setupPageInfo(int itemCount, int pageStartIdx, int pageEndIdx) = win inline, m1 0x2f3264, imac 0x3601d0, ios 0x41a378 {
                m_prevPageButton->setVisible(pageStartIdx != 0);
                auto nextIndex = pageStartIdx + pageEndIdx;
                m_nextPageButton->setVisible(itemCount > nextIndex);
                nextIndex = std::min(nextIndex, itemCount);
                m_pageLabel->setString(cocos2d::CCString::createWithFormat("%i to %i of %i", pageStartIdx + 1, nextIndex, itemCount)->getCString());
            }

            int m_currentPage;
            CCMenuItemSpriteExtra* m_nextPageButton;
            CCMenuItemSpriteExtra* m_prevPageButton;
            cocos2d::CCLabelBMFont* m_pageLabel;
            cocos2d::CCPoint m_unkPoint;
        }"#;

        assert!(super::parse_class(&mut data).is_ok());
    }
}
