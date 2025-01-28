//! Basic WebAssembly types.
//!
//! These are simple types from the WebAssembly spec that can be shared across different crates like
//! `wasmparser` and `wasm-encoder`.

use core::fmt;
use index_vec::Idx;

/// A Wasm _typeidx_.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TypeIdx(pub u32);

/// A Wasm _funcidx_.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FuncIdx(pub u32);

/// A Wasm _tableidx_.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TableIdx(pub u32);

/// A Wasm _memidx_.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MemIdx(pub u32);

/// A Wasm _tagidx_.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TagIdx(pub u32);

/// A Wasm _globalidx_.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GlobalIdx(pub u32);

/// A Wasm _elemidx_.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ElemIdx(pub u32);

/// A Wasm _dataidx_.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DataIdx(pub u32);

/// A Wasm _localidx_.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LocalIdx(pub u32);

/// A Wasm _labelidx_.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LabelIdx(pub u32);

/// A Wasm _fieldidx_.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FieldIdx(pub u32);

macro_rules! impls {
    ($name:ident) => {
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl Idx for $name {
            fn from_usize(idx: usize) -> Self {
                Self(idx.try_into().unwrap())
            }

            fn index(self) -> usize {
                self.0 as usize
            }
        }
    };
}

impls!(TypeIdx);
impls!(FuncIdx);
impls!(TableIdx);
impls!(MemIdx);
impls!(TagIdx);
impls!(GlobalIdx);
impls!(ElemIdx);
impls!(DataIdx);
impls!(LocalIdx);
impls!(LabelIdx);
impls!(FieldIdx);
