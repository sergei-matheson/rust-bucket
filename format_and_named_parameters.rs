fn main() {
  /* A simple loop */
  let hi = "Hello";
  let mut count = 0;
  while count < 10 {
    println!("{greeting} Bob, this is the value: {value}", greeting = hi, value=count);
    count += 1;
  }
}
