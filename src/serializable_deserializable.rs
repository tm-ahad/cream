use std::string::String;
use crate::channel::Input;

pub trait Serializable {
    type SerializationType;

    fn serialize(_: Self::SerializationType) -> String;
}

pub trait Deserializable {
    type DeserializationType;

    fn deserialize(_: String) -> Option<Self::DeserializationType>;
}
