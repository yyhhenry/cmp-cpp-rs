fn main() {
    let current_dir = std::env::current_dir().unwrap();
    let current_dir = current_dir.to_str().unwrap();
    // read dir
    let read_dir = std::fs::read_dir(current_dir).unwrap();
    for entry in read_dir {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        println!("Filename: {}", file_name);
    }
}
