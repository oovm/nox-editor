use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserID {
    hash: u64,
}

impl Debug for UserID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("UserID").field(&self.hash).finish()
    }
}

impl Serialize for UserID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        u64::serialize(&self.hash, serializer)
    }
}

impl<'de> Deserialize<'de> for UserID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        Ok(UserID { hash: u64::deserialize(deserializer)? })
    }
}