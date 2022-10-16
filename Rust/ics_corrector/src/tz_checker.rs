#[path = "./unix_tz_converter.rs"]
mod unix_tz_converter;

use std::fs;
use std::io;
// use std::io::Error;
use std::io::ErrorKind;
use std::io::Lines;

pub fn check_ics(file_path: String) -> String {
    if !file_path.is_empty() {
        // if the path of the file is provided, it returns the file converted
        let invitation_result = fs::read_to_string(&file_path);
        let invitation = match invitation_result {
            Ok(file) => file,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    panic!("[ERROR] File: \"{}\" not found", &file_path)
                }
                other_error => panic!("[ERROR] Problem reading the file: \"{}\"", other_error),
            },
        };
        let tz_from_file = extract_tz(file_path);
        let unix_tz = unix_tz_converter::convert_tz(&tz_from_file);
        return invitation.replace(tz_from_file.as_str(), unix_tz);
    } else {
        // if the input came from stin, return the details of the file
        let lines = io::stdin().lines();
        return extract_information(lines);
    };
}

fn extract_information(lines: Lines<std::io::StdinLock>) -> String {
    let mut description: String = "Description\n".to_string();
    for line in lines {
        let line_unwrap = line.as_ref().unwrap();
        if line_unwrap.contains("TZID:") {
            description += line.unwrap().split(':').last().unwrap();
            description += "\n";
        } else if line_unwrap.contains("SUMMARY;") {
            description += "Summary: ";
            description += line.unwrap().split(':').last().unwrap();
            description += "\n";
        } else if line_unwrap.contains("DTSTART;") {
            description += "From: ";
            description += line
                .unwrap()
                .split(':')
                .last()
                .unwrap()
                .split('T')
                .last()
                .unwrap()
                .get(..4)
                .unwrap();
            description += "\n";
        } else if line_unwrap.contains("DTEND;") {
            description += "To: ";
            description += line
                .unwrap()
                .split(':')
                .last()
                .unwrap()
                .split('T')
                .last()
                .unwrap()
                .get(..4)
                .unwrap();
            description += "\n";
        } else if line_unwrap.contains("DESCRIPTION;") {
            description += "Description: ";
            description += line.unwrap().split(':').last().unwrap();
            description += "\n";
        }
    }
    return description;
}

pub fn extract_tz(file_path: String) -> String {
    let invitation = fs::read_to_string(file_path).unwrap();
    let lines: Vec<&str> = invitation.split('\n').collect();
    let mut tz_from_file: String = "".to_string();
    for i in 0..lines.len() {
        if lines.get(i).unwrap().contains("TZID:") {
            let tz_from_file_array: Vec<&str> = lines.get(i).unwrap().split(':').collect();
            tz_from_file = tz_from_file_array.get(1).unwrap().to_string();
        }
    }
    tz_from_file.replace('\r', "")
}
