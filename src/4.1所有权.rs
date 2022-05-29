// 所有权
// 把一个值,赋值给其他变量时就会发生移动
// 当一个包含堆数据的变量离开作用域后,他的值就会被drop
// 除非所有权移到了另一个变量上
fn main() {
    let mut s = String::from("hello");
    s.push_str(", world");

    // move
    // 两个5 被压到了栈中
    let x = 5;
    let y = x;

    // stack上复制了一份 指针,长度,容量
    // 没有复制所指heap上的值
    let s1 = String::from("s1");
    let s2 = s1;

    // 这里看着像浅拷贝,但其实不是,因为到这里s1已经失效了
    // println!("{}", s1);

    // 一个新的术语: 转移 move

    let s = String::from("hello");
    take_ownership(s);
    //borrow of moved value: `s`
    //value borrowed here after move
    // 所有权被转移了,访问不到了
    // print!("{}", s);

    let s = String::from("hello");
    let returns = take_and_gives_back_ownership(s);
    // 这里s 被借走,又还回来了
    print!("{}", returns);
}
// 当变量离开作用域时,会调用drop函数

fn take_ownership(s: String) { // s进入了作用域
} // 离开了作用域,会调用drop函数.s的内存就被释放了
fn take_and_gives_back_ownership(s: String) -> String {
    return s;
}
