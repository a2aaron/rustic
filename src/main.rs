extern crate rand;

use rand::{Rand, Rng};

fn main() {
    for _ in 0..10 {
        let note = Note::rand(&mut rand::thread_rng());
        println!("{:?} {:?} {:?}", note.octave, note.note.letter, note.note.accidental);
        let signature = KeySignature::rand(&mut rand::thread_rng());
        println!("{:?} {:?}", signature.mode, signature.key);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeySignature {
    mode: Mode,
    key: NoteName,
}

impl Rand for KeySignature {
    fn rand<R: Rng>(rng: &mut R) -> KeySignature {
        KeySignature {
            mode: Mode::rand(rng),
            key: NoteName::rand(rng)
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Note {
    octave: i64,
    note: NoteName,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NoteName {
    letter: Letter,
    accidental: Accidental
}

impl Rand for Note {
    fn rand<R: Rng>(rng: &mut R) -> Note {
        Note {
            octave: rng.gen_range(-3, 8) as i64,
            note: NoteName::rand(rng)
        }
    }
}

impl Rand for NoteName {
    fn rand<R: Rng>(rng: &mut R) -> NoteName {
        NoteName {
            letter: Rand::rand(rng),
            accidental: Rand::rand(rng),
        }
    }
}

macro_rules! count {
    () => { 0 };
    ($x:expr) => { 1 };
    ($x:expr, $($xs:expr),*) => { 1 + count!($($xs),*) }
}

macro_rules! randable_enum {
    (pub enum $Name:ident { $($x:ident),* }) => {

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum $Name {
            $($x),*
        }

        impl $Name {
            fn values() -> [$Name; count!($($x),*)] {
                use $Name::*;
                [$($x),*]
            }
        }

        impl Rand for $Name {
            fn rand<R: Rng>(rng: &mut R) -> $Name {
                $Name::values()[rng.gen_range(0, $Name::values().len())]
            }
        }
    }
}

randable_enum! {
    pub enum Letter {
        A, B, C, D, E, F, G
    }
}

randable_enum! {
    pub enum Accidental {
        Sharp,
        Flat,
        Natural
    }
}

randable_enum! {
    pub enum Mode {
        Major,
        Minor
    }
}