use std::fs;

fn main() {
    println!("Hello, world!");
    let mut path: String = String::new();
    println!("enter the path: ");
    std::io::stdin().read_line(&mut path).expect("cannot read user input");
    println!("the path is: {path}");

    path = path.trim().to_string();

    let file_contents = fs::read_to_string(path).expect("couldnot read from file");
    println!("info.txt content: \n{file_contents}")
}
