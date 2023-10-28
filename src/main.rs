// use std::io;
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
    }
}

fn main() {
    //     let name = "Alice";
    //     let age = 30;
    //     let greeting = format!("Hello, my name is {name} and I am {age} years old."); //format! 可讓變數與字串拼接
    //     println!("{}", greeting); //println! 輸出到命令列,且帶換行格式
    //     print!("{}", greeting); //print! 輸出到命令列
    //     eprintln!("{greeting}"); //輸出到錯誤列,且帶換行格式
    //     eprint!("{greeting}"); //輸出到錯誤列

    //================================================================
    let p = Point { x: 3, y: 5 };
    println!("Debug format: {:?}", p);
}
