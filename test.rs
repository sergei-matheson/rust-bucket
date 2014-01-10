fn price(item: &str) -> float {
  match item {
    "salad" => 3.501111111,
    "muffin" => 2.258232,
    "woots" => 4.0,
    _ => 2.0 // For everything else, theres 2.0
  }
}

//TODO: ~str vs &str ?
fn number_string(number: float) -> ~str{
  match number {
    4.0 => ~"four",
    2.0 => ~"two",
    _ => format!("${:.2f}", number)
  }
}

fn main() {
  let items = ["salad", "muffin", "woots", "coat-rack"];
  for item_ptr in items.iter() {
    //TODO: why is this necessary? Is there a for loop construct that dereferences?
    let item = *item_ptr;
    let price = price(item);
    println!("Bob, the price for {item} is {price}", price = number_string(price), item = item) }
}
