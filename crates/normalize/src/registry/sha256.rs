//! SHA-256, as FIPS 180-4 defines it.
//!
//! Here rather than from a library because the registry's version is the one
//! thing in this crate that must be exactly the digest the record names, and a
//! digest is the rare piece of work whose correctness is fully pinned by
//! published vectors: the tests below are the ones NIST publishes with the
//! standard, so an error in the constants or the schedule cannot pass them.
//!
//! It hashes a stream rather than a slice, because the tree it digests is read
//! one file at a time and holding every byte of the registry in one buffer to
//! hash it would be a copy of the registry made for no reason.

/// The first thirty-two bits of the fractional parts of the cube roots of the
/// first sixty-four primes, which is what the standard says this table is.
const ROUND: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/// The first thirty-two bits of the fractional parts of the square roots of the
/// first eight primes.
const START: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

const BLOCK: usize = 64;

/// A digest being taken, over as many writes as the caller makes.
pub(super) struct Sha256 {
    words: [u32; 8],
    block: [u8; BLOCK],
    held: usize,
    counted: u64,
}

impl Sha256 {
    pub(super) fn new() -> Self {
        Sha256 {
            words: START,
            block: [0; BLOCK],
            held: 0,
            counted: 0,
        }
    }

    pub(super) fn write(&mut self, mut bytes: &[u8]) {
        self.counted = self.counted.wrapping_add(bytes.len() as u64);

        if self.held > 0 {
            let taken = (BLOCK - self.held).min(bytes.len());
            self.block[self.held..self.held + taken].copy_from_slice(&bytes[..taken]);
            self.held += taken;
            bytes = &bytes[taken..];
            if self.held == BLOCK {
                let block = self.block;
                self.compress(&block);
                self.held = 0;
            }
        }

        while bytes.len() >= BLOCK {
            let (block, rest) = bytes.split_at(BLOCK);
            self.compress(block);
            bytes = rest;
        }

        if !bytes.is_empty() {
            self.block[..bytes.len()].copy_from_slice(bytes);
            self.held = bytes.len();
        }
    }

    /// The padding the standard fixes — a one bit, zeroes, and the length in
    /// bits — written through `write` like anything else, so there is one path
    /// into the compression function rather than two.
    pub(super) fn finish(mut self) -> [u8; 32] {
        let bits = self.counted.wrapping_mul(8);

        self.write(&[0x80]);
        while self.held != BLOCK - 8 {
            self.write(&[0]);
        }
        self.write(&bits.to_be_bytes());

        let mut digest = [0; 32];
        for (at, word) in self.words.iter().enumerate() {
            digest[at * 4..at * 4 + 4].copy_from_slice(&word.to_be_bytes());
        }
        digest
    }

    fn compress(&mut self, block: &[u8]) {
        let mut schedule = [0u32; 64];
        for (at, word) in schedule.iter_mut().take(16).enumerate() {
            let held = &block[at * 4..at * 4 + 4];
            *word = u32::from_be_bytes([held[0], held[1], held[2], held[3]]);
        }
        for at in 16..64 {
            let low = schedule[at - 15];
            let high = schedule[at - 2];
            let mixed_low = low.rotate_right(7) ^ low.rotate_right(18) ^ (low >> 3);
            let mixed_high = high.rotate_right(17) ^ high.rotate_right(19) ^ (high >> 10);
            schedule[at] = schedule[at - 16]
                .wrapping_add(mixed_low)
                .wrapping_add(schedule[at - 7])
                .wrapping_add(mixed_high);
        }

        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = self.words;
        for at in 0..64 {
            let sum_e = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let chosen = (e & f) ^ (!e & g);
            let first = h
                .wrapping_add(sum_e)
                .wrapping_add(chosen)
                .wrapping_add(ROUND[at])
                .wrapping_add(schedule[at]);
            let sum_a = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let majority = (a & b) ^ (a & c) ^ (b & c);
            let second = sum_a.wrapping_add(majority);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(first);
            d = c;
            c = b;
            b = a;
            a = first.wrapping_add(second);
        }

        for (word, added) in self.words.iter_mut().zip([a, b, c, d, e, f, g, h]) {
            *word = word.wrapping_add(added);
        }
    }
}

/// The digest as the sixty-four lowercase hexadecimal characters a version is
/// named by.
pub(super) fn rendered(digest: &[u8; 32]) -> String {
    let mut spelled = String::with_capacity(64);
    for byte in digest {
        spelled.push(char::from_digit((byte >> 4) as u32, 16).expect("a nibble is a hex digit"));
        spelled.push(char::from_digit((byte & 0xf) as u32, 16).expect("a nibble is a hex digit"));
    }
    spelled
}

/// The vectors FIPS 180-4 publishes with the algorithm, which is what makes
/// writing it here rather than taking it from a library a checkable decision
/// rather than a hopeful one.
#[cfg(test)]
mod matches_the_published_vectors {
    use super::{Sha256, rendered};

    fn digest_of(message: &[u8]) -> String {
        let mut taken = Sha256::new();
        taken.write(message);
        rendered(&taken.finish())
    }

    #[test]
    fn the_empty_message() {
        assert_eq!(
            digest_of(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn one_block_short_of_its_padding() {
        assert_eq!(
            digest_of(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn a_message_whose_padding_needs_a_second_block() {
        assert_eq!(
            digest_of(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
        );
    }

    #[test]
    fn a_million_of_one_letter() {
        assert_eq!(
            digest_of(&[b'a'; 1_000_000]),
            "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0"
        );
    }

    /// The same bytes in one write and in many. The tree is written a file at a
    /// time, so a digest that depended on where the writes fell would name a
    /// tree by how it happened to be read.
    #[test]
    fn the_writes_it_arrives_in_do_not_change_it() {
        let message = [b'a'; 1_000_000];

        let mut piecemeal = Sha256::new();
        let mut at = 0;
        let mut step = 1;
        while at < message.len() {
            let end = (at + step).min(message.len());
            piecemeal.write(&message[at..end]);
            at = end;
            step = step * 3 + 1;
        }

        assert_eq!(rendered(&piecemeal.finish()), digest_of(&message));
    }
}
