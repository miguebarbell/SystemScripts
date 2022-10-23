use std::env;
use std::fs;

fn main() {
    let mut args = env::args().collect::<Vec<String>>();
    args.remove(0);
    let mut emails_path = String::new();
    if !args.is_empty() {
        emails_path.push_str(args.join(" ").as_str())
    } else {
        emails_path.push_str("/home/t800/.local/share/mail/redpills/INBOX/new")
    };
    let files = match fs::read_dir(&emails_path) {
        Ok(num) => num,
        Err(_) => panic!("Provided a bad path."),
    };
    for file in files {
        let file_path = file.unwrap().path();
        let file_raw = fs::read_to_string(file_path).unwrap();
        let file_array = file_raw.split('\n');
        for line in file_array {
            if line.contains("From:") && line.split(':').collect::<Vec<&str>>()[0] == "From" {
                println!();
                println!("{:?}", line)
            }
            if line.contains("Subject:") && line.split(':').collect::<Vec<&str>>()[0] == "Subject" {
                println!("{:?}", line);
            }
        }
    }
}
