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
            // 类似 Message::Quit 被称为模式，模式可由字面值、变量、通配符和许多其他内容构成
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to rgb({}, {}, {})", r, g, b),
        }
    }
}

// 匹配分支的另一个有用功能是可以绑定匹配的模式的部分值。也就是如何从枚举变体中提取值
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// 匹配 Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {
    println!("add_fancy_hat");
}

fn remove_fancy_hat() {
    println!("remove_fancy_hat");
}

fn move_player(num_spaces: u8) {
    println!("move_player: {}", num_spaces);
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    println!("home: {:?}, loopback: {:?}", home, loopback);
    let m = Message::Write(String::from("hello"));
    m.call();
    let coin = Coin::Quarter(UsState::California);
    println!("value_in_cents: {}", value_in_cents(coin));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six: {:?}, none: {:?}", six, none);

    // 通配模式和 _ 占位符
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // 我们必须把通配分支放在最后，因为模式是按顺序匹配的，一旦模式匹配了，其它的分支将不会被匹配
        other => move_player(other),
    }

    // Rust 还提供了一个模式，当我们不想使用通配模式获取的值时，请使用 _ ，
    // 这是一个特殊的模式，可以匹配任意值而不绑定到该值。
    // 这告诉 Rust 我们不会使用这个值，所以 Rust 也不会警告我们存在未使用的变量。
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}
