use std::process::Command;

fn main() {
  // let mut stat = Command::new("bluetooth --help | egrep -o 'on|off'")
  let mut stat = Command::new("/usr/bin/bluetooth");
  stat.arg("--help");
  println!(stat);
}
