
// fn main(){
//     let s = sum(1,2);
//     println!("{}",s);
// }

// fn sum(a: u32, b: u32) -> u32 {
//     return a + b;
// }



// use chrono::{Utc,Local};

// fn main() {
//     let utc = Utc::now();
//     let local_time =Local::now();
//     print!("{}", utc);
//     print!("{}", local_time);
// }



// use dotenv::dotenv;
// use std::env;

// fn main(){
//     dotenv().ok();
//     let var = env::var("REDIS_ADDRESS");

//     match var{
//         Ok(str) => println!("{}",str),
//         Err(_e) => print!("Error while reading variable")
//     }
// }



// use dotenv::dotenv;
// use std::env;

// fn main(){
//     dotenv().ok();
//     let redis_url = env::var("REDIS_ADDRESS");
//     let redis_url_result =  redis_url.unwrap();

//     print!("{}", redis_url_result);
// }



//GENERICS AND TRAITS

// fn main(){
//     let s1 = sum(1.1, 2.0);
//     let s2 = sum(1 , 2);
//     println!("{} {}",s1,s2);
// }

// fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T{
//     return a + b;
// }



// struct Rect {
//     width  : u32,
//     height : u32
// }

// impl Rect {
//     fn area(&self) -> u32 {
//         return self.width * self.height
//     }
// }

// fn main(){
//     let r = Rect {
//         width  : 10,
//         height : 10
//     };

//     print!("{}", r.area());
// }



// struct Rect<T> {
//     width  : T,
//     height : T
// }

// impl <T: std::ops::Mul<Output = T> + Copy> Rect <T> {
//     fn area(&self) -> T {
//        return self.width  * self.height
//       }    
// }

// fn main(){
//     let r = Rect {
//         width  : 10,
//         height : 10
//     };
//     print!("{}", r.area());
//




// use std::f32::consts::PI;
// trait Shape {
//     fn area(&self) -> f32;
// }

// struct Rect {
//     width  : f32,
//     height : f32
// }

// struct Circle {
//     radius: f32,
// }

// impl Shape for Circle {
//     fn area(&self) -> f32{
//         return PI * self.radius * self.radius
//     }
// }

// impl Shape for Rect {
//     fn area(&self) -> f32 {
//         return self.width * self.height
//     }
// }

// fn print_area_of_shape<T: Shape>(s: T) {
//     println!("{}", s.area());
// }

// fn main(){
//     let r = Rect {
//         width : 10.0,
//         height: 10.0
//     };

//     let c = Circle {
//         radius: 10.0
//     };

//     print_area_of_shape(c);
//     print_area_of_shape(r);
// }



//Procedural Macros

use std::{fmt::format, path::Display};

#[derive(Debug)]
struct User {
    username: String,
    password: String,
    age: u32
}
fn main(){
    let u = User {
        username: String::from("Rohan"),
        password: String::from("Rohan"),
        age: 32
    };

    print!("{:?}", u);
}