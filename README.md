# bigi-rsa

A Rust library that implements [RSA](https://en.wikipedia.org/wiki/RSA_(cryptosystem)) using [bigi](https://github.com/fomalhaut88/bigi) as a library for multi precision arithmetic. As far as `bigi` has static data allocation for integers it is necessary to compile the library with `BIGI_BITS` environment variable. **bigi-rsa** is developed for Rust Nightly strictly.

## Test and bench

Test:

```
BIGI_BITS=2048 cargo test
```

Bench:

```
BIGI_BITS=2048 cargo bench
```

## Usage example

```rust
use bigi_rsa::{Mapper, RSA};

fn main() {
    const BITS: usize = 1024;

    let text = "RSA (Rivest-Shamir-Adleman) is one of the first public-key cryptosystems".as_bytes().to_vec();
    let mapper = Mapper::new(BITS);
    let blocks = mapper.pack(&text);

    let mut rng = rand::thread_rng();
    let rsa = RSA::new(BITS, &mut rng);
    let (private_key, public_key) = rsa.gen_keys(&mut rng);

    let encrypted = public_key.encrypt(&blocks);
    let decrypted = private_key.decrypt(&encrypted);

    let decrypted_text = String::from_utf8(mapper.unpack(&decrypted)).unwrap();

    println!("{:?}", decrypted_text);
}
```
