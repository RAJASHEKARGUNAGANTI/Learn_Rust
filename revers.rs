use std::io;
fn main(){
    println!("Enter the number");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut num:i32 = match input.trim().parse(){
        Ok(n) => n,
        Err(_) =>{
            println!("Invalid input type");
            return;
        }
    };
    let mut rev: i32 = 0;
    let mut rem: i32 ;
    loop{
        rem = num % 10;
        rev = rev *10 + rem;
        num /= 10;
        if num == 0{
            break;
        }
    }
    
    println!("Reverse of given num is {}",rev);
}