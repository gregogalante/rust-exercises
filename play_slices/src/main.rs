fn main() {
    // slice of an array of characters
    let arr = ['a', 'b', 'c', 'd', 'e'];
    let slice = &arr[1..3];
    println!("{:?}", slice); // -> ['b', 'c']
    let slice = &arr[1..=3];
    println!("{:?}", slice); // -> ['b', 'c', 'd']

    // slice of a vector of integers // NOTE: vectors are resizable arrays
    let vec = vec![1, 2, 3, 4, 5];
    let slice = &vec[1..3];
    println!("{:?}", slice); // -> [2, 3]
    let slice = &vec[1..=3];
    println!("{:?}", slice); // -> [2, 3, 4]

    let s1 = String::from("string world");
    let s2 = "stringlit world";
    let s1_first_word = first_word(&s1);
    let s2_first_word = first_word(&s2);
    println!("{}, {}", s1_first_word, s2_first_word); // -> string, stringlit
    let s2_first_word = first_word(s2); // NOTE: string literals are slices so they are already references 💣
    println!("{}", s2_first_word); // -> stringlit
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}