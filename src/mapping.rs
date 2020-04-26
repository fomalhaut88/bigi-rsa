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
        let size = body.len();
        let mut nums: Vec<Bigi> = Vec::new();

        for i in (0..size).step_by(self.block_size) {
            let end = cmp::min(i + self.block_size, size);
            let block = &body[i..end];
            let num = Bigi::from_bytes(&block).unwrap();
            nums.push(num);
        }

        nums
    }

    pub fn unpack(&self, nums: &Vec<Bigi>) -> Vec<u8> {
        let mut body: Vec<u8> = Vec::new();
        for num in nums.iter() {
            let block = num.to_bytes().to_vec();
            body.extend(&block);
        }
        body
    }
}
