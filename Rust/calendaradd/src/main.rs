// mod args;
mod unix_tz_converter;
use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().skip(2).collect();
    let file_path = args.join(" ");
    let invitation = fs::read_to_string(file_path).unwrap();
    let lines: Vec<&str> = invitation.split('\n').collect();
    let mut tz_from_file: String = "".to_string();
    for i in 0..lines.len() {
        if lines.get(i).unwrap().contains("TZID:") {
            let tz_from_file_array: Vec<&str> = lines.get(i).unwrap().split(':').collect();
            tz_from_file = tz_from_file_array.get(1).unwrap().to_string();
        }
    }
    tz_from_file = tz_from_file.replace('\r', "");
    let unix_tz = unix_tz_converter::convert_tz(&tz_from_file);
    // println!("changing TZID:{:?} to {:?}", tz_from_file, unix_tz);
    println!("{}", invitation.replace(tz_from_file.as_str(), unix_tz));
}
