pub struct GameState;

#[derive(PartialEq, Eq)]
pub struct Location(u8);

pub fn to_location(s: &str) -> Option<Location> {
    Some(Location(4 * (s.bytes().nth(0).unwrap() - b'A') + s.bytes().nth(1).unwrap() - b'1'))
}

pub fn from_location(Location(loc): Location) -> String {
    format!("{}{}", (loc / 4 + b'A') as char, (loc % 4 + b'1') as char)
}

#[cfg(test)]
mod tests;
