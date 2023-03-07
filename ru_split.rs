fn ru_split(s: &str, c: char) -> Vec<&str> {
	let mut i: i32 = 0;
	let mut j: i32 = 0;
	let mut k: i32 = 0;
	let mut n: i32 = 0;
	let mut c: char;
	let mut v: Vec<&str> = Vec::new();

	while s.chars().nth(i as usize).unwrap() != '\0' {
		c = s.chars().nth(i as usize).unwrap();
		if c == c {
			n += 1;
		}
		i += 1;
	}
	v = Vec::with_capacity(n as usize);
	i = 0;
	while s.chars().nth(i as usize).unwrap() != '\0' {
		c = s.chars().nth(i as usize).unwrap();
		if c == c {
			j = i;
			while s.chars().nth(i as usize).unwrap() != '\0' {
				c = s.chars().nth(i as usize).unwrap();
				if c == c {
					break;
				}
				i += 1;
			}
			v.push(&s[j as usize..i as usize]);
			k += 1;
		}
		i += 1;
	}
	return v;
}