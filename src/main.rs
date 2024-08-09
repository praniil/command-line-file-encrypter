use std::{fs::{self, File}, io::Write, path::Path};

fn stream_cipher(file_content: String) -> String {
    return file_content.to_string()
}

fn main() {
    println!("Hello, world!");
    let mut path: String = String::new();
    println!("enter the path: ");
    std::io::stdin().read_line(&mut path).expect("cannot read user input");
    println!("the path is: {path}");

    path = path.trim().to_string();

    let file_contents = fs::read_to_string(&mut path).expect("couldnot read from file");
    println!("info.txt content: \n{file_contents}");

    let encrypted_text: String = stream_cipher(file_contents);
    println!("encrypted file: \n{}", encrypted_text);

    let mut req_index = 0;

    for (mut index, _char) in path.chars().enumerate() {
        if index == path.len() - 1 {
            while path.as_bytes()[index] != b'/' {
                index = index - 1;
                println!("not imp {}", index);
            }
            req_index = index;
            println!("imp index: {}", index);
        }
    }

    let new_path;
    let offset;
    (new_path, offset) = path.split_at(req_index + 1);
    println!("{}", new_path);
    println!("{}", offset);

    let binding = format!("{}encrypted_{}", new_path, offset);
    let concatinated_path = Path::new(&binding);
    let mut out_file = match File::create(&concatinated_path) {
        Err(err) => panic!("couldnot create a file: {}", err),
        Ok(out_file) => out_file,
    };

    match out_file.write_all(encrypted_text.as_bytes()) {
        Err(err) => panic!("couldn't write due to: {}", err),
        Ok(_) => println!("successfully wrote")
,    }


}
