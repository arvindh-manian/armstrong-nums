//armstrong number checker
//by Arvindh Manian

#[allow(dead_code)]
fn main() {

    use std::io;



    //get input from user
    println!("Enter a number:");
    let mut i = String::new();
    io::stdin().read_line(&mut i).expect("Failed to get console input");


    //process input
    let i = i.trim().parse::<i32>().unwrap();
    let string_i = i.to_string();
    let length = string_i.chars().count();
    let mut sum = 0;
    let digits: Vec<_> = string_i.chars().map(|d| d.to_digit(10).unwrap()).collect();

    //add sum of digits to the power of length
    for y in digits {
        sum += i32::pow(y as i32, length as u32);
    }
    //print
    if sum == i {
        print!("{}{}", i, " is an Armstrong number!");
    }
    else {
        print!("{}{}", i, " is not an Armstrong number!");
    }
}