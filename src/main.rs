fn main() {
    data_types();
    poc_if();
    poc_match();
    poc_loops();
}

fn data_types() {
    let x = 5;
    let y = 10.0;
    let z = x + y as i32;

    println!("z is {}", z);

    // let msg1 = String::from("Hello, world!");
    // let msg2 = "Hello, world!".to_string();
    // let msg3 = "Hello, world!";
    let msg4 = format!("Point: {},{}", x, y);

    println!("msg4 is {}", msg4);
}

fn poc_if() {
    let check = 50;

    if check > 100 {
        println!("check is greater than 100");
    } else if check < 100 {
        println!("check is less than 100");
    } else {
        println!("check is equal to 100");
    }
}

fn poc_match() {
    let test_string = "test1";

    match test_string {
        "test1" => println!("test1"),
        "test2" => println!("test2"),
        _ => println!("default"),
    }
}

fn poc_loops() {
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count;
        }
    };
    println!("result is {}", result);
}
