fn ru_strjoin(s1: &str, s2: &str) -> String {
	let mut i: i32 = 0;
	let mut j: i32 = 0;
	let mut c: char;
	let mut s: String = String::new();

	while s1.chars().nth(i as usize).unwrap() != '\0' {
		c = s1.chars().nth(i as usize).unwrap();
		s.push(c);
		i += 1;
	}
	while s2.chars().nth(j as usize).unwrap() != '\0' {
		c = s2.chars().nth(j as usize).unwrap();
		s.push(c);
		j += 1;
	}
	return s;
}