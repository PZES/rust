fn main() {
    let mut num=String::new();
    use std::io::{stdin,stdout,Write};
    print!("Type in a year: ");
    let _=stdout().flush();
    stdin().read_line(&mut num).expect("Did not enter a correct string");
    if let Some('\n')=num.chars().next_back() {
        num.pop();
    }
    if let Some('\r')=num.chars().next_back() {
        num.pop();
    }
    let s = num.parse::<u32>().unwrap();
    leap_year(s);

}
fn leap_year(s:u32){
    if s%100==0{
        if s%400==0{
            println!("This is a leap year!");
        }else{
            println!("This is not a leap year :(");
        }
    }else if s%4==0{
        println!("This is a leap year!");
    } else{
        println!("This is not a leap year :(");
    }

}
