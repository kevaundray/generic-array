extern crate generic_array;
extern crate typenum;
use typenum::consts::U97;
use generic_array::GenericArray;

#[test]
fn test() {
    let mut list97 = [0; 97];
    for i in 0..97 {
        list97[i] = i as i32;
    }
    let l : GenericArray<i32, U97> = GenericArray::from_slice(&list97);
    assert_eq!(l[0], 0);
    assert_eq!(l[1], 1);
    assert_eq!(l[32], 32);
    assert_eq!(l[56], 56);
}
