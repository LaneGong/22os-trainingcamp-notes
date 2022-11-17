# Rust学习笔记3

> 参考资料：
>
> - [清华计算机系大一学生2022暑期课程：Rust程序设计训练](https://lab.cs.tsinghua.edu.cn/rust/)
> - [Rust语言圣经(Rust Course)](https://course.rs/about-book.html)
> - [rustlings](https://github.com/rust-lang/rustlings)
>

## 结构化数据

Rust有两种创建结构化数据类型的方式：

- 结构体struct：像C/C++那样的结构体，用于保存数据
- 枚举enum：像OCaml，数据可以是几种类型之一

结构体和枚举都可以有若干实现块impl，用于定义相应类型的方法 => Rust基于对象，不是面向对象（不涉及继承、多态）

### 结构体

```rust
struct Point {
	x: i32,
	y: i32,
}
```

- `name: type`来声明结构体的域(field)，也称为成员变量、字段
- 结构体用CamelCase命名，里面的域用snake_case命名

#### 初始化/实例化

```rust
let origin = Ponit { x: 0, y: 0 };
```

- 结构体的域可以用点记号来访问
- 结构体不能部分初始化：必须在创建时给所有的域赋值，也可以先声明一个未初始化的结构体，后续再进行初始化

#### 结构体可变性

- 结构体没有域级的可变性控制
- 可变性是变量绑定的属性，跟类型无关
- 域级的可变性(内部可变性)可以通过Cell类型来实现

#### 结构体访问权限

- 结构体在它所在的模块的名字空间里。Point的完整名字(full qualified name)是`foo::Point`
- 结构体的域默认是私有的，可以通过pub关键字变成公有
- 私有域只能在结构体所在模块内访问

```rust
mod foo {
	pub struct Point{
    pub x: i32,
    y: i32,
	}
}
fn main() {
	let b = foo::Point { x: 12, y:12}//error:y is private
}
```

#### 结构体的更新语法

- 结构体初始化时可以用`..s`从s中获取部分或者全部的域=>Copy特型是拷贝，其余是移动
- 所有没有在初始化列表里指定的域都从目标结构体里获取
- 两个结构体的类型必须是一致的，从不同类型结构体获得相同类型的域也是不行的

> 语法糖：使用别的方法也可以实现，但是该语言专门对其做了特殊的设计，使得使用起来更简便。

#### 元组结构体

- tuple struct 是结构体的一种形态，有结构体名字，但没有域名字
- 可以像元组那样通过数字来访问域，eg. x.0, x.1 等等
- 也可以通过match来进行匹配

```rust
struct Color(i32, i32, i32);
let mut c = Color(0, 255, 255);
c.0 = 255;
match c {
  Color(r, g, b) => println!("({}, {}, {})",r, g, b)
}
```

- 可用来创建新的类型，而不仅仅只是一个别名，被称为“新类型”模式
- 两种类型在结构上是相同的，但是并不等价(不是同一种类型)

#### 单位元结构体(零大小的类型)

- 可以声明零大小的结构体，这样的结构体没有域
- 这种结构体也是可以实例化的
- 通常被用来作为其他数据结构的标记类型，例如可以用来指示一个容器保存的数据类型 => 结合枚举

```rust
struct Unit;
let u = Unit;
```

### 枚举

枚举(enum) 是 和类型(sum type)，用来表示可以是多选一的数据，相对地，结构体和元组都是积类型(product type)

- 枚举的每种变体(variant)可以：
  - 没有数据(单位元变体)
  - 有命名的数据域(结构体变体)
  - 有不命名的有序数据域(元组变体)

```rust
enum Resultish {
  Ok,
  Warning { code: i32, message: String },
  Err(String)
}
```

- 枚举的变体存在于枚举本身的名字空间中：`Resultish::Ok`，可以使用`use Resultish::*`把所有变体引入当前的名字空间
- 与其他类型相同，也可以对枚举进行匹配

```rust
match make_requeast() {
  resultish::Ok => println!("Success"),
  Resultish::Warning { code, message } => println!("Warning: {}", message),
  Resultish::Err(s) => println!("Failed with error: {}", s),
}
```

- 枚举的构造器(constructor)可以像函数一样使用
- 在迭代器和闭包时很有用

#### 递归类型

考虑试图创建一种类似函数式编程的List编程

```rust
enum List {
	Nil,
	Cons(i32, List),
}
```

- 在编译时会出现无穷大小的问题
- 结构体和枚举默认情况下是内联存储的，因此不能递归。他们的元素正常情况下不使用引用来存储，但可以显示指定。

#### Box简介

- `Box<T>`是指向堆上对象的指针，作为对象的唯一所有者。Box唯一拥有它的数据(T类型)，不能创建别名。
- Box在超过作用域时会自动销毁
- 通过Box::new()来创建Box

```rust
let box_five = Box::new(5);
enum List {
  Nil,
  Cons(i32, Box<List>),
}
```

### 方法

- 结构体和枚举的方法可以实现在impl代码块里
- 和域相同，方法也通过点记号进行访问
- 可以用pub将方法声明为公开的，impl代码块本身不需要是pub的
- 对枚举和结构体是一样的

#### 方法与所有权

方法的第一个参数(名字为self)决定这个方法需要的所有权种类

- &self：方法借用对象的值。
- &num self：方法可变地借用对象的值。
- self：方法获得对象所有权。方法会消耗掉对象，同时可以返回其他的值。

#### 关联函数

- 关联函数与方法类似，但是没有self参数。调用时使用名字空间语法，Point::new()，而不是点记号。类似C++静态成员函数
- 一般会创建一个名为new的关联函数起到构造函数作用。Rust没有内置的构造函数语法，也不会自动构造。

#### 实现

- 方法、关联函数和函数不能重载
- 方法不能被继承
  - Rust中结构体和枚举用的是合成（compose）的办法
  - 特型(trait)具有基本的继承功能