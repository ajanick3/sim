//! Randomness the engine can replay.
//!
//! The generator lives in the state, so a game is reproducible from its seed.
//! Tests swap in [`ScriptedRng`], which hands back a written list of results
//! and makes a shuffle or a coin flip an ordinary assertion.

/// What the engine asks of chance.
pub trait Rng {
    /// A number in `0..bound`. `bound` is never 0.
    fn next_below(&mut self, bound: u32) -> u32;

    /// A coin flip. `true` is heads.
    fn flip(&mut self) -> bool {
        self.next_below(2) == 1
    }
}

/// Shuffle in place, by Fisher-Yates walked from the back.
///
/// This is a free function rather than a method because a generic method would
/// stop `Rng` from being dyn compatible, and the state holds a `Box<dyn Rng>`
/// so that a test can swap the generator out.
pub fn shuffle<T>(rng: &mut dyn Rng, items: &mut [T]) {
    for i in (1..items.len()).rev() {
        let j = rng.next_below(i as u32 + 1) as usize;
        items.swap(i, j);
    }
}

/// A seeded xorshift64* generator. Small, deterministic, and not for
/// cryptography.
#[derive(Debug, Clone)]
pub struct SeededRng {
    state: u64,
}

impl SeededRng {
    pub fn new(seed: u64) -> SeededRng {
        // A zero state is a fixed point of xorshift, so move it off zero.
        SeededRng {
            state: seed ^ 0x9E37_79B9_7F4A_7C15,
        }
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
}

impl Rng for SeededRng {
    fn next_below(&mut self, bound: u32) -> u32 {
        (self.next_u64() % bound as u64) as u32
    }
}

/// A generator that reads from a written list. It repeats the last value once
/// the list runs out, so a test states only the draws it cares about.
#[derive(Debug, Clone)]
pub struct ScriptedRng {
    values: Vec<u32>,
    next: usize,
}

impl ScriptedRng {
    pub fn new(values: Vec<u32>) -> ScriptedRng {
        assert!(!values.is_empty(), "a scripted generator needs a value");
        ScriptedRng { values, next: 0 }
    }
}

impl Rng for ScriptedRng {
    fn next_below(&mut self, bound: u32) -> u32 {
        let value = self.values[self.next.min(self.values.len() - 1)];
        self.next += 1;
        value % bound
    }
}
