use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
   
   let (tx,rx)=mpsc::channel();   
   
   let tx1 = tx.clone();
   thread::spawn(move||{

	let vals = vec![
	    String::from("hi"),
	    String::from("from"),
	    String::from("the"),
	    String::from("thread")
	];

	for val in vals{
	    tx1.send(val).unwrap();
	    thread::sleep(Duration::new(1, 0));
	    
	}
	 
    });


    thread::spawn(move||{
	
	let vals = vec![
	    String::from("more"),
	    String::from("messages"),
	    String::from("for"),
	    String::from("your"),
	];
	
	for val in vals {
	    tx.send(val).unwrap();
	    thread::sleep(Duration::new(1, 0));
	}
    });
    
    for received in rx{
    
	println!("Go :{}", received);
    }
    
}
