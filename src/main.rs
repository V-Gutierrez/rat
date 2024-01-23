use file_reader::FileReader; // Add import statement for FileReader module
mod args_parser;
mod file_reader;

#[tokio::main]
async fn main() {
    let file_paths: Vec<String> = args_parser::fetch();

    FileReader {
        file_paths
    }
    .read_from_paths()
    .await;
}
