
// Project: YCC
// A Toy C Compiler for self-education and fun
// Reference: https://github.com/rui314/9cc

fn main() {
    println!("Hello, world!");
}


// #[test]
// fn compare_with_clang_output() {
//     use std::fs;
//     use std::process::Command;
//     use std::io::Write;
//     use std::io::stderr;

//     let example_paths = match fs::read_dir("examples") {
//         Ok(paths) => paths,
//         Err(e) =>  panic!("Error reading examples directory: {}", e)
//     };

//     for path in example_paths {
//         let name = path.unwrap().path().to_str().unwrap().to_string();

//         writeln!(&mut stderr(), "comparing {} ...", name).unwrap();   
//     }
// }

