use std::string::String;

pub trait Serializable {
    type SerializationType;

    fn serialize(_: Self::SerializationType) -> String;
}

pub trait Deserializable {
    type DeserializationType;

    fn deserialize(_: String) -> Option<Self::DeserializationType>;
}
