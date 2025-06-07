use num_bigint::{BigUint, RandBigInt};
use rand;

pub struct Proof {
    p: BigUint,     // prime modulus
    q: BigUint,     // order of the group
    alpha: BigUint, // generator of the group
    beta: BigUint,  // generator of the group
}

impl Proof {
    pub fn new(p: BigUint, q: BigUint, alpha: BigUint, beta: BigUint) -> Self {
        Proof { p, q, alpha, beta }
    }

    // r1 == alpha^s * y1^c
    // r2 == beta^s * y2^c
    pub fn verify(
        &self,
        r1: &BigUint,
        r2: &BigUint,
        y1: &BigUint,
        y2: &BigUint,
        c: &BigUint,
        s: &BigUint,
    ) -> bool {
        let lhs1 = (&self.alpha.modpow(s, &self.p) * y1.modpow(c, &self.p))
            .modpow(&BigUint::from(1u32), &self.p);
        let lhs2 = (&self.beta.modpow(s, &self.p) * y2.modpow(c, &self.p))
            .modpow(&BigUint::from(1u32), &self.p);

        lhs1 == *r1 && lhs2 == *r2
    }

    // s = k - c * x
    // k cant be less then c * x
    pub fn solve(&self, k: &BigUint, c: &BigUint, x: &BigUint) -> BigUint {
        if *k >= c * x {
            return (k - c * x).modpow(&BigUint::from(1u32), &self.q);
        }

        return &self.q - (c * x - k).modpow(&BigUint::from(1u32), &self.q);
    }

    pub fn create_pair(&self, exp: &BigUint) -> [BigUint; 2] {
        [
            self.alpha.modpow(exp, &self.p),
            self.beta.modpow(exp, &self.p),
        ]
    }

    pub fn generate_random(&self) -> BigUint {
        let mut rng = rand::thread_rng();
        rng.gen_biguint_below(&self.q)
    }
}
