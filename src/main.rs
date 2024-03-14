// fn main() { 
//     let n: Vec<i32> = vec![10,20,30];

//     let mut i = 0;

//     while i < 5 {
//         imprime(n.clone());
//         i = i + 1;
//     }
// }

// fn imprime(n: Vec<i32>) {
//     println!("{:?}", n);
// }

// for primitive and non-expansive types (arrays, integers, floats) Rust does not lose the variable property
// but for vectors and structs we must pass the reference to them or clone them.

// fn main() {
//     let v = vec!["liberté".to_string(),
//                                 "egalité".to_string(),
//                                 "fraternité".to_string()];

//     for mut s in v {
//         s.push('!');
//         println!("{}", s);
//     }
    
// }

// fn main() {
//     struct Person { name: Option<String>, birth: i32 }

//     let mut composers = Vec::new();

//     composers.push(Person { name: Some("Yoko Ono".to_string()), birth: 1943 });

//     // let first_name = composers[0].name; // if I transfer the person's name here, the name of the object will be empty, so I have to pass it as reference, using &:

//     // let first_name = &composers[0].name; // or I can use the take() method, when I have no primitive types

//     // (Takes the value out of the option, leaving a None in its place)
//     let first_name = composers[0].name.take();
//     // clone is also a good option!
//     let birth = composers[0].birth; 

//     println!("{:?}, {}", first_name, birth);

//     println!("{:?}, {}", composers[0].name.take(), birth);
// }

fn main() {
    let name: String = "Poe".to_string();

    let number: i32 = 10;

    // string is a struct in rust ( pub struct String { vec: Vec<u8, Global>,} ) so, it will not be passed with a copy of value
    // If I don't clone or borrow it, I won't be able to use the variable name later as I lost ownership of this variable
    // println!("{}", test(name)); -> THIS IS WRONG IF I WANT TO USE "NAME" LATER!!!

    println!("FN -> {}", test_str(name.clone()));

    // now for primitive types I do not need to clone or borrow it.. Rust clones it automatically, it's easier for him...
    println!("FN -> {}", test_num(number));

    println!("ORIGINAL -> {}", name);
    println!("ORIGINAL -> {}", number); // I can use the same primitive variable without cloning it first...
}   

fn test_str(mut str: String) -> String{
    str.push_str(" Edgar");
    str
}

fn test_num(number: i32) -> i32 {
    number + 10
}