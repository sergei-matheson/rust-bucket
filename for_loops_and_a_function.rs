fn is_four(x: float) -> bool {
  x == 4.0
}

fn main() {
  static MONSTER_FACTOR: float = 57.8;
  let items = ["salad", "muffin", "woots", "coat-rack"];
  for item_ptr in items.iter() {
    //TODO: why is this necessary? Is there a for loop construct that dereferences?
    let item = *item_ptr;
    let price =
      if item == "salad" {
        3.501111111
      } else if item == "muffin" {
        2.258232
      } else if item == "woots" {
        4.0
      } else {
        2.0
      };
    if is_four(price) { println!("{} is four!", item) }
    else { println!("Bob, the price for {item} is {price:.2f}", price = price, item = item) }
  }
}
