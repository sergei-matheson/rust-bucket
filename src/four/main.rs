fn woot(x: uint) -> uint {
  23 * x
}

fn main() {
  let mut x = 25u;
  loop {
    x += x - 3;
    let stuff = x;
    do spawn {
      println(woot(stuff).to_str());
    }
    if x % 5 == 0 { println("Found it!"); break; }
  }
}
