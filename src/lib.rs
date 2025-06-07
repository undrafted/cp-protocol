use num_bigint::{BigUint, RandBigInt};
use rand::{self, Rng};

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

    pub fn generate_random_number_below(limit: &BigUint) -> BigUint {
        let mut rng = rand::thread_rng();
        rng.gen_biguint_below(&limit)
    }

    pub fn generate_random_string_below(size: usize) -> String {
        let rng = rand::thread_rng();
        rng.sample_iter(rand::distributions::Alphanumeric)
            .take(size)
            .map(char::from)
            .collect()
    }

    pub fn generate_random(&self) -> BigUint {
        Proof::generate_random_number_below(&self.q)
    }

    pub fn get_constants() -> (BigUint, BigUint, BigUint, BigUint) {
        let p = BigUint::from_bytes_be(&hex::decode(
            "AD107E1E9123A9D0D660FAA79559C51FA20D64E5683B9FD1B54B1597B61D0A75E6FA141DF95A56DBAF9A3C407BA1DF15EB3D688A309C180E1DE6B85A1274A0A66D3F8152AD6AC2129037C9EDEFDA4DF8D91E8FEF55B7394B7AD5B7D0B6C12207C9F98D11ED34DBF6C6BA0B2C8BBC27BE6A00E0A0B9C49708B3BF8A317091883681286130BC8985DB1602E714415D9330278273C7DE31EFDC7310F7121FD5A07415987D9ADC0A486DCDF93ACC44328387315D75E198C641A480CD86A1B9E587E8BE60E69CC928B2B9C52172E413042E9B23F10B0E16E79763C9B53DCF4BA80A29E3FB73C16B8E75B97EF363E2FFA31F71CF9DE5384E71B81C0AC4DFFE0C10E64F",
        ).unwrap());

        let alpha = BigUint::from_bytes_be(&hex::decode(
            "AC4032EF4F2D9AE39DF30B5C8FFDAC506CDEBE7B89998CAF74866A08CFE4FFE3A6824A4E10B9A6F0DD921F01A70C4AFAAB739D7700C29F52C57DB17C620A8652BE5E9001A8D66AD7C17669101999024AF4D027275AC1348BB8A762D0521BC98AE247150422EA1ED409939D54DA7460CDB5F6C6B250717CBEF180EB34118E98D119529A45D6F834566E3025E316A330EFBB77A86F0C1AB15B051AE3D428C8F8ACB70A8137150B8EEB10E183EDD19963DDD9E263E4770589EF6AA21E7F5F2FF381B539CCE3409D13CD566AFBB48D6C019181E1BCFE94B30269EDFE72FE9B6AA4BD7B5A0F1C71CFFF4C19C418E1F6EC017981BC087F2A7065B384B890D3191F2BFA",
        ).unwrap());

        let exp = BigUint::from_bytes_be(&hex::decode("65B384B890D3191F2BFA").unwrap());

        // q is the order of the group, it must be a prime number
        let q = BigUint::from_bytes_be(
            &hex::decode("801C0D34C58D93FE997177101F80535A4738CEBCBF389A99B36371EB").unwrap(),
        );

        // alpha^x is also a generator of the group
        // any element of the group elevated to x is also a generator
        let beta = alpha.modpow(&exp, &p);

        (p, q, alpha, beta)
    }
}
