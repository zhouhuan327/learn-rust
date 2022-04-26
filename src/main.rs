fn main() {
    // 数据类型
    // 这里需要指定类型
    let guess: u32 = "42".parse().expect("Not a number");
    println!("{}", guess);

    // 整数类型 默认i32
    // u 开头无符号整数, i 开头 有符号整数
    // u32 i32 ....
    let a = 57u8;

    // 浮点类型 默认f64
    // f32 单精度浮点 f64双精度浮点
    let x = 2.0;
    let y: f32 = 3.0;
    let res = x / y;

    // 布尔类型
    // bool
    let t = true;
    let f: bool = false;

    // 字符类型 char
    let x = 'z';
    let emoji = '😂';
}
