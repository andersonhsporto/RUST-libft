fn ru_bzero(slice: &mut [u8]) {
	unsafe {
		ptr::write_bytes(slice.as_mut_ptr(), 0, slice.len());
	}
}
