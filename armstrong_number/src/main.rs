fn main() {
    use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    print!("Please enter a number: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    armstrong_number(s);
}
fn armstrong_number(s:String){
    let mut count =0;
    let b= s.chars().count();
    let size=b as u32;
    let num = s.parse::<u32>().unwrap();
    let char_vec: Vec<char> = s.chars().collect();
    for q in char_vec {
        let p =(q.to_string()).parse::<u32>().unwrap();
        let z = u32::pow(p,size);
        count+=z;
    
    }
    if count == num{
        println!("{} is an Armstrong number!", num);
    } else {
        println!("{} is not an Armstrong number :(", num);
    }

}
