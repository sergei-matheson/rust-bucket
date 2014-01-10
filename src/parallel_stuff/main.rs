fn main() {
  do 100.times {
    do spawn {
      let greeting_message = "Hello?";
      println(greeting_message);
    }
  }
}
