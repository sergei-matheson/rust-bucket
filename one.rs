fn main() {
  let nums = [1, 2];
  let noms = ["Tim", "Eston", "Aaron", "Ben"];

  let mut odds = nums.iter().map(|&x| x * 2 - 1);

  //for num in odds {
  for num in odds {
    println!("num:{:i}", num)
    //do spawn {
    //  println!("{:s} says hello from a lightweight thread!", noms[num]);
    //}
  }
}
