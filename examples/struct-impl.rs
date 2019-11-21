fn test_struct(){
    #[derive(Debug)]
    struct Color (u16, u16, u16);
    let black = Color (0,0,0);
    println!{"Struct black = {:?}", black};

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8
    }
    let p1: Person = Person {name: "David".to_string(), age: 40};
    println!{"Person p1 = {:?}", p1};

    impl Person {
        // Associated Method
        fn init(name:&str, age: u8) -> Person {
            Person {
                name: name.to_string(),
                age: age
            }
        }

	// Associated Method : constructor
	fn new() -> Person {
	  let s = String::new();
	  Person { name: s, age:0 }
	}

        // Instance Method
        fn print(&self) {
            println!("Person {{ name: {}, age:{} }}",self.name, self.age);
        }
    }
    let p2 = Person::init("Steve",22);
    p2.print();
    let p3 = Person::new();
    p3.print();
}

fn main() {
  test_struct();
}
