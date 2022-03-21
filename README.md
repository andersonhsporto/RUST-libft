# RUST-libru
Regrouping usual C functions in RUST
---
### Part 1 - Libc functions

* > [ru_isalnum](/ru_isalnum.rs) `(ru_isalnum(c: char) -> i32)` checks for an alphanumeric character; it is equivalent to (isalpha(c) || isdigit(c)).

* > [ru_isalpha](/ru_isalpha.rs) `(ru_isalpha(c: char) -> i32 )` checks  for an alphanumeric character; it is equivalent to (isalpha(c) || isdigit(c)).

* > [ru_isascii](/ru_isascii.rs) `(ru_isascii(c: char) -> i32)` checks whether c is a 7-bit unsigned char value that fits into the ASCII character set.

* > [ru_isdigit](/ru_isdigit.rs) `(ru_isdigit(c: char) -> i32)` checks for a digit (0 through 9).

* > [ru_isprint](/ru_isprint.rs) `(ru_isprint(c: char) -> i32)` checks for any printable character including space.

* > [ru_isspace](/ru_isspace.rs) `(ru_isspace(c: char) -> i32)` checks for any 'spaces' characters including tab.

* > [ru_strlen](/ru_strlen.rs) `(ru_strlen(str: String) -> usize)` The strlen() function calculates the length of the string pointed to by s, excluding the terminating null byte ('\0').

* > [ru_toupper](/ru_toupper.rs) `(ru_tolower(c: char) -> char)` If  c  is a lowercase letter, toupper() returns its uppercase equivalent, if an uppercase representation exists in the current locale.  Otherwise,
       it returns c.

* > [ru_tolower](/ru_tolower.rs) `(ru_toupper(c: char) -> char)` If c is an uppercase letter, tolower() returns its lowercase equivalent, if a lowercase representation exists in the current  locale.   Otherwise,
       it returns c.


Inspired by [Libft](https://github.com/andersonhsporto/ft-libft)