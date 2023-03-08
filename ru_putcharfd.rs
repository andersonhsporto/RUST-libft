fn ru_putcharfd(c: char, fd: i32) {
	if fd == 1 {
		print!("{}", c);
	} else if fd == 2 {
		eprint!("{}", c);
	}
}