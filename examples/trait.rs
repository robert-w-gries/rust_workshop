
pub trait Draw {
  fn draw(&self); 
}

struct Square {
 side: u32 
}

impl Draw for Square {

  fn draw(&self) {
    println!("Drawing a square of side {}",self.side);
  }
}

#[derive(Debug)]
struct Circle {
  radius:u32,
  x: f64,
  y: f64
}

impl Draw for Circle {
  fn draw(&self) {
    println!("Drawing a circle of radius {:?}", self);
  }
}


fn main() {
  let square = Square {side:10};
  square.draw();
  
  let circle = Circle {radius:10, x:0.0, y:0.0};
  circle.draw();
}
