// 声明常量
// const MAX_POINTS = 100_000;
fn main() {
    // mut 将变量变为可变的
    let mut x = 5;
    x = 6;
    println!("The value of x is {}", x);

    // shadowing 隐藏
    // 可以声明同名变量
    let a = 10;
    let a = a + 1;
    println!("The a is {}", a);

    // 可以是不同类型
    let space = "";
    let space = space.len();

    println!("The space  x is {}", space);
}
