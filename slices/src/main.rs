//slice have the format [start..end], where start is the first position
//and end is one more than the last position
//it does this as end-start is the length of the slice
//if the range starts at 0 the first value can be omitted [..end]
//vice versa [start..]
//also [..] is the entire string
//str is the type for a string slice
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {}

