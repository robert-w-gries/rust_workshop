
// C-like struct

#[derive(Debug)]
struct Color { 	
	r: u8,
	g: u8,
	b: u8
}

// tuple struct
#[derive(Debug)]
struct ColorTuple (u8, u8, u8); 
// tuple struct with one field -- newtype
struct Miles(u32);

// Unit struct
struct foo;

fn main() {

 let cstruct = Color {r:10,g:20,b:30};
 println!("cstruct is {} {} {}", cstruct.r, cstruct.g, cstruct.b); 
 println!("cstruct debug print {:?}", cstruct);

 let tuplestruct = ColorTuple(0,0,0);
 println!("tuplestruct is {} {} {}", tuplestruct.0, tuplestruct.1, tuplestruct.2);
 println!("tuplestruct debug print {:?}", tuplestruct);
 // destructure the tuple
 let (r, g, b) = tuplestruct;
 println!("Destructuring tuplestruct {} {} {} ", r, g, b);

 let distance = Miles(100);

 // destructure the instance of Miles 
 let Miles(dm) = distance;
 println!("Distance in miles = {}",dm); 

}
