# RUST-libft
Regrouping usual C functions in RUST
This project is only for educational purpose and is not intended to be used in production.

*** Theres a lot unsafe code in this project, so be careful when using it. ***
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

* > [ru_atoi](/ru_atoi.rs) `(ru_atoi(str: String) -> i32)` The atoi() function converts the initial portion of the string pointed to by str to int representation.

* > [ru_bzero](/ru_bzero.rs) `(ru_bzero(s: &mut String, n: usize))` The bzero() function writes n zeroed bytes to the string s.  If n is zero, bzero() does nothing.

* > [ru_split](/ru_split.rs) `(ru_split(s: String, c: char) -> Vec<String>)` The split() function splits string s into a vector of strings using the character c as a delimiter.

* > [ru_itoa](/ru_itoa.rs) `(ru_itoa(n: i32) -> String)` The itoa() function converts the integer n to a string.

* > [ru_memschr](/ru_memschr.rs) `(ru_memschr(s: String, c: char) -> String)` The memchr() function locates the first occurrence of c (converted to an unsigned char) in string s.

* > [ru_memcmp](/ru_memcmp.rs) `(ru_memcmp(s1: String, s2: String, n: usize) -> i32)` The memcmp() function compares byte string s1 against byte string s2.  Both strings are assumed to be n bytes long.

* > [ru_strjoin](/ru_strjoin.rs) `(ru_strjoin(s1: String, s2: String) -> String)` The strjoin() function allocates (with malloc(3)) and returns a new string, which is the result of the concatenation of ’s1’ and ’s2’.

Inspired by [<em> Libft </em>](https://github.com/andersonhsporto/ft-libft)
