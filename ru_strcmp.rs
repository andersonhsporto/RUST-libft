fn ru_strcmp(s1: &str, s2: &str) -> i32 {
	let mut i: i32 = 0;
	let mut c1: char;
	let mut c2: char;

	while s1.chars().nth(i as usize).unwrap() != '\0' {
		c1 = s1.chars().nth(i as usize).unwrap();
		c2 = s2.chars().nth(i as usize).unwrap();
		if c1 != c2 {
			return c1 as i32 - c2 as i32;
		}
		i += 1;
	}
	return 0;
}