use cp_protocol::Proof;
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

    let proof = Proof::new(p, q, alpha, beta);

    let [y1, y2] = proof.create_pair(&x);

    assert_eq!(y1, BigUint::from(2u32));
    assert_eq!(y2, BigUint::from(3u32));

    let [r1, r2] = proof.create_pair(&k);

    assert_eq!(r1, BigUint::from(8u32));
    assert_eq!(r2, BigUint::from(4u32));

    let s = proof.solve(&k, &c, &x);
    assert_eq!(s, BigUint::from(5u32));

    let result = proof.verify(&r1, &r2, &y1, &y2, &c, &s);
    assert!(result);

    let x_fake = BigUint::from(7u32);
    let s = proof.solve(&k, &c, &x_fake);

    let result = proof.verify(&r1, &r2, &y1, &y2, &c, &s);
    assert!(!result);
}

#[test]
fn test_rand() {
    let alpha = BigUint::from(4u32);
    let beta = BigUint::from(9u32);
    let p = BigUint::from(23u32);
    let q = BigUint::from(11u32);
    let x = BigUint::from(6u32);
    let proof = Proof::new(p, q, alpha, beta);

    let k = proof.generate_random();
    let c = proof.generate_random();

    let [y1, y2] = proof.create_pair(&x);

    assert_eq!(y1, BigUint::from(2u32));
    assert_eq!(y2, BigUint::from(3u32));

    let [r1, r2] = proof.create_pair(&k);

    let s = proof.solve(&k, &c, &x);

    let result = proof.verify(&r1, &r2, &y1, &y2, &c, &s);
    assert!(result);
}
