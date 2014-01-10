use std::float;

struct Point {
  x: float,
  y: float,
}

enum Shape {
  Circle { center: Point, radius: float },
  Rectangle { top_left: Point, bottom_right: Point }
}

fn print_point(point: Point) {
  println!("{},{}", point.x, point.y)
}

fn square(value: float) -> float {
  value * value
}

fn area(sh: Shape) -> float {
  match sh {
    Circle { radius: radius, _ } => float::consts::pi * square(radius),
    Rectangle { top_left: top_left, bottom_right: bottom_right } => {
      (bottom_right.x - top_left.x) * (bottom_right.y - top_left.y)
    }
  }
}


fn main() {
  let circle = Circle{center: Point { x: 0f, y: 0f }, radius: 10f };
  println!("Circle area: {}", area(circle));
  let rect = Rectangle{top_left: Point { x: 0f, y: 0f }, bottom_right: Point { x: 100f, y: 200f }};
  println!("Rect area: {}", area(rect));
}

