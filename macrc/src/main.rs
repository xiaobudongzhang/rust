macro_rules! say_hello{
  () => (
     println!("hello mac")
  )
}

fn main() {
    say_hello!()
}
