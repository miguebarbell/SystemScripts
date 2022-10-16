use std::env;

#[path = "./../src/tz_checker.rs"]
mod tz_checker;
#[path = "./../src/write_file.rs"]
mod write_file;
#[test]
fn check_ics_diff() {
    // check only if the difference in the file are the timezones
    // verify that the timezones are changed according
    let name = "/tmp/test_ics".to_string();
    write_file::write(
        name.clone(),
        tz_checker::check_ics("./tests/resources/test_ics".to_string()),
    );

    let tz_verified = tz_checker::extract_tz(name);
}
