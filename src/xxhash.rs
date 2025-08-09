use std::hash::Hasher;

const PRIME32_1: u32 = 0x9E3779B1;
const PRIME32_2: u32 = 0x85EBCA77;
const PRIME32_3: u32 = 0xC2B2AE3D;
const PRIME32_4: u32 = 0x27D4EB2F;
const PRIME32_5: u32 = 0x165667B1;

const PRIME64_1: u64 = 0x9E3779B185EBCA87;
const PRIME64_2: u64 = 0xC2B2AE3D27D4EB4F;
const PRIME64_3: u64 = 0x165667B19E3779F9;
const PRIME64_4: u64 = 0x85EBCA77C2B2AE63;
const PRIME64_5: u64 = 0x27D4EB2F165667C5;

pub struct XxHash32 {
    seed: u32,
    acc1: u32,
    acc2: u32,
    acc3: u32,
    acc4: u32,
    buffer: [u8; 16],
    buffer_len: usize,
    total_len: u64,
}

impl XxHash32 {
    pub fn new(seed: u32) -> Self {
        Self {
            seed,
            acc1: seed.wrapping_add(PRIME32_1).wrapping_add(PRIME32_2),
            acc2: seed.wrapping_add(PRIME32_2),
            acc3: seed,
            acc4: seed.wrapping_sub(PRIME32_1),
            buffer: [0; 16],
            buffer_len: 0,
            total_len: 0,
        }
    }

    pub fn write(&mut self, input: &[u8]) {
        self.total_len += input.len() as u64;
        let mut input = input;

        if self.buffer_len > 0 {
            let fill = 16 - self.buffer_len;
            if input.len() >= fill {
                self.buffer[self.buffer_len..16].copy_from_slice(&input[..fill]);
                let buffer_copy = self.buffer;
                self.process_stripe(&buffer_copy);
                input = &input[fill..];
                self.buffer_len = 0;
            } else {
                self.buffer[self.buffer_len..self.buffer_len + input.len()].copy_from_slice(input);
                self.buffer_len += input.len();
                return;
            }
        }

        while input.len() >= 16 {
            self.process_stripe(&input[..16]);
            input = &input[16..];
        }

        if !input.is_empty() {
            self.buffer[..input.len()].copy_from_slice(input);
            self.buffer_len = input.len();
        }
    }

    fn process_stripe(&mut self, stripe: &[u8]) {
        let lane1 = u32::from_le_bytes([stripe[0], stripe[1], stripe[2], stripe[3]]);
        let lane2 = u32::from_le_bytes([stripe[4], stripe[5], stripe[6], stripe[7]]);
        let lane3 = u32::from_le_bytes([stripe[8], stripe[9], stripe[10], stripe[11]]);
        let lane4 = u32::from_le_bytes([stripe[12], stripe[13], stripe[14], stripe[15]]);

        self.acc1 = self.acc1.wrapping_add(lane1.wrapping_mul(PRIME32_2)).rotate_left(13).wrapping_mul(PRIME32_1);
        self.acc2 = self.acc2.wrapping_add(lane2.wrapping_mul(PRIME32_2)).rotate_left(13).wrapping_mul(PRIME32_1);
        self.acc3 = self.acc3.wrapping_add(lane3.wrapping_mul(PRIME32_2)).rotate_left(13).wrapping_mul(PRIME32_1);
        self.acc4 = self.acc4.wrapping_add(lane4.wrapping_mul(PRIME32_2)).rotate_left(13).wrapping_mul(PRIME32_1);
    }

    pub fn finish(&self) -> u32 {
        let mut hash = if self.total_len >= 16 {
            self.acc1.rotate_left(1)
                .wrapping_add(self.acc2.rotate_left(7))
                .wrapping_add(self.acc3.rotate_left(12))
                .wrapping_add(self.acc4.rotate_left(18))
        } else {
            self.seed.wrapping_add(PRIME32_5)
        };

        hash = hash.wrapping_add(self.total_len as u32);

        let mut remaining = &self.buffer[..self.buffer_len];
        
        while remaining.len() >= 4 {
            let lane = u32::from_le_bytes([remaining[0], remaining[1], remaining[2], remaining[3]]);
            hash = hash.wrapping_add(lane.wrapping_mul(PRIME32_3));
            hash = hash.rotate_left(17).wrapping_mul(PRIME32_4);
            remaining = &remaining[4..];
        }

        for &byte in remaining {
            hash = hash.wrapping_add((byte as u32).wrapping_mul(PRIME32_5));
            hash = hash.rotate_left(11).wrapping_mul(PRIME32_1);
        }

        hash ^= hash >> 15;
        hash = hash.wrapping_mul(PRIME32_2);
        hash ^= hash >> 13;
        hash = hash.wrapping_mul(PRIME32_3);
        hash ^= hash >> 16;

        hash
    }

