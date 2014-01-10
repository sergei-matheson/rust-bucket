fn main() {
  for num in range(0, 100) {
    do spawn {
      println!("Hello to {}", num);
      println!("Hello again to {}", num);
    }
  }
}
