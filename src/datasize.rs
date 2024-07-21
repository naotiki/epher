use std::fmt;
use std::fmt::Formatter;

use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub struct Datasize {
    pub value: u64,
}
const PREFIX: [&str; 5] = ["K", "M", "G", "T", "P"];
type DatasizeFormat<'l> = (u64, [&'l str; 6]);
pub const FORMAT_DEC: DatasizeFormat = (1000, ["B", "KB", "MB", "GB", "TB", "PB"]);
pub const FORMAT_BIN: DatasizeFormat = (1024, ["", "K", "M", "G", "T", "P"]);
pub const FORMAT_BIN_I: DatasizeFormat = (1024, ["B", "KiB", "MiB", "GiB", "TiB", "PiB"]);


static DATASIZE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?<bytes>\d+)(?:(?<unit_dec>KB|MB|GB|TB|PB)?|(?<unit_bin>B|K|M|G|T|P|KiB|MiB|GiB|TiB|PiB)?)$").unwrap()
});
impl Datasize {
    pub fn new(bytes: u64) -> Self {
        Self {
            value: bytes
        }
    }
    pub fn parse(value: u64, accuracy: f64, (unit, unit_str): DatasizeFormat) -> (f64, String) {
        let mut bytes_double = value as f64;
        let mut unit_index = 0;
        let unit = unit as f64;
        while (bytes_double / unit) >= 1.0 && unit_index < unit_str.len() {
            unit_index += 1;
            bytes_double /= unit;
        }
        ((bytes_double * accuracy).round() / accuracy, unit_str[unit_index].to_string())
    }
    pub fn is_datasize_string(str: &String) -> bool {
        DATASIZE_REGEX.is_match(str.as_str())
    }
    pub fn calc_from_prefix(bytes: u64, unit: &str, base: u64) -> u64 {
        let mut exp: u64 = base;
        for pre in PREFIX {
            if unit.starts_with(pre) {
                break;
            }
            exp *= base;
        }
        bytes * exp
    }
    pub fn with_unit_string(self: &Self, accuracy: f64, format: DatasizeFormat) -> (f64, String) {
        Self::parse(self.value, accuracy, format)
    }
}

impl TryFrom<&str> for Datasize {
    type Error = ();
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        let Some(caps) = DATASIZE_REGEX.captures(str) else { return Err(()); };
        let Ok(bytes) = caps["bytes"].to_string().parse::<u64>() else { return Err(()) };
        let unit_dec = caps.name("unit_dec").map(|m| m.as_str());
        let unit_bin = caps.name("unit_bin").map(|m| m.as_str());
        let value: u64 = if let Some(unit) = unit_dec {
            Self::calc_from_prefix(bytes, unit, 1000)
        } else if let Some(unit) = unit_bin {
            if unit == "B" {
                bytes
            } else {
                Self::calc_from_prefix(bytes, unit, 1024)
            }
        } else {
            bytes
        };
        Ok(Self {
            value
        })
    }
}

impl fmt::Display for Datasize {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (v, u) = self.with_unit_string(100.0, FORMAT_BIN);
        write!(f, "{}{}", v, u)
    }
}


#[test]
fn test() {
    println!("{}", Datasize::try_from("2GiB").unwrap());
}