    pub fn oneshot(input: &[u8], seed: u32) -> u32 {
        let mut hasher = Self::new(seed);
        hasher.write(input);
        hasher.finish()
    }
}

impl Default for XxHash32 {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Hasher for XxHash32 {
    fn write(&mut self, bytes: &[u8]) {
        self.write(bytes);
    }

    fn finish(&self) -> u64 {
        self.finish() as u64
    }
}

pub struct XxHash64 {
    seed: u64,
    acc1: u64,
    acc2: u64,
    acc3: u64,
    acc4: u64,
    buffer: [u8; 32],
    buffer_len: usize,
    total_len: u64,
}

impl XxHash64 {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            acc1: seed.wrapping_add(PRIME64_1).wrapping_add(PRIME64_2),
            acc2: seed.wrapping_add(PRIME64_2),
            acc3: seed,
            acc4: seed.wrapping_sub(PRIME64_1),
            buffer: [0; 32],
            buffer_len: 0,
            total_len: 0,
        }
    }

    pub fn write(&mut self, input: &[u8]) {
        self.total_len += input.len() as u64;
        let mut input = input;

        if self.buffer_len > 0 {
            let fill = 32 - self.buffer_len;
            if input.len() >= fill {
                self.buffer[self.buffer_len..32].copy_from_slice(&input[..fill]);
                let buffer_copy = self.buffer;
                self.process_stripe(&buffer_copy);
                input = &input[fill..];
                self.buffer_len = 0;
            } else {
                self.buffer[self.buffer_len..self.buffer_len + input.len()].copy_from_slice(input);
                self.buffer_len += input.len();
                return;
            }
        }

        while input.len() >= 32 {
            self.process_stripe(&input[..32]);
            input = &input[32..];
        }

        if !input.is_empty() {
            self.buffer[..input.len()].copy_from_slice(input);
            self.buffer_len = input.len();
        }
    }

    fn process_stripe(&mut self, stripe: &[u8]) {
        let lane1 = u64::from_le_bytes([stripe[0], stripe[1], stripe[2], stripe[3], stripe[4], stripe[5], stripe[6], stripe[7]]);
        let lane2 = u64::from_le_bytes([stripe[8], stripe[9], stripe[10], stripe[11], stripe[12], stripe[13], stripe[14], stripe[15]]);
        let lane3 = u64::from_le_bytes([stripe[16], stripe[17], stripe[18], stripe[19], stripe[20], stripe[21], stripe[22], stripe[23]]);
        let lane4 = u64::from_le_bytes([stripe[24], stripe[25], stripe[26], stripe[27], stripe[28], stripe[29], stripe[30], stripe[31]]);

        self.acc1 = self.acc1.wrapping_add(lane1.wrapping_mul(PRIME64_2)).rotate_left(31).wrapping_mul(PRIME64_1);
        self.acc2 = self.acc2.wrapping_add(lane2.wrapping_mul(PRIME64_2)).rotate_left(31).wrapping_mul(PRIME64_1);
        self.acc3 = self.acc3.wrapping_add(lane3.wrapping_mul(PRIME64_2)).rotate_left(31).wrapping_mul(PRIME64_1);
        self.acc4 = self.acc4.wrapping_add(lane4.wrapping_mul(PRIME64_2)).rotate_left(31).wrapping_mul(PRIME64_1);
    }

    fn merge_accumulator(&self, mut acc: u64, acc_n: u64) -> u64 {
        acc ^= acc_n.wrapping_mul(PRIME64_2).rotate_left(31).wrapping_mul(PRIME64_1);
        acc = acc.wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
        acc
    }

    pub fn finish(&self) -> u64 {
        let mut hash = if self.total_len >= 32 {
            let mut h = self.acc1.rotate_left(1)
                .wrapping_add(self.acc2.rotate_left(7))
                .wrapping_add(self.acc3.rotate_left(12))
                .wrapping_add(self.acc4.rotate_left(18));
            
            h = self.merge_accumulator(h, self.acc1);
            h = self.merge_accumulator(h, self.acc2);
            h = self.merge_accumulator(h, self.acc3);
            h = self.merge_accumulator(h, self.acc4);
            
            h
        } else {
            self.seed.wrapping_add(PRIME64_5)
        };

        hash = hash.wrapping_add(self.total_len);

        let mut remaining = &self.buffer[..self.buffer_len];
        
        while remaining.len() >= 8 {
            let lane = u64::from_le_bytes([remaining[0], remaining[1], remaining[2], remaining[3], remaining[4], remaining[5], remaining[6], remaining[7]]);
            hash ^= lane.wrapping_mul(PRIME64_2).rotate_left(31).wrapping_mul(PRIME64_1);
            hash = hash.rotate_left(27).wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4);
            remaining = &remaining[8..];
        }

        while remaining.len() >= 4 {
            let lane = u32::from_le_bytes([remaining[0], remaining[1], remaining[2], remaining[3]]) as u64;
            hash ^= lane.wrapping_mul(PRIME64_1);
            hash = hash.rotate_left(23).wrapping_mul(PRIME64_2).wrapping_add(PRIME64_3);
            remaining = &remaining[4..];
        }

        for &byte in remaining {
            hash ^= (byte as u64).wrapping_mul(PRIME64_5);
            hash = hash.rotate_left(11).wrapping_mul(PRIME64_1);
        }

        hash ^= hash >> 33;
        hash = hash.wrapping_mul(PRIME64_2);
        hash ^= hash >> 29;
        hash = hash.wrapping_mul(PRIME64_3);
        hash ^= hash >> 32;

        hash
    }

    pub fn oneshot(input: &[u8], seed: u64) -> u64 {
        let mut hasher = Self::new(seed);
        hasher.write(input);
        hasher.finish()
    }
}

