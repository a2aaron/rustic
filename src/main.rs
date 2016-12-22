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

pub struct Note {
    octave: i64,
    note: NoteName,
}

impl Rand for Note {
    fn rand<R: Rng>(rng: &mut R) -> Note {
        Note {
            octave: rng.gen_range(-3, 8) as i64,
            note: NoteName::rand(rng),
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

macro_rules! randable_struct {
    (pub struct $Name:ident {
         $($field_name:ident: $field_type:ty,)*
     }) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct $Name {
            $($field_name: $field_type,)*
        }

        impl Rand for $Name {
            fn rand<R: Rng>(rng: &mut R) -> $Name {
                $Name {
                    $($field_name: Rand::rand(rng),)*
                }
            }
        }
    }
}

randable_struct! {
    pub struct NoteName {
        letter: Letter,
        accidental: Accidental,
    }
}
randable_struct! {
    pub struct KeySignature {
        mode: Mode,
        key: NoteName,
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