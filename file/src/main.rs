static LOREM_IPSUM: &'static str =
"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";



use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/*fn main() {
   let path = Path::new("hello.txt");
   let display = path.display();
   
   let mut file = match File::open(&path) {
       Err(why) => panic!("can't open {}:{}", display, why.description()),
       Ok(file) => file,
   };

   let mut s = String::new();
   match file.read_to_string(&mut s){
   	 Err(why) => panic!("can't    read {}:{}", display, why.description()),
	 Ok(_) => print!("{} contains\n{}", display,s),
	
   }    

}*/



fn main(){
   let path = Path::new("h.txt");
   let display = path.display();

   let mut file = match File::create(&path){
       Err(why) => panic!("can't create {}:{}", display, why.description()),
       Ok(file) => file,
   };
   
   match file.write_all(LOREM_IPSUM.as_bytes()){
   	  Err(why) => panic!("can't write {}:{}", display, why.description()),
	  Ok(_) => println!("success wrote to {}", display),
   }

}