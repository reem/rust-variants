#![license = "MIT"]
#![deny(warnings)]

//! A crate full of `Either` variants of many sizes.

#[cfg(feature = "serialize")]
extern crate serialize;

#[deriving(Show, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Rand)]
#[cfg_attr(feature = "serialize", deriving(Encodable, Decodable))]
pub enum Two<A, B> {
    First(A),
    Second(B)
}

#[deriving(Show, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Rand)]
#[cfg_attr(feature = "serialize", deriving(Encodable, Decodable))]
pub enum Three<A, B, C> {
    First(A),
    Second(B),
    Third(C)
}

#[deriving(Show, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Rand)]
#[cfg_attr(feature = "serialize", deriving(Encodable, Decodable))]
pub enum Four<A, B, C, D> {
    First(A),
    Second(B),
    Third(C),
    Fourth(D)
}

#[deriving(Show, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Rand)]
#[cfg_attr(feature = "serialize", deriving(Encodable, Decodable))]
pub enum Five<A, B, C, D, E> {
    First(A),
    Second(B),
    Third(C),
    Fourth(D),
    Fifth(E)
}

#[deriving(Show, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Rand)]
#[cfg_attr(feature = "serialize", deriving(Encodable, Decodable))]
pub enum Six<A, B, C, D, E, F> {
    First(A),
    Second(B),
    Third(C),
    Fourth(D),
    Fifth(E),
    Sixth(F)
}

#[deriving(Show, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Rand)]
#[cfg_attr(feature = "serialize", deriving(Encodable, Decodable))]
pub enum Seven<A, B, C, D, E, F, G> {
    First(A),
    Second(B),
    Third(C),
    Fourth(D),
    Fifth(E),
    Sixth(F),
    Seventh(G)
}

