fn main() {
  // wait for 3 seconds
  std::thread::sleep(std::time::Duration::from_secs(3));

  // NOTE: The exclamation mark is a way to say that we are calling a macro and not a function
  println!("Hello, world!");
}