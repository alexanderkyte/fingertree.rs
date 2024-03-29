#![allow(dead_code)]

//! Persistent Finger Trees.

use epsilonz_algebra::{
    MgM,
    Monoid,
};

pub use self::Digit::{
    One,
    Two,
    Three,
    Four,
};

pub use self::FingerTree::{
    Deep,
    Empty,
    Single,
};

#[deriving(Clone)]
#[deriving(Eq)]
#[deriving(Ord)]
#[deriving(PartialEq)]
#[deriving(PartialOrd)]
#[deriving(Show)]
pub enum Digit<A> {
    One(A),
    Two(A,A),
    Three(A,A,A),
    Four(A,A,A,A),
}

impl<A> Digit<A> {
    fn fold_map<M>(&self, f:|&A| -> M) -> M
        where
            M:Monoid,
    {
        match self {
            &One(ref a) => {
                f(a)
            },
            &Two(ref a, ref b) => {
                let MgM(res) = MgM(f(a)) * MgM(f(b));
                res
            },
            &Three(ref a, ref b, ref c) => {
                let MgM(res) = MgM(f(a)) * MgM(f(b)) * MgM(f(c));
                res
            },
            &Four(ref a, ref b, ref c, ref d) => {
                let MgM(res) = MgM(f(a)) * MgM(f(b)) * MgM(f(c)) * MgM(f(d));
                res
            },
        }
    }
}

#[deriving(Clone)]
#[deriving(Eq)]
#[deriving(Ord)]
#[deriving(PartialEq)]
#[deriving(PartialOrd)]
#[deriving(Show)]
pub enum FingerTree<V,A> {
    Empty,
    Single(A),
    Deep {
        measure: V,
        prefix:  Digit<A>,
        tree:    Box<FingerTree<V,A>>,
        suffix:  Digit<A>,
    },
}

pub trait Measurable<V>
    where
        V:Monoid,
{
    fn measure(&self) -> V;
}

impl<V,A> Measurable<V> for Digit<A>
    where
        A:Measurable<V>,
        V:Monoid,
{
    fn measure(&self) -> V {
        self.fold_map(|d| { Measurable::measure(d) })
    }
}

impl<V,A> Measurable<V> for FingerTree<V,A>
    where
        A:Measurable<V>,
        V:Clone,
        V:Monoid,
{
    fn measure(&self) -> V {
        match self {
            &Empty => {
                Monoid::nil()
            },
            &Single(ref x) => {
                x.measure()
            },
            &Deep {
                measure:ref v,
                ..
            } => {
                (*v).clone()
            },
        }
    }
}

impl<V,A> FingerTree<V,A> {
    pub fn is_empty(&self) -> bool {
        match self {
            &Empty => {
                true
            },
            _ => {
                false
            }
        }
    }
}