impl Default for XxHash64 {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Hasher for XxHash64 {
    fn write(&mut self, bytes: &[u8]) {
        self.write(bytes);
    }

    fn finish(&self) -> u64 {
        self.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xxhash32_empty() {
        assert_eq!(XxHash32::oneshot(&[], 0), 0x02CC5D05);
        assert_eq!(XxHash32::oneshot(&[], 0x9E3779B1), 0x36B78AE7);
    }

    #[test]
    fn test_xxhash32_basic() {
        let input = b"Nobody inspects the spammish repetition";
        assert_eq!(XxHash32::oneshot(input, 0), 0xE2293B2F);
    }

    #[test]
    fn test_xxhash64_empty() {
        assert_eq!(XxHash64::oneshot(&[], 0), 0xEF46DB3751D8E999);
    }

    #[test]
    fn test_xxhash64_basic() {
        let input = b"Nobody inspects the spammish repetition";
        assert_eq!(XxHash64::oneshot(input, 0), 0xFBCEA83C8A378BF1);
    }

    #[test]
    fn test_xxhash32_incremental() {
        let mut hasher = XxHash32::new(0);
        hasher.write(b"hello");
        hasher.write(b" ");
        hasher.write(b"world");
        let result = hasher.finish();
        
        let oneshot_result = XxHash32::oneshot(b"hello world", 0);
        assert_eq!(result, oneshot_result);
    }

    #[test]
    fn test_xxhash64_incremental() {
        let mut hasher = XxHash64::new(0);
        hasher.write(b"hello");
        hasher.write(b" ");
        hasher.write(b"world");
        let result = hasher.finish();
        
        let oneshot_result = XxHash64::oneshot(b"hello world", 0);
        assert_eq!(result, oneshot_result);
    }

    #[test]
    fn test_hasher_trait() {
        let mut hasher32 = XxHash32::new(0);
        hasher32.write(b"test");
        let _result32 = hasher32.finish();

        let mut hasher64 = XxHash64::new(0);
        hasher64.write(b"test");
        let _result64 = hasher64.finish();
    }
}