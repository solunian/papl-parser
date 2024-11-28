use std::io::{self, Write};
use std::fs;

fn main() {
  print!("enter a file please: ");
  std::io::stdout().flush().expect("failed to flush stdout");

  let mut fname = String::new();
  io::stdin().read_line(&mut fname).expect("failed to read line");

  println!("<{}>", fname.strip_suffix("\n").unwrap());
  std::io::stdout().flush().expect("failed to flush stdout");

  let file = fs::read_to_string(fname.strip_suffix("\n").unwrap()).expect("failed to read file");
  for line in file.split("\n") {
    println!("{}", line);
  }
}
