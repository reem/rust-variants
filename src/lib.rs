#![license = "MIT"]
#![deny(warnings)]

//! A crate full of `Either` variants of many sizes.

#[cfg(feature = "serialize")]
extern crate serialize;

pub mod two {
    #[deriving(Show, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Rand)]
    #[cfg_attr(feature = "serialize", deriving(Encodable, Decodable))]
    pub enum Either<A, B> {
        First(A),
        Second(B)
    }
}

pub mod three {
    #[deriving(Show, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Rand)]
    #[cfg_attr(feature = "serialize", deriving(Encodable, Decodable))]
    pub enum Either<A, B, C> {
        First(A),
        Second(B),
        Third(C)
    }
}

pub mod four {
    #[deriving(Show, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Rand)]
    #[cfg_attr(feature = "serialize", deriving(Encodable, Decodable))]
    pub enum Either<A, B, C, D> {
        First(A),
        Second(B),
        Third(C),
        Fourth(D)
    }
}

pub mod five {
    #[deriving(Show, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Rand)]
    #[cfg_attr(feature = "serialize", deriving(Encodable, Decodable))]
    pub enum Either<A, B, C, D, E> {
        First(A),
        Second(B),
        Third(C),
        Fourth(D),
        Fifth(E)
    }
}

pub mod six {
    #[deriving(Show, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Rand)]
    #[cfg_attr(feature = "serialize", deriving(Encodable, Decodable))]
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
    #[deriving(Show, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Rand)]
    #[cfg_attr(feature = "serialize", deriving(Encodable, Decodable))]
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

