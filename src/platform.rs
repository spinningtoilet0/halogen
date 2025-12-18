use winnow::{Parser, Result, combinator::alt};

#[derive(Clone)]
pub enum Platform {
    Windows,
    Mac,
    IntelMac,
    M1Mac,
    IOS,
    Android,
    Android32,
    Android64,
}

pub fn parse_platform(input: &mut &str) -> Result<Platform> {
    alt((
        "android64".value(Platform::Android64),
        "android32".value(Platform::Android32),
        "android".value(Platform::Android),
        "ios".value(Platform::IOS),
        "imac".value(Platform::IntelMac),
        "m1".value(Platform::M1Mac),
        "mac".value(Platform::Mac),
        "win".value(Platform::Windows),
    ))
    .parse_next(input)
}
