use num_bigint::BigUint;

// r1 == alpha^s * y1^c
// r2 == beta^s * y2^c
pub fn verify(
    r1: &BigUint,
    r2: &BigUint,
    alpha: &BigUint,
    beta: &BigUint,
    y1: &BigUint,
    y2: &BigUint,
    c: &BigUint,
    s: &BigUint,
    q: &BigUint,
) -> bool {
    let lhs1 = (alpha.modpow(s, q) * y1.modpow(c, q)) % q;
    let lhs2 = (beta.modpow(s, q) * y2.modpow(c, q)) % q;
    lhs1 == *r1 && lhs2 == *r2
}

// s = k - c * x
// k cant be less then c * x
pub fn solve(k: &BigUint, c: &BigUint, x: &BigUint, q: &BigUint) -> BigUint {
    if *k >= c * x {
        return (k - c * x).modpow(&BigUint::from(1u32), q);
    }

    return q - (c * x - k).modpow(&BigUint::from(1u32), q);
}

// n^exp mod p
pub fn exponentiate(n: &BigUint, exp: &BigUint, p: &BigUint) -> BigUint {
    n.modpow(exp, p)
}
