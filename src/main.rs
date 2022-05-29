struct User {
    name: String,
    email: String,
}
fn main() {
    // mut之后所有字段都是可变了的
    let mut user1 = User {
        name: String::from("sean"),
        email: String::from("xx"),
    };

    user1.email = String::from("xxxx update");
    let name = String::from("sean");

    let user2 = User {
        name, // 简写 和js一样
        email: String::from("xx"),
    };

    // struct更新语法,基于现有实例创建新的实例
    // 和js一样
    let user3 = User {
        name: String::from("sean"),
        ..user2
    };

    // tuple struct
    // 整体有个名,元素没有名
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let balck = Color(0, 0, 0);
    let point = Point(0, 0, 0);
}
