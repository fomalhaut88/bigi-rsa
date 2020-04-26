extern crate rand;
extern crate bigi;

use rand::Rng;
use bigi::{bigi, BIGI_MAX_DIGITS, Bigi};
use bigi::prime;


#[derive(Debug)]
pub struct RSA {
    bits: usize,
    phi: Bigi,
    modulo: Bigi
}


#[derive(Debug)]
pub struct PrivateKey {
    bits: usize,
    modulo: Bigi,
    key: Bigi
}


#[derive(Debug)]
pub struct PublicKey {
    bits: usize,
    modulo: Bigi,
    key: Bigi
}


impl RSA {
    pub fn new<R: Rng + ?Sized>(bits: usize, rng: &mut R) -> Self {
        let p1 = prime::gen_prime(rng, bits / 2);
        let p2 = {
            let mut p2;
            loop {
                p2 = prime::gen_prime(rng, bits / 2);
                if p1 != p2 {
                    break;
                }
            }
            p2
        };
        Self {
            bits: bits,
            phi: (p1 - &bigi![1]) * &(p2 - &bigi![1]),
            modulo: p1 * &p2,
        }
    }

    pub fn gen_keys<R: Rng + ?Sized>(&self, rng: &mut R) -> (PrivateKey, PublicKey) {
        let (e, d) = {
            let mut e;
            let mut d;
            loop {
                e = Bigi::gen_random(rng, self.bits, false) % &self.phi;
                if e > bigi![1] {
                    let res = prime::euclidean_extended(&e, &self.phi);
                    let gcd = res.0;
                    d = res.1;
                    if gcd == bigi![1] {
                        break;
                    }
                }
            }
            (e, d)
        };
        (
            PrivateKey { bits: self.bits, modulo: self.modulo, key: d },
            PublicKey { bits: self.bits, modulo: self.modulo, key: e }
        )
    }
}


impl PrivateKey {
    pub fn decrypt(&self, blocks: &Vec<Bigi>) -> Vec<Bigi> {
        blocks.iter().map(|m| {
            m.powmod(&self.key, &self.modulo)
        }).collect()
    }
}


impl PublicKey {
    pub fn encrypt(&self, blocks: &Vec<Bigi>) -> Vec<Bigi> {
        blocks.iter().map(|m| {
            m.powmod(&self.key, &self.modulo)
        }).collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsa() {
        let blocks: Vec<Bigi> = vec![bigi![5], bigi![12], bigi![894]];

        let mut rng = rand::thread_rng();
        let rsa = RSA::new(32, &mut rng);
        let (private_key, public_key) = rsa.gen_keys(&mut rng);
        let encrypted = public_key.encrypt(&blocks);
        let decrypted = private_key.decrypt(&encrypted);

        assert_eq!(decrypted, blocks);
    }
}
