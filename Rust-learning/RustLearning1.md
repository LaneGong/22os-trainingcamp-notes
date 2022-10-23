# Rust学习笔记1

## 本地运行环境+Github Classroom

1. 按照[指示](https://github.com/LearningOS/rust-based-os-comp2022/blob/main/scheduling.md) a、b、c步骤创建好基于rustlings加githubclassroom的实验仓库
2. 在本地执行`git clone`（建议ssh）
3. `cd 实验仓库` 并执行如下命令 `make setupclassroom` 完成自动评分配置

![image-20221021130424443](../Fig/rust1.png)

> ⚠️本地已有rust环境，没有可以自己装。在线环境可以执行`sudo make codespaces_setenv`

4. 基于上述步骤，在本地完成几个实验即可push到仓库中 => 实测应该是要全部完成之后才会有评分(应该是`rustlings watch`导致的，直接循环所有实验)

   ![image-20221023134710495](../Fig/rust2.png)

## Rust语言

### 学习资源

- [清华计算机系大一学生2022暑期课程：Rust程序设计训练](https://lab.cs.tsinghua.edu.cn/rust/)
- [Rust语言圣经(Rust Course)](https://course.rs/about-book.html)
- [rustlings](https://github.com/rust-lang/rustlings)

### 目标

构建可靠且高效的软件

### 特性

- 高效performance：没有运行时和垃圾收集器，代码的运行速度快，内存使用效率高，可用来开发对性能要求高的服务。
- 可靠reliability：用类型系统和所有权模型来确保内存安全性和线程安全性，在编译时消除各种潜在的问题。
- 好用productivity=>多产：有丰富的文档、有好的编译器（提供有用的错误信息)和一流的工具集，包括集成的包管理器和构建工具、支持各种编辑器的代码自动补全和类型查看功能、代码自动格式化工具等

### 使用场景

- 命令行工具
- 网页应用
- 网络服务
- 嵌入式开发

### Rust编写的软件

- Rust语言工具链
- Servo浏览器引擎
- Redox操作系统
- Linux内核正在加入用Rust语言写驱动和模块的支持
- exa、bat、fd等命令行工具
- rCore教学操作系统
- MadFS文件系统

### 初识

```rust
fn main() { //fn => function
	println!("Hello,world!");//输出（! => 宏调用）
}
//c++中#include,#define也是宏
//宏是在编译之前就做了
```

单文件编译运行：

- `rustc hello.rs`
- `./hello`

对于项目：

1. `cargo run`

2. `cargo bulid` + `./hello`

> 项目默认运行的是 `debug` 模式，在这种模式下，**代码的编译速度会非常快**，可是福兮祸所依，**运行速度就慢了**. 原因是，在 `debug` 模式下，Rust 编译器不会做任何的优化，只为了尽快的编译完成，让你的开发流程更加顺畅。
>
> 要性能 => `--release`
>
> https://course.rs/first-try/cargo.html

创建项目：

- `cargo new name`

### 入门小项目

![image-20221023143118011](../Fig/rust3.png)

```rust
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess Number Start!");
    let secret_number = rand::thread_rng().gen_range(1..=100);//rng => random number generator
    loop {
        println!("Please input a number:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("read line failed");
        let guess: u32 =match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("The number you input is {guess}.");
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

> 依赖一个软件要指定他的版本号的意义？包会更新（虽然最好是能够向后兼容），但是万一没有，新版本中有些旧功能被移除或变化，导致项目依赖出问题而不能运行。

```toml
[dependencies]
rand = "0.8.3"
```

### 基本语法-简

