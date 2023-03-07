fn ru_substr(s: &str, start: usize, len: usize) -> &str {
	let mut i: usize = 0;
	let mut j: usize = 0;
	let mut c: char;
	let mut s2: &str = "";

	while s.chars().nth(i as usize).unwrap() == ' ' {
		i += 1;
	}
	while s.chars().nth(i as usize).unwrap() != '\0' {
		c = s.chars().nth(i as usize).unwrap();
		if j >= start && j < start + len {
			s2 = s2 + &c.to_string();
		}
		i += 1;
		j += 1;
	}
	return s2;
}