fn main() {
    // 控制流 if else
    let number = 1;
    // 这里不会做类型转换
    if number < 5 {
        println!("...");
    } else {
        println!("...");
    }

    let condition = true;
    let res = if condition { 2 } else { 5 };

    // 循环 loop while for
    // loop
    let mut i = 0;
    let res = loop {
        i = i + 1;
        println!("again!");
        if i == 3 {
            break i * 2;
            // 作为loop的返回值
        }
    };
    println!("loop result : {}", res);
    // while
    let mut j = 10;
    while j != 0 {
        j = j - 1;
    }
    println!("while stop, j : {}", j);

    // for
    let a = [1, 2, 3, 4, 5];
    for element in a.iter() {
        println!("value is {}", element)
    }
    // 从3 到 1
    for number in (1..4).rev() {
        println!("321: {}", number)
    }
}
