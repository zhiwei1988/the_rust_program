**包（Packages）**：Cargo 的一个功能，它允许你构建、测试和分享 crate。一个包可以包含多个二进制 crate 项和一个可选的 crate 库。

**Crates** ：一个模块的树形结构，它形成了库或二进制项目。

## 包和 Crate 

crate 是 Rust 在编译时最小的代码单位。如果你用 rustc 而不是 cargo 来编译一个文件，编译器还是会将那个文件认作一个 crate。crate 可以包含模块，模块可以定义在其他文件，然后和 crate 一起编译。

crate 有两种形式：二进制项和库。二进制项 可以被编译为可执行程序，比如一个命令行程序或者一个 web server。它们必须有一个 main 函数来定义当程序被执行的时候所需要做的事情。

**库**并没有 main 函数，它们也不会编译为可执行程序。大多数时间 Rustaceans 说的 crate 指的都是库，这与其他编程语言中 library 概念一致。

**crate root 是一个源文件**，Rust 编译器以它为起始点，并构成你的 crate 的根模块。

**包（package）**是提供一系列功能的一个或者多个 crate。一个包会包含一个 Cargo.toml 文件，阐述如何去构建这些 crate。

包中可以包含至多一个库 crate(library crate)。包中可以包含任意多个二进制 crate(binary crate)，但是必须至少包含一个 crate（无论是库的还是二进制的）。

创建一个包

```rust
cargo new my-project
```

Cargo 遵循的一个约定：`src/main.rs` 就是一个与包同名的二进制 crate 的 crate 根。同样的，Cargo 知道如果包目录中包含 `src/lib.rs`，则包带有与其同名的库 crate。

通过将文件放在 src/bin 目录下，一个包可以拥有多个二进制 crate：**每个 src/bin 下的文件都会被编译成一个独立的二进制 crate**。

## 定义模块来控制作用域与私有性 

### 模块小抄 

- 声明模块: 在 crate 根文件中，你可以声明一个新模块；比如，你用mod garden;声明了一个叫做garden的模块。编译器会在下列路径中寻找模块代码 
    - 内联，在大括号中，当mod garden后方不是一个分号而是一个大括号
    - 在文件 `src/garden.rs`
    - 在文件 `src/garden/mod.rs`
- 声明子模块: 在除了 crate 根节点以外的其他文件中，你可以定义子模块。比如，你可能在`src/garden.rs`中定义了mod vegetables;。编译器会在以父模块命名的目录中寻找子模块代码：
    - 内联，在大括号中，当mod vegetables后方不是一个分号而是一个大括号
    - 在文件 `src/garden/vegetables.rs`
    - 在文件 `src/garden/vegetables/mod.rs`
- 模块中的代码路径: 一旦一个模块是你 crate 的一部分，你可以在隐私规则允许的前提下，从同一个 crate 内的**任意地方**，通过代码路径引用该模块的代码。举例而言，一个 garden vegetables 模块下的Asparagus类型可以在`crate::garden::vegetables::Asparagus`被找到。
- 私有 vs 公用: 一个模块里的代码默认对其父模块私有。为了使一个模块公用，应当在声明时使用pub mod替代mod。为了使一个公用模块内部的成员公用，应当在声明前使用pub。
- use 关键字: 在一个作用域内，use关键字创建了一个成员的快捷方式，用来减少长路径的重复。在任何可以引用`crate::garden::vegetables::Asparagus`的作用域，你可以通过 `use crate::garden::vegetables::Asparagus`;创建一个快捷方式，然后你就可以在作用域中只写Asparagus来使用该类型。

### 在模块中对相关代码进行分组

*模块*让我们可以将一个 crate 中的代码进行分组，以提高可读性与重用性。因为一个模块中的代码默认是私有的，所以还可以利用模块控制项的*私有性*。

## 引用模块项目的路径 

路径有两种形式：

- *绝对路径（absolute path）*是以 crate 根（root）开头的全路径；对于外部 crate 的代码，是以 **crate 名**开头的绝对路径，对于当前 crate 的代码，则以**字面值 crate** 开头。
- *相对路径（relative path）*从当前模块开始，以 self、super 或定义在当前模块中的标识符开头。

父模块中的项不能使用子模块中的私有项，但是子模块中的项可以使用它们父模块中的项。

### 使用 pub 关键字暴露路径 

使模块公有并不使其内容也是公有的。

> 二进制和库 crate 包的最佳实践 

