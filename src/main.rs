use std::{fs::{self, File}, io::Write, path::Path, vec};
use rand::Rng;

fn pseudo_random_generator(length: usize) -> Vec<usize> {
    println!("in pseduo random");
    let mut rng = rand::thread_rng();
    let mut vector_random : Vec<usize> = vec![];
    let mut i = 0;
    while length > i {
        // println!("in side while loop");
        let random_bit = rng.gen_range(0..2);
        // println!("random bits: {}", random_bit);
        vector_random.push(random_bit);
        i = i + 1;
    }
    // println!("{:?}", vector_random);
    return vector_random;
}

fn stream_cipher(file_content: String) -> String {
    println!("in stream cipher");
    let mut file_content_bits: Vec<usize> = vec![];
    let random_bits = pseudo_random_generator(file_content.len());
    for (_int, char) in file_content.chars().enumerate() {
        let int_bits = char.to_string().parse::<usize>().unwrap();
        file_content_bits.push(int_bits)
    }
    println!("{:?}", file_content_bits);
    println!("length: {}, random bits length: {}", file_content.len(), random_bits.len());
    return file_content.to_string();
}

fn main() {
    println!("Hello, world!");
    let mut path: String = String::new();
    println!("enter the path: ");
    std::io::stdin().read_line(&mut path).expect("cannot read user input");
    println!("the path is: {path}");

    path = path.trim().to_string();

    let file_contents = fs::read_to_string(&mut path).expect("couldnot read from file");
    // println!("info.txt content: \n{file_contents}");

    let file_contents_ascii = file_contents.as_bytes();
    let mut file_content_binary = String::default();
    for bytes in file_contents_ascii.iter() {
        file_content_binary += &format!("{:b},", bytes);
    }
    // println!("{:?}", file_content_binary);
    // println!("{:?}", file_content_binary.as_bytes());

    let encrypted_text: String = stream_cipher(file_content_binary);
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
