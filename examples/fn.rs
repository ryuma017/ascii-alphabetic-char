use ascii_alphabetic_char::AsciiAlphabeticChar;

fn reject_non_alphabetic_at_compile_time<const ALPHABETIC_CHAR: char>()
where
    char: AsciiAlphabeticChar<ALPHABETIC_CHAR>,
{
    println!("{} is not an ASCII alphabetic character", ALPHABETIC_CHAR);
}

fn main() {
    reject_non_alphabetic_at_compile_time::<'A'>();
    reject_non_alphabetic_at_compile_time::<'Z'>();
    reject_non_alphabetic_at_compile_time::<'a'>();
    reject_non_alphabetic_at_compile_time::<'z'>();
    // some_fn::<'0'>(); // Compile Error: `0` is not an ASCII alphabetic character
}
