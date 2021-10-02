
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
