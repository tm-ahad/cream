use crate::serializable_deserializable::{Serializable, Deserializable};

pub struct Smp;

impl Deserializable for Smp {
    type DeserializationType = (String, usize);

    fn deserialize(s: String) -> Option<Self::DeserializationType> {
        let i = s.find('&').unwrap();
        let mut ret_none = false;

        let idx: usize = s[..i]
            .trim()
            .parse()
            .unwrap_or_else(|_| {
                ret_none = true;
                1
            });

        if ret_none { None } else {
            let s = &s[i+1..];

            Some((String::from(s), idx))
        }
    }
}

impl Serializable for Smp {
    type SerializationType<'a> = (&'a String, &'a String);

    fn serialize(d: Self::SerializationType) -> String {
        format!("{}&{}", d.0, d.1)
    }
}