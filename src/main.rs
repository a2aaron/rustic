extern crate rand;

use rand::{Rand, Rng};

fn main() {
    for _ in 0..10 {
        let note = Note::rand(&mut rand::thread_rng());
        println!("{:?} {:?} {:?}", note.octave, note.letter, note.accidental);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Note {
    octave: i64,
    letter: Letter,
    accidental: Accidental,
}

impl Rand for Note {
    fn rand<R: Rng>(rng: &mut R) -> Note {
        Note {
            octave: rng.gen_range(-2, 8),
            letter: Rand::rand(rng),
            accidental: Rand::rand(rng),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Letter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

const LETTER_VALUES: [Letter; 7] = {
    use Letter::*;
    [A, B, C, D, E, F, G]
};


impl Rand for Letter {
    fn rand<R: Rng>(rng: &mut R) -> Letter {
        LETTER_VALUES[rng.gen_range(0, LETTER_VALUES.len())]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accidental {
    Sharp,
    Flat,
    Natural,
}


const ACCIDENTAL_VALUES: [Accidental; 3] = {
    use Accidental::*;
    [Sharp, Flat, Natural]
};

impl Rand for Accidental {
    fn rand<R: Rng>(rng: &mut R) -> Accidental {
        ACCIDENTAL_VALUES[rng.gen_range(0,  ACCIDENTAL_VALUES.len())]
    }
}