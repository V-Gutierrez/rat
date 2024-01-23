mod args_parser;
mod file_reader;

fn main() {
    let file_paths: Vec<String> = args_parser::fetch();

    file_reader::read_from_paths(file_paths);
}
