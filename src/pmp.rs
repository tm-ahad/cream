use crate::serializable_deserializable::{Deserializable, Serializable};

pub type MatchedPattern = (usize, usize);

pub struct PmpDeserializationType {
    sin: Vec<MatchedPattern>,
    cam: Vec<MatchedPattern>,
    dom: Vec<MatchedPattern>
}

pub struct Pmp;

impl Serializable for Pmp {
    type SerializationType = String;

    fn serialize(d: Self::SerializationType) -> String {
        d
    }
}

impl Deserializable for Pmp {
    type DeserializationType = PmpDeserializationType;

    fn deserialize(s: String) -> Option<Self::DeserializationType> {
        
    }
}
