use std::num::NonZeroUsize;

use crate::platform::Platform;

#[derive(Default)]
pub struct Bind {
    pub win: Option<NonZeroUsize>,
    pub intel_mac: Option<NonZeroUsize>,
    pub m1_mac: Option<NonZeroUsize>,
    pub ios: Option<NonZeroUsize>,
    pub android32: Option<NonZeroUsize>,
    pub android64: Option<NonZeroUsize>,
}

peg::parser! {
    grammar bind_parser() for str {
        rule _ = ([' ']*)

        rule hex_usize() -> Option<usize>
            = "0x" h:$(['0'..='9' | 'a'..='f' | 'A'..='F']+)
            { usize::from_str_radix(h, 16).ok() }

        rule addr() -> Option<usize>
            = "inline" { None }
            / x:hex_usize() { x }

        rule entry() -> (Platform, Option<NonZeroUsize>)
            = "win" _ v:addr()
                { (Platform::Windows, v.and_then(|v| NonZeroUsize::new(v))) }
            / "imac" _ v:addr()
                { (Platform::IntelMac, v.and_then(|v| NonZeroUsize::new(v))) }
            / "m1" _ v:addr()
                { (Platform::M1Mac, v.and_then(|v| NonZeroUsize::new(v))) }
            / "mac" _ v:addr()
                { (Platform::Mac, v.and_then(|v| NonZeroUsize::new(v))) }
            / "ios" _ v:addr()
                { (Platform::IOS, v.and_then(|v| NonZeroUsize::new(v))) }
            / "android32" _ v:addr()
                { (Platform::Android32, v.and_then(|v| NonZeroUsize::new(v))) }
            / "android64" _ v:addr()
                { (Platform::Android64, v.and_then(|v| NonZeroUsize::new(v))) }
            / "android" _ v:addr()
                { (Platform::Android, v.and_then(|v| NonZeroUsize::new(v))) }

        pub rule bind() -> Bind
            = l:(entry() ++ ("," _)) {
                let mut bind = Bind::default();

                // ew
                for (platform, addr) in l {
                    match platform {
                        Platform::Android => {
                            bind.android32 = addr;
                            bind.android64 = addr;
                        },
                        Platform::Android32 => bind.android32 = addr,
                        Platform::Android64 => bind.android64 = addr,
                        Platform::IOS => bind.ios = addr,
                        Platform::Mac => {
                            bind.intel_mac = addr;
                            bind.m1_mac = addr;
                        }
                        Platform::IntelMac => bind.intel_mac = addr,
                        Platform::M1Mac => bind.m1_mac = addr,
                        Platform::Windows => bind.win = addr,
                    }
                }

                bind
            }
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZero;

    use super::*;

    #[test]
    fn parse_bind() {
        let parsed = bind_parser::bind("ios 0x69, win 0x67, android 0x42, m1 0x1, imac 0x3")
            .expect("failed to parse");

        assert_eq!(parsed.ios, NonZero::new(0x69));
        assert_eq!(parsed.win, NonZero::new(0x67));
        assert_eq!(parsed.android64, NonZero::new(0x42));
        assert_eq!(parsed.android32, NonZero::new(0x42));
        assert_eq!(parsed.m1_mac, NonZero::new(0x1));
        assert_eq!(parsed.intel_mac, NonZero::new(0x3));
    }

    #[test]
    fn parse_bind_inline() {
        let parsed = bind_parser::bind("ios inline, win 0x67, android 0x42, m1 0x1, imac 0x3")
            .expect("failed to parse");

        assert_eq!(parsed.ios, None);
        assert_eq!(parsed.win, NonZero::new(0x67));
        assert_eq!(parsed.android64, NonZero::new(0x42));
        assert_eq!(parsed.android32, NonZero::new(0x42));
        assert_eq!(parsed.m1_mac, NonZero::new(0x1));
        assert_eq!(parsed.intel_mac, NonZero::new(0x3));
    }

    #[test]
    fn invalid_bind_platform() {
        let parsed = bind_parser::bind("asdf 0x1, ios 0x10");

        assert!(parsed.is_err());
    }

    #[test]
    fn invalid_bind_addr() {
        let parsed = bind_parser::bind("win 0x0").expect("failed to parse");

        assert_eq!(parsed.win, None);
    }
}
