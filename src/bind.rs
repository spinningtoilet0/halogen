use std::num::NonZeroU64;

use winnow::{
    Parser, Result,
    ascii::{hex_uint, line_ending, space0, space1},
    combinator::{alt, eof, opt, peek, preceded, repeat, separated_pair, terminated},
};

use crate::platform::{Platform, parse_platform};

#[derive(Default, Debug)]
pub struct Bind {
    pub win: Option<NonZeroU64>,
    pub intel_mac: Option<NonZeroU64>,
    pub m1_mac: Option<NonZeroU64>,
    pub ios: Option<NonZeroU64>,
    pub android32: Option<NonZeroU64>,
    pub android64: Option<NonZeroU64>,
}

pub fn parse_bind(input: &mut &str) -> Result<Bind> {
    repeat(
        1..,
        terminated(
            // platform{space1}addr
            separated_pair(
                parse_platform,
                space1,
                alt((
                    "inline".value(None),
                    preceded("0x", hex_uint).map(NonZeroU64::new),
                )),
            ),
            // seperator
            alt((
                // force comma if in the middle of data
                (space0, ",", space0).value(()),
                // optional comma at the end of the line
                (space0, opt(","), space0, peek(alt((line_ending, eof)))).value(()),
            )),
        ),
    )
    .fold(Bind::default, |mut bind, (platform, addr)| {
        match platform {
            Platform::Android => {
                bind.android32 = addr;
                bind.android64 = addr;
            }
            Platform::Android32 => bind.android32 = addr,
            Platform::Android64 => bind.android64 = addr,
            Platform::Mac => {
                bind.m1_mac = addr;
                bind.intel_mac = addr;
            }
            Platform::IntelMac => bind.intel_mac = addr,
            Platform::M1Mac => bind.m1_mac = addr,
            Platform::IOS => bind.ios = addr,
            Platform::Windows => bind.win = addr,
        };

        bind
    })
    .parse_next(input)
}

#[cfg(test)]
mod test {
    use std::num::NonZeroU64;

    #[test]
    fn parse() {
        let mut data = "win 0xff, ios 0x1, mac 0x84, android32 0xfffffff";

        let bind = super::parse_bind(&mut data).expect("failed to parse");

        assert_eq!(bind.win, NonZeroU64::new(0xff));
        assert_eq!(bind.ios, NonZeroU64::new(0x1));
        assert_eq!(bind.m1_mac, NonZeroU64::new(0x84));
        assert_eq!(bind.intel_mac, NonZeroU64::new(0x84));
        assert_eq!(bind.android32, NonZeroU64::new(0xfffffff));
        assert_eq!(bind.android64, None);
    }

    #[test]
    fn zero_addr() {
        let mut data = "win 0x0";

        let bind = super::parse_bind(&mut data).expect("failed to parse");

        assert_eq!(bind.win, None);
    }

    #[test]
    fn terminating_spaces() {
        let mut data = "win 0x45 , ios 0x451";

        let bind = super::parse_bind(&mut data).expect("failed to parse");

        assert_eq!(bind.win, NonZeroU64::new(0x45));
        assert_eq!(bind.ios, NonZeroU64::new(0x451));
    }

    // TODO write more tests
}
