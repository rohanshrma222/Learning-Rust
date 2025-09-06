
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

fn main(){
    let s1 = sum(1.1, 2.0);
    let s2 = sum(1 , 2);
    println!("{} {}",s1,s2);
}

fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T{
    return a + b;
}