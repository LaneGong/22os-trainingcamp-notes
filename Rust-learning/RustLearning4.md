# Rust学习笔记4

> 参考资料：
>
> - [清华计算机系大一学生2022暑期课程：Rust程序设计训练](https://lab.cs.tsinghua.edu.cn/rust/)
> - [Rust语言圣经(Rust Course)](https://course.rs/about-book.html)
> - [rustlings](https://github.com/rust-lang/rustlings)

## 模式匹配

### 结构体的匹配

- match语句对结构体进行解构

  - 匹配时域不一定要按照结构体声明时的顺序
  - 将结构体的域列出来，和对应的变量名做绑定(struct_field: new_var_binding)
  忽略部分域：使用..忽略所有没有提到名字的域

### 以引用方式匹配

- 使用ref可以在匹配时获得一个变量的引用(否则是绑定，直接获得所有权)
- 使用 ref mut 以可变引用的方式进行匹配(仅当被匹配的对象是 mut 的)。

```rust
let x = 17;
match x {
	ref r => println!("Of type &i32: {}", r),
}


let mut x = 17;
match x {
	ref r if x == 5 => println!("{}", r),
	ref mut r => *r = 5
}
```

### if let语句/while let 语句

如果只需要单个匹配分支，用 if let 语法会比较便捷。

```rust
enum Resultish {
	Ok,
	Warning { code: i32, message: String },
	Err(String),
}
//原先写法：假设希望在出错时报告错误，其他情况什么也不做。
match make_request() {
	Resultish::Err(_) => println!("Total and utter failure."),
	_ => println!("ok."),
}

//改进
if let Resultish::Err(s) = make_result() {
	println!("Total and utter failure: {}", s);
} else {
	println!("ok.");
}
```

还有一个类似的 while let 语句，作用是循环迭代直至匹配条件失败。

```rust
let mut v = vec![1, 2, 3];
while let Some(x) = v.pop() {
	println!("{}", x);
}
```

### 内部绑定

对于更复杂的数据结构，可以用 @ 创建内部元素的变量绑定。

```rust
#[derive(Debug)]
enum A { None, Some(B) }
#[derive(Debug)]
enum B { None, Some(i32) }
fn foo(x: A) {
	match x {
		a @ A::None => println!("a is A::{:?}", a),
		ref a @ A::Some(B::None) => println!("a is A::{:?}", *a),
		A::Some(b @ B::Some(_)) => println!("b is B::{:?}", b),
	}
}

//result
foo(A::None); // ==> x is A::None
foo(A::Some(B::None)); // ==> a is A::Some(None)
foo(A::Some(B::Some(5))); // ==> b is B::Some(5)
```

### 模式匹配说明

- 穷尽性：match 的所有分支对于模式来说必须是穷尽的。
- 模式匹配的使用场合：
  - 模式匹配语句 match 
  - 变量绑定，包括 let、if let、while let 
  - for 循环 
  - 函数和闭包的参数

举例：

```rust
//在 for 循环的 for 和 in 之间描述循环变量时可以使用模式匹配。
let v = vec![1, 2, 3];
for (i, x) in v.iter().enumerate() {
	println!("v[{i}] = {x}");
}
//描述函数参数时可以使用模式匹配。
fn tuple_add((a, b): (i32, i32)) -> i32 {
	a + b
}
fn main() {
	tuple_add((1, 2));
}
```

