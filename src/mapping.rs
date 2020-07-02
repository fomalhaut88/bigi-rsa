extern crate bigi;

use std::cmp;
use bigi::{Bigi};


pub struct Mapper {
    block_size: usize
}


impl Mapper {
    pub fn new(bits: usize) -> Self {
        Self {
            block_size: bits / 8 - 1
        }
    }

    pub fn pack(&self, body: &Vec<u8>) -> Vec<Bigi> {
        (0..body.len()).step_by(self.block_size).map(|start| {
            let end = cmp::min(start + self.block_size, body.len());
            let block = &body[start..end];
            Bigi::from_bytes(&block)
        }).collect()
    }

    pub fn unpack(&self, nums: &Vec<Bigi>) -> Vec<u8> {
        let mut body: Vec<u8> = Vec::new();
        for num in nums.iter() {
            let block = num.to_bytes()[..self.block_size].to_vec();
            body.extend(&block);
        }
        if let Some(idx) = body.iter().rposition(|e| *e != 0) {
            let end = idx + 1;
            body.truncate(end);
        }
        body
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_mapper() {
        let text = "RSA (Rivest-Shamir-Adleman) is one of the first public-key cryptosystems".as_bytes().to_vec();
        let mapper = Mapper::new(1024);

        let packed = mapper.pack(&text);
        let unpacked = mapper.unpack(&packed);

        assert_eq!(unpacked, text);
    }

    #[bench]
    fn bench_pack(b: &mut Bencher) {
        let text = "RSA (Rivest-Shamir-Adleman) is one of the first public-key cryptosystems".as_bytes().to_vec();
        let mapper = Mapper::new(1024);
        b.iter(|| mapper.pack(&text));
    }

    #[bench]
    fn bench_unpack(b: &mut Bencher) {
        let text = "RSA (Rivest-Shamir-Adleman) is one of the first public-key cryptosystems".as_bytes().to_vec();
        let mapper = Mapper::new(1024);
        let packed = mapper.pack(&text);
        b.iter(|| mapper.unpack(&packed));
    }
}
