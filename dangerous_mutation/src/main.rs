fn main() {
  let mut buffer: String = format!("Rustacean");
  {
    let slice = &buffer[1..];
    println!("{:?}", slice);
  }
  buffer.push_str("s");
  println!("{:?}", buffer);
}
