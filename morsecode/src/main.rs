fn main() {
    use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    print!("Please enter some text in Morse code or English: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    let mut num=String::new();
    print!("Press 1 to change to Morse code and 2 to change to English: ");
    let _=stdout().flush();
    stdin().read_line(&mut num).expect("Did not enter a correct string");
    if let Some('\n')=num.chars().next_back() {
        num.pop();
    }
    if let Some('\r')=num.chars().next_back() {
        num.pop();
    }
    let c = num.parse::<i32>().unwrap();
    if c==1{
        to_morse_code(s);
    } else{
        to_english(s);
    }
    
    
}
fn to_morse_code(s:String){
    let char_vec: Vec<char> = s.chars().collect();
    for c in char_vec {
        if c =='A'|| c=='a'{
            print!(".- ");
        }else if c =='B'|| c=='b'{
            print!("-... ");
        }else if c =='C'|| c=='c'{
            print!("-.-. ");
        }else if c =='D'|| c=='d'{
            print!("-.. ");
        }else if c =='E'|| c=='e'{
            print!(". ");
        }else if c =='F'|| c=='f'{
            print!("..-. ");
        }else if c =='G'|| c=='g'{
            print!("--. ");
        }else if c =='H'|| c=='h'{
            print!(".... ");
        }else if c =='I'|| c=='i'{
            print!(".. ");
        }else if c =='J'|| c=='j'{
            print!(".--- ");
        }else if c =='K'|| c=='k'{
            print!("-.- ");
        }else if c =='L'|| c=='l'{
            print!(".-.. ");
        }else if c =='M'|| c=='m'{
            print!("-- ");
        }else if c =='N'|| c=='n'{
            print!("-. ");
        }else if c =='O'|| c=='o'{
            print!("--- ");
        }else if c =='P'|| c=='p'{
            print!(".--. ");
        }else if c =='Q'|| c=='q'{
            print!("--.- ");
        }else if c =='R'|| c=='r'{
            print!(".-. ");
        }else if c =='S'|| c=='s'{
            print!("... ");
        }else if c =='T'|| c=='t'{
            print!("- ");
        }else if c =='U'|| c=='u'{
            print!("..- ");
        }else if c =='V'|| c=='v'{
            print!("...- ");
        }else if c =='W'|| c=='w'{
            print!(".-- ");
        }else if c =='X'|| c=='x'{
            print!("-..- ");
        }else if c =='Y'|| c=='y'{
            print!("-.-- ");
        }else if c =='Z'|| c=='z'{
            print!("--.. ");
        }else if c =='1'{
            print!(".---- ");
        }else if c =='2'{
            print!("..--- ");
        }else if c =='3'{
            print!("...-- ");
        }else if c =='4'{
            print!("....- ");
        }else if c =='5'{
            print!("..... ");
        }else if c =='6'{
            print!("-.... ");
        }else if c =='7'{
            print!("--... ");
        }else if c =='8'{
            print!("---.. ");
        }else if c =='9'{
            print!("----. ");
        }else if c =='0'{
            print!("----- ");
        }else if c ==' '{
            print!("\\ ");
        }else{
            print!(" ");
        }
        
       
    }
}
fn to_english(s:String){
    for k in s.split_whitespace() {
        if k.trim() ==".-"{
            print!("A");
        }else if k.trim()=="-..."{
            print!("B");
        }else if k.trim()=="-.-."{
            print!("C");
        }else if k.trim()=="-.."{
            print!("D");
        }else if k.trim()=="."{
            print!("E");
        }else if k.trim()=="..-."{
            print!("F");
        }else if k.trim()=="--."{
            print!("G");
        }else if k.trim()=="...."{
            print!("H");
        }else if k.trim()==".."{
            print!("I");
        }else if k.trim()==".---"{
            print!("J");
        }else if k.trim()=="-.-"{
            print!("K");
        }else if k.trim()==".-.."{
            print!("L");
        }else if k.trim()=="--"{
            print!("M");
        }else if k.trim()=="-."{
            print!("N");
        }else if k.trim()=="---"{
            print!("O");
        }else if k.trim()==".--."{
            print!("P");
        }else if k.trim()=="--.-"{
            print!("Q");
        }else if k.trim()==".-."{
            print!("R");
        }else if k.trim()=="..."{
            print!("S");
        }else if k.trim()=="-"{
            print!("T");
        }else if k.trim()=="..-"{
            print!("U");
        }else if k.trim()=="...-"{
            print!("V");
        }else if k.trim()==".--"{
            print!("W");
        }else if k.trim()=="-..-"{
            print!("X");
        }else if k.trim()=="-.--"{
            print!("Y");
        }else if k.trim()=="--.."{
            print!("Z");
        }else if k.trim()==".----"{
            print!("1");
        }else if k.trim()=="..---"{
            print!("2");
        }else if k.trim()=="...--"{
            print!("3");
        }else if k.trim()=="....-"{
            print!("4");
        }else if k.trim()=="....."{
            print!("5");
        }else if k.trim()=="-...."{
            print!("6");
        }else if k.trim()=="--..."{
            print!("7");
        }else if k.trim()=="---.."{
            print!("8");
        }else if k.trim()=="----."{
            print!("9");
        }else if k.trim()=="-----"{
            print!("0");
        }else if k.trim()=="\\"{
            print!(" ");
        }
    }
}

