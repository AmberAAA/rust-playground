use core::fmt;
use std::mem;
use crate::List::*;

fn main() {
    println!("Amber");
    sta_out();
    learn_arr_and_slice();
    var_bindings();
}

// struct Unprintable(i32);

#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

/// 标准化输出
fn sta_out() {
    println!("Hello World!");

    println!("{} days", 12);

    println!("{1} is {0}", "Hello", "World!");

    // 没有#[derive(debug)] 则不能在{:?}中被打印
    // println!("{:?} is Unprintable", Unprintable(32));
    println!("{:?} is Debug Print Able", DebugPrintable(32));

    let name = "Amber";
    let age = 12;

    let person = Person { name, age };

    println!("{:#?}", person);

    let left_top = Point { x: 0.0, y: 100.0 };
    let right_bottom = Point { x: 100.0, y: 0.0 };
    let rect = Rectangle {
        left_top,
        right_bottom,
    };
    println!("{}", rect.area());

    inspect(WebEvent::Click { x: 32, y: 32 });
    inspect(WebEvent::PageUp);
    inspect(WebEvent::KeyPress('A'));
    inspect(WebEvent::Paste("Hello World".to_string()));
    inspect(WebEvent::PageDown);

    emit_web_event();

    emit_web_event();

    println!("{:?}", learn_tuples((1, false)));
}

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    left_top: Point,
    right_bottom: Point,
}

impl Rectangle {
    fn area(&self) -> f32 {
        (self.right_bottom.x - self.left_top.x) * (self.left_top.y - self.right_bottom.y)
    }
}

enum WebEvent {
    PageDown,                 // 单元结构体
    PageUp,                   // 单元结构体
    KeyPress(char),           // 元祖结构体
    Paste(String),            // 元祖结构体
    Click { x: i64, y: i64 }, //  普通结构体行
}

enum VideoType {
    MP4 = 3,
    FLV,
    MOV,
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageUp => println!("PageUp"),
        WebEvent::PageDown => println!("PageDown"),
        WebEvent::KeyPress(c) => println!("pressed in {}", c),
        WebEvent::Click { x, y } => println!("point x: {}, y: {}", x, y),
        WebEvent::Paste(c) => println!("paste is {}", c),
    }
}

fn emit_web_event() {
    use WebEvent::*;
    inspect(PageDown);
    inspect(PageUp);
    inspect(Click { x: 32, y: 32 });
}

// Tuples 元祖
// 一般我们也使用元祖返回多个值

fn learn_tuples(pair: (i32, bool)) -> (i32, bool) {
    // 元祖解构
    let (a, b) = pair;

    let matrix = Matrix(1.1, 1.2, 3.2, 3.2);
    
    println!("{}", transpose(&matrix));
    
    println!("Count is: {}", matrix.add());

    println!("Matrix: \n{}", Matrix(1.1, 1.2, 3.2, 3.2));

    (a + 1, !b)
}

struct Matrix(f32, f32, f32, f32);

impl Matrix {
    fn add(&self) -> f32 {
      self.0 + self.1 + self.2 + self.3
    }
}

// 实现Display
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}) \n({}, {})", self.0, self.1, self.2, self.3)
    }
}

fn transpose(m: &Matrix) -> Matrix {
  Matrix(m.0, m.2, m.1, m.3)
}


// 数组与切片
fn analyze_slice (slice: &[i32]) {
  println!("first element is {}", slice[0]);
  println!("the slice has {} elements", slice.len())
}

fn learn_arr_and_slice() {
  // [type; size] 左边类型；右边大小；初始化时，必须填满；
  let xs: [i32; 5] = [1,2,3,4, 5];
  analyze_slice(&xs);

  // 500个元素全部用0填满
  let ys: [i32; 500] = [0; 500];

  // 可以通过mem获取占用的内存大小
  println!("array occupies {} bytes.", mem::size_of_val(&ys));

  // 将整个数组当当成切片
  analyze_slice(&ys);

  // 中间两个点表示 从0开始 在第1个元素之前
  analyze_slice(&xs[0 .. 1]);

  // 尝试超界访问
  // 当尝试超界访问时，会直接 panicked
  // analyze_slice(&xs[3 .. 6]);


  let empty_array: [u32; 0] = [];
  assert_eq!(&empty_array, &[]);
  assert_eq!(&empty_array, &[][..]);

  // 数组的安全访问
  for i in 0..xs.len()+1 {
    match xs.get(i) {
        Some(xval) => println!("item is {}", xval),
        None => println!("None") 
    }
  }

  // println!("Hello World!")
}


enum List {
    Cons(u32, Box<List>),
    Nil
}

impl List {
    fn new() -> List {
        List::Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new (self))
    }
}


fn var_bindings () {
    let name = "Amber";
    println!("{}", name);

    // 以_开头的未使用变量不会触发告警
    let _unused_val = 1;
    let noisy_unused_val = 2;

    let mut name = 23;
    name = 32;
}

// https://doc.rust-lang.org/stable/rust-by-example/types/cast.html
