pub fn is_byte_in_str(index: usize, str: &strs) -> bool {
	let front = &str[..index];
	let end = &str[index..];

	let mut s_f: u32 = 0;
	let mut s_e: u32 = 0;

	while let Some(_) = front.find("\"") {
		s_f += 1;
	}

	while let Some(_) = front.find("'") {
		s_f += 1;
	}

	while let Some(_) = front.find("`") {
		s_f += 1;
	}

	while let Some(_) = end.find("\"") {
		s_e += 1;
	}

	while let Some(_) = end.find("'") {
		s_e += 1;
	}

	while let Some(_) = end.find("`") {
		s_e += 1;
	}

	return if s_f%2 == 0 && s_e%2 == 0 {
		false
	} else if s_f%2 != 0 && s_f%2==0 {
		true
	} else {
		panic!("Syntax Error");
		todo!()
	}


}