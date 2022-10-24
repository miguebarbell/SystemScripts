use std::env;
use std::fs;

fn main() {
    let mut args = env::args().collect::<Vec<String>>();
    args.remove(0);
    let mut emails_path = String::new();
    let mut colored = false;
    for (i, arg) in args.clone().iter().enumerate() {
        if arg == "-c" {
            colored = true;
            args.remove(i);
        }
    }
    if !args.is_empty() {
        emails_path.push_str(args.join(" ").as_str())
    } else {
        emails_path.push_str("/home/t800/.local/share/mail/redpills/INBOX/new")
    };
    let files = match fs::read_dir(&emails_path) {
        Ok(mails) => mails,
        Err(_) => panic!("Provided a bad path."),
    };
    let unread_emails_count = fs::read_dir(&emails_path).unwrap().count();
    if unread_emails_count > 0 {
        if colored {
            println!(
                // "\nYou have \x1b[93m{} \x1b[0munread emails.",
                "\nYou have \x1b[6;30;42m{}\x1b[0m \x1b[1munread emails.\x1b[0m",
                unread_emails_count
            )
        } else {
            println!("\nYou have {} unread emails.", unread_emails_count)
        }
    } else {
        if colored {
            println!("\n\x1b[0;32;44mNo new emails.\x1b[0m")
        } else {
            println!("\nNo new emails.")
        }
    }

    for file in files {
        let file_path = file.unwrap().path();
        let file_raw = fs::read_to_string(file_path).unwrap();
        let file_array = file_raw.split('\n');
        let mut from = String::new();
        let mut has_subject: bool = false;
        let mut has_from: bool = false;
        let mut subject = String::new();
        for line in file_array {
            if line.contains("From:") && line.split(':').collect::<Vec<&str>>()[0] == "From" {
                has_from = true;
                from.push_str(
                    line.replace('\"', "").split("From:").collect::<Vec<&str>>()[1].trim(),
                );
            }
            if line.contains("Subject:") && line.split(':').collect::<Vec<&str>>()[0] == "Subject" {
                has_subject = true;
                subject.push_str(
                    line.replace('\"', "")
                        .split("Subject:")
                        .collect::<Vec<&str>>()[1]
                        .trim(),
                );
            }
        }
        if (has_subject) && (has_from) {
            if colored {
                println!("\x1b[1;34;42m{:?}\x1b[0m\n{:?}\n", from, subject);
            } else {
                println!("{:?}\n{:?}\n", from, subject);
            }
        }
    }
}
