fn main() {
    let s1 = String::from("hello");
    // &s1  传入的是一个引用
    // 所有权不会发生变化
    let len = calc_Len(&s1);
    println!("{}", len);
    // 把引用作为函数参数的行为,叫做借用

    // &mut s2 传入的是可变引用
    // 可变引用在一个作用域内只能有一个
    let mut s2 = String::from("hello");
    calc_Len2(&mut s2);

    //! 可变引用在一个作用域内只能有一个,可以避免数据竞争
    // let yinyong1 = &mut s2;
    // let yinyong2 = &mut s2; // 报错
    // calc_Len2(yinyong1);

    let yinyong1 = &mut s2;
    {
        let mut s2 = String::from("hello");
        let yinyong2 = &mut s2;
        // 用一个作用域隔开就不报错}
    }
    calc_Len2(yinyong1);

    // 悬空引用
    let s = dangle();
}

fn calc_Len(s: &String) -> usize {
    // s.push_str("s"); // 报错,引用也是 不可变的
    s.len()
}
fn calc_Len2(s: &mut String) -> usize {
    s.push_str("s"); // 可变引用才能变
    s.len()
}
// &s 除了作用域就没了
// 这里编译时就能检测出来
fn dangle() -> &String {
    let s = String::from("h");
    &s
}
