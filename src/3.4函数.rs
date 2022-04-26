fn main() {
    // 函数
    // 针对函数和变量,rust 使用 下划线的风格

    another_function(5);
    let x = five(6);
}
// 形参的类型 必须指定
fn another_function(x: i32) {
    println!("{}", x);

    let y = {
        let x = 1;
        x + 3
    };
}
fn five(x: i32) -> i32 {
    x + 5
    // 返回值就是函数体里最后一个表达式的值
    // 注意不能加分号,加了分号就变成语句了
}
