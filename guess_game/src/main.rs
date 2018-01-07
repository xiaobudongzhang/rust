extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1,101);
    
    //println!("the secret number is :{}",secret_number);
 
 loop{
    println!("please input your guess");
    
    let mut guess = String::new();
    
    io::stdin().read_line(&mut guess)
    		    .expect("fail to read");
		    
    let guess: u32 = match guess.trim().parse(){
    	Ok(num) => num,
	Err(_) => continue,
    };   

   println!("your guess:{}",guess);   
   match guess.cmp(&secret_number){
   	 Ordering::Less => println!("Too small"),
	 Ordering::Greater => println!("Too big"),
	 Ordering::Equal => {
	 		    println!("You win");
			    break;
			 }
   }
 }
}
