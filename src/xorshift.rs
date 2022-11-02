#[derive(Default, Copy, Clone)]
pub struct Xoroshiro {
    pub seed0: u64,
    pub seed1: u64,
}

impl Xoroshiro {
    pub fn new(seed: u64) -> Self {
        Self {
            seed0: seed,
            seed1: 0x82A2B175229D6A5B,
        }
    }

    pub fn next_u64(&mut self) -> u64 {
        let result = self.seed0.wrapping_add(self.seed1);

        self.seed1 ^= self.seed0;
        self.seed0 = self.seed0.rotate_left(24) ^ self.seed1 ^ (self.seed1 << 16);
        self.seed1 = self.seed1.rotate_left(37);

        result
    }

    pub fn next(&mut self) -> u32 {
        self.next_u64() as u32
    }

    pub fn next_max(&mut self, max: u64) -> u64 {
        let mask = Xoroshiro::get_bit_mask(max);
        let mut res;
        while {
            res = self.next_u64() & mask;
            res >= max
        } {}
        res
    }

    fn get_bit_mask(mut x: u64) -> u64 {
        x -= 1;
        x |= x >> 1;
        x |= x >> 2;
        x |= x >> 4;
        x |= x >> 8;
        x |= x >> 16;
        x
    }

    pub fn next_f32(&mut self, range: f32, bias: f32) -> f32 {
        const INV_64_F: f32 = 5.421e-20;
        let next = self.next_u64();
        (range * (next as f32 * INV_64_F)) + bias
    }
}
