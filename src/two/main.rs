fn signum(x: int) -> int {
  println!("signum for :{:?}", x);
  if x < 0 { -1 }
  else if x > 0 { 1 }
  else { 0 }
}

fn main() {
  let nums = [1, 2];
  let noms = ["Tim", "Eston", "Aaron", "Ben"];

  //let mut values = nums.iter().;

  for num in nums.iter() {
    //do spawn {
      //println!("num is found");
      println(format!("num: {}", num));
      //let val = signum(num);
      //println!("num:{:?} to signum:{:?}", num, val);
    //}
  }
}
