fn main() {
    let month_days: std::ops::Range<i8> = 1..31;
    let letters: std::ops::Range<char> = 'b'..'r';
    println!("{month_days:?} {letters:?}");

    let month_days: std::ops::Range<i32> = 1..31;
    let letters: std::ops::Range<char> = 'a'..'z';
    
    println!("{month_days:?}");
}
