use rand::Rng;
use std::io;
// use std::cmp::Ordering;

type t = (i32, f64);

fn main() {
    // ---------2023/10/28------------
    // --snip--
    //     let mut rng_number = rand::thread_rng().gen_range(1..=100);
    //     let mut guess = String::new();

    //     println!("輸入數字");
    //     io::stdin().read_line(&mut guess);
    //     println!("You guessed: {guess}");
    //     let guess: i32 = {
    //         eprintln!("Guess variable contents: {:?}", guess);
    //         guess.parse().expect("msg is not")
    //     };
    //     println!("You guessed:{} ", guess);
    //     // match guess.cmp(&rng_number) {
    //     //     Ordering::Less => println!("Too small!"),
    //     //     Ordering::Greater => println!("Too big!"),
    //     //     Ordering::Equal => println!("You win!"),
    //     // }
    // --------------------------------
    let tup: t = (500, 6.14);
    let (x, y) = tup;
    let a = tup.0;
    print!("輸出Y: {y}");
    print!("輸出X: {a}");
    let d: [i32; 5] = [3; 5]; //array [ type : length]
    let number = 13;
    let f = if number < 0 { 1 } else { 2 };
}
