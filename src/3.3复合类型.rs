fn main() {
    // 数据类型 复合类型

    // 元祖 tuple
    // 长度固定 类型可以不同
    let tuple = (1, 'a', 3.3);

    // 解构tuple
    let (x, y, z) = tuple;

    // 访问tuple元素
    println!("{},{},{}", tuple.0, tuple.1, tuple.2);

    // 数组
    //长度固定 类型相同
    // 数据会存在栈内存上
    let arr = [1, 2, 3];

    // 指定类型
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // 相当于三个5
    let b = [3; 5];

    // 超出索引范围时候,
    // 简单的情况下编译会报错
    // a[6];

    //复杂的时候就不报错了
    let index = [10; 10];
    a[index[9]];
}
