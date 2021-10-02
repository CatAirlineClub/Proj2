
#[test]
fn it_works() {
    use crate::*;
    
    let a3 = to_location("A3").unwrap();
    let a4 = to_location("A4").unwrap();
    assert_eq!(a3.0 + 1, a4.0);
    
    let a1 = to_location("A1").unwrap();
    let b1 = to_location("B1").unwrap();
    assert_eq!(a1.0 + 4, b1.0);
}

#[test]
fn test_from_location() {
    use crate::*;
    
    let a3 = to_location("A3").unwrap();
    let a4 = to_location("A4").unwrap();
    assert_eq!("A3", from_location(a3));
    assert_eq!("A4", from_location(a4));
    
    let a1 = to_location("A1").unwrap();
    let b1 = to_location("B1").unwrap();
    assert_eq!("A1", from_location(a1));
    assert_eq!("B1", from_location(b1));
}

#[test]
fn test_feedbacks() {
    test(("H1", "B2", "D3"), ("B3", "C3", "H3"), (0, 2, 1));
    test(("H1", "B2", "D3"), ("B1", "A2", "H3"), (0, 2, 1));
    test(("H1", "B2", "D3"), ("B2", "H2", "H1"), (2, 1, 0));
    test(("A1", "D2", "B3"), ("A3", "D2", "H1"), (1, 1, 0));
    test(("A1", "D2", "B3"), ("H4", "G3", "H2"), (0, 0, 0));
    test(("A1", "D2", "B3"), ("D2", "B3", "A1"), (3, 0, 0));
}


fn test(locations: (&str, &str, &str), guess: (&str, &str, &str), feedback: (i32, i32, i32)) {
    use crate::to_location;
    let locations_array = [
        to_location(locations.0).unwrap(),
        to_location(locations.1).unwrap(),
        to_location(locations.2).unwrap(),
    ];

    let guess_array = [
        to_location(guess.0).unwrap(),
        to_location(guess.1).unwrap(),
        to_location(guess.2).unwrap(),
    ];

    let value = crate::feedback(locations_array, guess_array);
    assert_eq!(value, feedback,
        "{:?} {:?} expect {:?} got {:?}", locations, guess, feedback, value);
}
