extern crate serde;
extern crate dirs;

mod absolute_path;
mod relative_path;

pub use absolute_path::AbsolutePath;
pub use relative_path::RelativePath;

fn check_and_create_segments(path: &str) -> Result<Vec<String>, String> {
    let invalid =
        if !path.is_empty() {
            if is_invalid_start_byte(path.bytes().nth(0).unwrap()) {
                Some(0)
            } else if is_invalid_end_byte(path.bytes().rev().nth(0).unwrap()) {
                Some(path.len()-1)
            } else {
                path.bytes().position(is_invalid_byte)
            }
        } else  {
            None
        };

    invalid.map_or_else(
        || Ok(path.split("/").filter(|&s| s != "").map(String::from).collect()),
        |index| Err(format!("Invalid char {} at position {}", path.bytes().nth(index).unwrap(), index)))    
}

fn is_invalid_start_byte(b: u8) -> bool { // incorrect, path isn't split yet!
    b == '-' as u8
}
fn is_invalid_end_byte(b: u8) -> bool { // incorrect, path isn't split yet!
    b == ' ' as u8 || b == '.' as u8
}
const INVALID_BYTES: [u8; 10] = *br#""$*?:<>\^|"#;

fn is_invalid_byte(b: u8) -> bool {
    match b {
        _ if INVALID_BYTES.contains(&b) => {println!("invalid: {}", b as char); true},
        // https://www.dwheeler.com/essays/fixing-unix-linux-filenames.html
        _ if (b as char).is_control()  => true,
        _                              => false
    }
}

#[cfg(test)]
mod test;
