fn main() {
    // slice 不持有所有权
    let s = String::from("Hello World");

    // 起始位置..终止位置+1
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{},{}", hello, world);

    println!("{},{}", &s[..5], &s[6..]);

    let whole = &s[..];

    println!("{}", whole);

    // 字符串切片
    let my_string_literal = "Hello Word";
    println!("first world, 把string转成切片: {}", first_words(&s[..]));
    println!("first world: {}", first_words(my_string_literal));

    // 数组切片
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("array slice {:#?}", slice);
}

fn first_words(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}
