trait Monster {
  fn attack(&self) {
    println!("{name} attacks for {damage:d} damage!", name = self.name(), damage = self.attack())
  }
  fn name(&self);
  fn health(&self);
  fn attack(&self);
}

impl Monster {
  fn new(name: ~str, health: int, attack: int) -> Monster {
    Monster { name: name, health: health, attack: attack }
  }

  fn attack(&self) {
    println!("{name} attacks for {damage:d} damage!", name = self.name, damage = self.attack)
  }

  fn count() {
    println!("There are a whole bunch of monsters out tonight")
  }
}

fn main() {

  let monster = Monster::new(~"Wilbur", 10, 20);

  println!("{:?}", monster);

  monster.attack();
  Monster::count();
}
