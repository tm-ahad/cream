use crate::channel::Input;

pub struct Mp;

impl Mp {
    pub fn decode_res(s: String) -> Option<(String, usize)> {
        let i = s.find('&').unwrap();
        let mut ret_none = false;

        let idx: usize = s[..i]
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

    pub fn encode_inp(input: Input) -> String {
        format!("{}&{}", input.0, input.1)
    }
}