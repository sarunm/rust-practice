fn main() {
    println!("Hello, crabby!");

    let mut n = 10;
    n = 99;
    println!("n is {}", n);

    let x = 5;
    let y = 10.0;
    let z = x + y as i32;

    println!("z is {}", z);

    let msg1 = String::from("Hello, world!");
    let msg2  = "Hello, world!".to_string();
    let msg3 = "Hello, world!";
    let msg4 = format!("Point: {},{}",x,y);

    println!("msg4 is {}", msg4);
}


