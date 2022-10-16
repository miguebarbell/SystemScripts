use std::fs::File;
use std::io::Write;

pub fn write(name: String, content: String) {
    // Create a temporary file.
    // let temp_directory = env::temp_dir();
    // let temp_file = temp_directory.join(name);

    // Open a file in write-only (ignoring errors).
    // This creates the file if it does not exist (and empty the file if it exists).
    let mut file = File::create(name).unwrap();

    // Write a &str in the file (ignoring the result).
    writeln!(&mut file, "{}", content.as_str()).unwrap();
}
