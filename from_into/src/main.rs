fn main() {
    let arr1 = [0x05, 0x03];
    let nkl = People::from(&arr1);

    assert_eq!(
        People {
            ver: 0x05,
            kind: Kind::Enginner
        },
        nkl
    );
}

#[derive(Debug, PartialEq)]
enum Kind {
    Singer,
    Dancer,
    Doctor,
    Enginner,
    Farmer,
    Nobody,
}

impl From<u8> for Kind {
    fn from(value: u8) -> Self {
        match value {
            0 => Kind::Singer,
            1 => Kind::Dancer,
            2 => Kind::Doctor,
            3 => Kind::Enginner,
            4 => Kind::Farmer,
            _ => Kind::Nobody,
        }
    }
}

#[derive(Debug, PartialEq)]
struct People {
    ver: u8,
    kind: Kind,
}

impl From<&[u8; 2]> for People {
    fn from(value: &[u8; 2]) -> Self {
        People {
            ver: value[0],
            kind: Kind::from(value[1]),
        }
    }
}
