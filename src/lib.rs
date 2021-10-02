pub struct GameState;

#[derive(PartialEq, Eq)]

#[derive(Clone, Copy)]
pub struct Location(i8);

pub fn to_location(s: &str) -> Option<Location> {
    Some(Location((4 * (s.bytes().nth(0).unwrap() - b'A') + s.bytes().nth(1).unwrap() - b'1') as i8))
}

pub fn from_location(Location(loc): Location) -> String {
    format!("{}{}", (loc as u8 / 4 + b'A') as char, (loc as u8 % 4 + b'1') as char)
}

pub fn feedback(target: [Location; 3], guess: [Location; 3]) -> (i32, i32, i32) {
    let mut ret = (0, 0, 0);
    for guess in guess {
        if feedback_exact(target, guess) > 0 {
            ret.0 += 1;
            continue;
        };
        let onespace = feedback_onespace(target, guess);
        if onespace > 0 {
            ret.1 += onespace;
            continue;
        }
        let twospace = feedback_twospace(target, guess);
        if twospace > 0 {
            ret.2 += twospace;
            continue;
        }
    }
    return ret;
}

fn feedback_exact(target: [Location; 3], guess: Location) -> i32 { // one or zero
    for i in 0 .. 3 {
        if target[i] == guess {
            return 1;
        }
    }
    return 0;
}

fn feedback_onespace(target: [Location; 3], Location(guess): Location) -> i32 { // one or zero
    let onespace = [
        guess - 4 - 1, guess - 4, guess - 4 + 1,
        guess - 1, guess + 1,
        guess + 4 - 1, guess + 4, guess + 4 + 1,
    ];
    for i in 0 .. 3 {
        if onespace.contains(&target[i].0) {
            println!("guess {} matching onespace from {}",
                from_location(Location(guess)), from_location(target[i]));
            return 1;
        }
    }
    return 0;
}

fn feedback_twospace(target: [Location; 3], Location(guess): Location) -> i32 { // one or zero
    let twospace = [
        guess - 8 - 2, guess - 8 - 1, guess - 8, guess - 8 + 1, guess - 8 + 2,
        guess - 4 - 2, guess - 4 + 2,
        guess - 2, guess + 2,
        guess + 4 - 2, guess + 4 + 2,
        guess + 8 - 2, guess + 8 - 1, guess + 8, guess + 8 + 1, guess + 8 + 2,
    ];
    for i in 0 .. 3 {
        if twospace.contains(&target[i].0) {
            return 1;
        }
    }
    return 0;
}

#[cfg(test)]
mod tests;
