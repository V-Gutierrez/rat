pub fn read_from_paths(file_paths: Vec<String>) -> () {
    for path in file_paths {
        let content = std::fs::read_to_string(&path);

        match content {
            Ok(content) => println!("{}", content),
            Err(content) => println!("Error: {}", content),
        }
    }
}
