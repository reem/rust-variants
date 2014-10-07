#![license = "MIT"]
#![deny(warnings)]

//! A crate full of `Either` variants of many sizes.

pub mod two {
    pub enum Either<A, B> {
        First(A),
        Second(B)
    }
}

pub mod three {
    pub enum Either<A, B, C> {
        First(A),
        Second(B),
        Third(C)
    }
}

pub mod four {
    pub enum Either<A, B, C, D> {
        First(A),
        Second(B),
        Third(C),
        Fourth(D)
    }
}

pub mod five {
    pub enum Either<A, B, C, D, E> {
        First(A),
        Second(B),
        Third(C),
        Fourth(D),
        Fifth(E)
    }
}

pub mod six {
    pub enum Either<A, B, C, D, E, F> {
        First(A),
        Second(B),
        Third(C),
        Fourth(D),
        Fifth(E),
        Sixth(F)
    }
}

pub mod seven {
    pub enum Either<A, B, C, D, E, F, G> {
        First(A),
        Second(B),
        Third(C),
        Fourth(D),
        Fifth(E),
        Sixth(F),
        Seventh(G)
    }
}

