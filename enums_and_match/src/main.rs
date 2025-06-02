/*
我们直接将数据附加到枚举的每个变体上，这样就不需要一个额外的结构体了。
每一个我们定义的枚举变体的名字也变成了一个构建枚举的实例的函数。
IpAddr::V4() 是一个获取 String 参数并返回 IpAddr 类型实例的函数调用
用枚举替代结构体还有另一个优势：每个变体可以处理不同类型和数量的数据。
*/
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

/*
这个枚举有四个含有不同类型的变体：

Quit 没有关联任何数据。
Move 类似结构体包含命名字段。
Write 包含单独一个 String。
ChangeColor 包含三个 i32。
*/
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 可以通过 impl 为枚举定义方法
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to rgb({}, {}, {})", r, g, b),
        }
    }
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    println!("home: {:?}, loopback: {:?}", home, loopback);
    let m = Message::Write(String::from("hello"));
    m.call();
}
