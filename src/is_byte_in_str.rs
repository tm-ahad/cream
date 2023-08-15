
pub fn is_byte_in_str(index: usize, str: &str) -> bool {
    let mut front = str[..index].to_string();
    let mut end = str[index..].to_string();

    let mut s_f: u32 = 0;
    let mut s_e: u32 = 0;

    while let Some(a) = front.find('"') {
        s_f += 1;
        front.remove(a);
    }

    while let Some(a) = front.find('\'') {
        s_f += 1;
        front.remove(a);
    }

    while let Some(a) = front.find('`') {
        s_f += 1;
        front.remove(a);
    }

    while let Some(a) = end.find('"') {
        s_e += 1;
        end.remove(a);
    }

    while let Some(a) = end.find('\'') {
        s_e += 1;
        end.remove(a);
    }

    while let Some(a) = end.find('`') {
        s_e += 1;
        end.remove(a);
    }

    return if s_f % 2 == 0 && s_e % 2 == 0 {
        false
    } else if s_f % 2 != 0 && s_e % 2 != 0 {
        true
    } else {
        panic!("Syntax Error");
    };
}
