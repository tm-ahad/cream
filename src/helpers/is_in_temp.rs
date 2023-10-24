pub fn is_in_temp(s: &str, i: usize) -> bool {
    let Some(ts) = s.find("<temp>") else {todo!()};
    let Some(te) = s.find("</temp>") else {todo!()};

    ts <= i && i <= te
}
