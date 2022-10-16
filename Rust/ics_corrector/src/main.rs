// mod args;
mod tz_checker;
use std::env;
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let file_path = args.join(" ");
    println!("{}", tz_checker::check_ics(file_path));
}

#[cfg(test)]
mod test {
    #[test]
    fn from_ms_to_unix() {
        // let test_file_path: str = "../tests/resources/test_ics";

        assert_eq!(2 + 2, 4);
    }
}
