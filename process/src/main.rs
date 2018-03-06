use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};


static PANGRAM: &'static str =
"the quik";
fn main() {
   let process = match Command::new("wc")    
       	       	       .stdin(Stdio::piped())
		       .stdout(Stdio::piped())
		       .spawn(){
			
		Err(why) => panic!("can't spawn wc:{}", why.description()),
		Ok(process) => process,
			
		       };

		       
  //stdin process
  match process.stdin.unwrap().write_all(PANGRAM.as_bytes()){
  	Err(why) => panic!("cant write to wc stdin:{}", why.description()),
	Ok(_) => println!("sent pangram to wc"),
  }

  //stdout process
  let mut s = String::new();
  match process.stdout.unwrap().read_to_string(&mut s){
  	Err(why) => panic!("cant read wc stdout :{}", why.description()),
	Ok(_) => print!("wc responed with:\n{}", s)
  }
}