> 我们提到过包（package）可以同时包含一个 src/[main.rs](main.rs) 二进制 crate 根和一个 src/[lib.rs](lib.rs) 库 crate 根，并且这两个 crate 默认以包名来命名。通常，这种包含二进制 crate 和库 crate 的模式的包，在二进制 crate 中只保留足以生成一个可执行文件的代码，并由可执行文件调用库 crate 的代码。又因为库 crate 可以共享，这使得其它项目从包提供的大部分功能中受益。

> 模块树应该定义在 *src/lib.rs* 中。这样通过以包名开头的路径，公有项就可以在二进制 crate 中使用。二进制 crate 就变得同其它在该 crate 之外的、使用库 crate 的用户一样：二者都只能使用公有 API。这有助于你设计一个好的 API；你不仅仅是作者，也是用户！

### 创建公有的结构体和枚举 

如果我们在一个结构体定义的前面使用了 pub，这个结构体会变成公有的，但是这个结构体的字段仍然是私有的。如果我们将枚举设为公有，则它的所有成员都将变为公有的。

## 使用 use 关键字将路径引入作用域

### use 语句只适用于其所在的作用域

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
```

mod customer 定义了一个不同于 use 的作用域，所以上面这段代码无法编译。为了修复这个问题，可以将 use 移动到 customer 模块内，或者在子模块 customer 内通过 `super::hosting` 引用父模块中的这个短路径。

### 创建惯用的 use 路径

要想使用 use 将函数的父模块引入作用域，我们必须在调用函数时指定父模块，这样可以清晰地表明函数不是在本地定义的，同时使完整路径的重复度最小化。另一方面，使用 use 引入结构体、枚举和其他项时，习惯是指定它们的完整路径。

### 使用 pub use 重导出名称

使用 use 关键字，将某个名称导入当前作用域后，这个名称在此作用域中就可以使用了，但它对此作用域之外还是私有的。如果想让其他人调用我们的代码时，也能够正常使用这个名称，就好像它本来就在当前作用域一样，那我们可以将 pub 和 use 合起来使用。这种技术被称为 “重导出（re-exporting）”：我们不仅将一个名称导入了当前作用域，还允许别人把它导入他们自己的作用域。

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

在这个修改之前，外部代码需要使用路径 `restaurant::front_of_house::hosting::add_to_waitlist()` 来调用 add_to_waitlist 函数。现在这个 pub use 从根模块重导出了 hosting 模块，外部代码现在可以使用路径 `restaurant::hosting::add_to_waitlist`。

### 使用外部包

文件名：Cargo.toml

```rust
rand = "0.8.5"
```

在 Cargo.toml 中加入 rand 依赖告诉了 Cargo 要从 [crates.io](crates.io) 下载 rand 和其依赖，并使其可在项目代码中使用。

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
```

### 通过 glob 运算符将所有的公有定义引入作用域

如果希望将一个路径下 所有 公有项引入作用域，可以指定路径后跟 *，glob 运算符：

```
use std::collections::*;
```

这个 use 语句将 `std::collections` 中定义的所有公有项引入当前作用域。使用 glob 运算符时请多加小心！Glob 会使得我们难以推导作用域中有什么名称和它们是在何处定义的。

### 嵌套路径来消除大量的 use 行

```rust
// use std::cmp::Ordering;
// use std::io;
use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;
use std::io::{self, Write};
```

## 将模块拆分成多个文件

只需在模块树中的某处使用一次 mod 声明就可以加载这个文件。一旦编译器知道了这个文件是项目的一部分（并且通过 mod 语句的位置知道了代码在模块树中的位置），项目中的其他文件应该使用其所声明的位置的路径来引用那个文件的代码。换句话说，mod 不是 你可能会在其他编程语言中看到的 "include" 操作。

> 目前为止我们介绍了 Rust 编译器所最常用的文件路径；不过一种更老的文件路径也仍然是支持的。
> 
> 对于声明于 crate 根的 front_of_house 模块，编译器会在如下位置查找模块代码：
> 
> src/front_of_house.rs（我们所介绍的）
> src/front_of_house/mod.rs（老风格，不过仍然支持）
> 对于 front_of_house 的子模块 hosting，编译器会在如下位置查找模块代码：
> 
> src/front_of_house/hosting.rs（我们所介绍的）
> src/front_of_house/hosting/mod.rs（老风格，不过仍然支持）
> 如果你对同一模块同时使用这两种路径风格，会得到一个编译错误。在同一项目中的不同模块混用不同的路径风格是允许的，不过这会使他人感到疑惑。
> 
> 使用 mod.rs 这一文件名的风格的主要缺点是会导致项目中出现很多 mod.rs 文件，当你在编辑器中同时打开它们时会感到疑惑。