#![allow(dead_code)]
#[derive(Debug)]
 enum Shape {
   Square,
   Rectangle,
   Circle,
   Hexagaon
 }

fn is_even(num:i32)->Option<bool> {
   if num%2 == 0 {
      Some(true)
   } else {
      None
   }
}

#[derive(Debug)]
enum Answer {
  Yes,
  No,
  Result {x:i64, y:i64},
  Response (String)
}

fn analyze_response (answer:Answer) {
 println!("Analyzing response {:?}", answer); 
  match answer {
    Answer::Yes => println!(" Yes"),
    Answer::No => println!(" No"),
    Answer::Result {x, y} => println! (" result is x = {}, y = {}", x, y),
    Answer::Response(s) => println!(" Result string is {}",s),
  }
}

fn main() {
   let e:Shape = Shape::Square;
   println!("value of enum e is {:?}",e);

   let mut  a:Answer = Answer::Result{x:10, y:20};
   analyze_response(a);
   a = Answer::Response("Hello".to_string());
   analyze_response(a);

   let result = is_even(3);
   println!("{:?}",result);
   println!("{:?}",is_even(30));
}

