use std::float;

struct Point {
  x: float,
  y: float,
}

enum Shape {
  Circle(Point, float),
  Rectangle(Point, Point)
}

fn print_point(point: Point) {
  println!("{},{}", point.x, point.y)
}

#[no_mangle]
fn determine_area(shape: Shape) -> float {
  match shape {
    Circle(_, radius) => float::consts::pi * radius * radius,
    //NB: you don't have to break it down to the individual point properties
    //Rectangle(top_left, bottom_right) => { print_point(top_left); print_point(bottom_right); 30.0 }
    Rectangle(Point {x: x1, y: y1}, Point {x: x2, y: y2}) => { (x2 - x1) * (y2 - y1) }
  }
}

fn main() {
  let point = Point {x:3f, y:7f};
  print_point(point);
  let circle = Circle(Point { x: 0f, y: 0f }, 10f);
  println!("Circle area: {}", determine_area(circle));
  let rect = Rectangle(Point { x: 0f, y: 0f }, Point { x: 100f, y: 200f });
  println!("Rect area: {}", determine_area(rect));
}
