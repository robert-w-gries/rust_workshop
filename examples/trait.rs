
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

struct Circle {
  radius:u32
}

impl Draw for Circle {
  fn draw(&self) {
    println!("Drawing a circle of radius {}", self.radius);
  }
}


fn main() {
  let square = Square {side:10};
  square.draw();
  
  let circle = Circle {radius:10};
  circle.draw();
}
