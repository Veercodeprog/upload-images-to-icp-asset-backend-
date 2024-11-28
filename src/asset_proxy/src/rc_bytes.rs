// src/rc_bytes.rs
//! This module contains the implementation of [RcBytes], a reference-counted byte array.

use candid::{
    types::{Serializer as CandidSerializer, Type, TypeInner},
    CandidType, Deserialize as CandidDeserialize,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer as SerdeSerializer};
use serde_bytes::ByteBuf;
use std::convert::AsRef;
use std::ops::Deref;
use std::rc::Rc;

/// A reference-counted byte array.
#[derive(Clone, Debug, Default)]
pub struct RcBytes(Rc<ByteBuf>);

impl CandidType for RcBytes {
    fn _ty() -> Type {
        TypeInner::Vec(TypeInner::Nat8.into()).into()
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: CandidSerializer,
    {
        serializer.serialize_blob(&self.0)
    }
}

impl<'de> CandidDeserialize<'de> for RcBytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        ByteBuf::deserialize(deserializer).map(Self::from)
    }
}

impl Serialize for RcBytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: SerdeSerializer,
    {
        self.0.serialize(serializer)
    }
}

impl From<ByteBuf> for RcBytes {
    fn from(b: ByteBuf) -> Self {
        Self(Rc::new(b))
    }
}

impl AsRef<[u8]> for RcBytes {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Deref for RcBytes {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.0
    }
}

