use cp_protocol::*;
use num_bigint::BigUint;

#[test]
fn test() {
    let alpha = BigUint::from(4u32);
    let beta = BigUint::from(9u32);
    let p = BigUint::from(23u32);
    let q = BigUint::from(11u32);

    let x = BigUint::from(6u32);
    let k = BigUint::from(7u32);

    let c = BigUint::from(4u32);

    let y1 = exponentiate(&alpha, &x, &p);
    let y2 = exponentiate(&beta, &x, &p);

    assert_eq!(y1, BigUint::from(2u32));
    assert_eq!(y2, BigUint::from(3u32));

    let r1 = exponentiate(&alpha, &k, &p);
    let r2 = exponentiate(&beta, &k, &p);

    assert_eq!(r1, BigUint::from(8u32));
    assert_eq!(r2, BigUint::from(4u32));

    let s = solve(&k, &c, &x, &q);
    assert_eq!(s, BigUint::from(5u32));

    let result = verify(&r1, &r2, &alpha, &beta, &y1, &y2, &c, &s, &p);
    assert!(result);

    let x_fake = BigUint::from(7u32);
    let s = solve(&k, &c, &x_fake, &q);

    let result = verify(&r1, &r2, &alpha, &beta, &y1, &y2, &c, &s, &p);
    assert!(!result);
}
