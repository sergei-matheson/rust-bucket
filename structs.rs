//struct Point;
struct Point {
  x: float,
  y: float
}

fn print_point(point: Point) {
  // Match by dereferencing Point struct
  match point {
    // Match on x and y
    Point { x: 0.0, y: 0.0 } => { println("Dead center") }
    // Match on x, refer to (alias?) y as the_y
    Point { x: 0.0, y: the_y } => { println("The y is important: " + the_y.to_str()) }
    // Match on y, refer to x by default name
    Point { y: 2.1, x} => { println("X is important:" + x.to_str()) }
    // Match on y, ignore everything else. Interestingly, 'y' is not resolvable with the action context.
    Point { y: 10000.0, _} => { println("It's over 9000!" /* + y.to_str()) */ ) }
    // Just dereference, with aliasing
    Point { x: the_x,  y: the_y } => { println("x: " + the_x.to_str() + " y:" + the_y.to_str()); }
  }
}

fn main() {

  let points = [
    Point { x: 23.0, y: 22.0 },
    Point { x: 32.80, y: 2.1 },
    Point { x: 0.0, y: 103.23 },
    Point { x: 18.7, y: 10000.0 },
    Point { x: 0.0, y: 0.0 },
    ];

  for point in points.iter() {
    print_point(*point);
  }

}
