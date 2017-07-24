use std::io;
extern crate rand;
use rand::Rng;
use std::cmp::Ordering;
use std::mem;

fn main() {   
    //guess_the_number();
    //struct_enum_option(); 
    //array();
    /*** Ownership And Borrow ***/
    let mut s2 = get_ownership();
    borrowed_as_read_only(&s2);
    borrowed_as_write_read(&mut s2);
    move_ownership(s2);
}
#[allow(dead_code)] 
fn get_ownership() -> String {
    let mut s1 : String = String::from("Hello, ");
    s1.push_str("Ownership");
    s1
}
#[allow(dead_code)] 
fn borrowed_as_read_only(s3 : &String){
    println!("{}", s3);
}
#[allow(dead_code)] 
fn borrowed_as_write_read(s4: &mut String) {
    s4.push_str(" Changed!");
    println!("{}", s4);
}
#[allow(dead_code)] 
fn move_ownership(mut s5 : String) {
    s5.push_str("!!");
    println!("{}", s5.replace("Ownership Changed!!!", "Own Memory Dropped!"));
}
#[allow(dead_code)] 
fn array() {
    #[derive(Debug)]
    let mut a: [i8; 5] = [1,3,5,7,9]; 
    a[2] = 55;
    for i in 0..a.len() {
        println!("a = {}", a[i]);
    }
    let b = [7u8;10];
    println!("b = {} : {:?}, b took up {} bytes", b.len(), b, mem::size_of_val(&b));
    let c:[[f32;4];3] = [[1.0;4],[2.0;4],[3.0;4]];
    println!("c = {:?}", c);
}
#[allow(dead_code)] 
fn struct_enum_option(){
    #[derive(Debug)]
    enum ShapeEnum {
        Rectangle,
        Square,
        Square2(i32),
    }
    impl ShapeEnum {
        fn default_area(&self) -> i32 {
            i32::pow(7, 2)
        }
    }
    struct Rectangle {
        shape_type: ShapeEnum,
        width: i32,
        height: i32,
    }
    impl Rectangle {
        fn area(&self) -> i32 {
            self.width * self.height
        }
    }
    struct Square {
        shape_type: ShapeEnum,
        width_or_height: i32,
    }
    let rectangle : Rectangle = Rectangle { shape_type: ShapeEnum::Rectangle, width: 30, height: 40, };
    let result: Option<i32> = if rectangle.width != 0 && rectangle.height != 0  
                                { Some(rectangle.area()) } else { None };
    match result {
        Some(v) => println!("Rectangle: Width {}, Height {}, Area {}", rectangle.width, rectangle.height, v),
        None => println!("Undefined"),
    }
    let square_width_or_height = ShapeEnum::Square2(45);
    println!("{:?} : {:?}", square_width_or_height, square_width_or_height.default_area());
}
#[allow(dead_code)] 
fn guess_the_number(){
    let secret_number : i8 = rand::thread_rng().gen_range(5,10);
    println!("Enter your guess: ");
     let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read");
    println!("You guessed : {}, Press x to exit", guess.trim());
    //let guess: i8 = guess.trim().parse().expect("InvalidDigit or Overflow");
    let guess: i8 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => 7,
    };
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Less"),
        Ordering::Equal => println!("Equal"),
        Ordering::Greater => println!("Greater"),
    }
    for i in 1..secret_number {
        let mut quit = String::new();
        io::stdin().read_line(&mut quit).expect("Failed to read");
        if quit.trim() == "x" {
            print!("Good Bye...");
            break;
        }
        println!("Failed : {0}. You wrote : {1}, Press x to exit", i, quit.trim());
    }
}