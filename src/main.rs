use std::fs;
use std::path::Path;
use std::process::Command;
fn main() {
    let current_dir = std::env::current_dir().unwrap();
    let path = Path::new(&current_dir);
    let mut files = fs::read_dir(path)
        .expect("Failed to read directory")
        .filter_map(|entry| {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            if path.is_file() {
                Some((
                    path.clone(),
                    fs::metadata(path.clone()).expect("Failed to read file metadata"),
                ))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    files.sort_by(|a, b| {
        let a_size = a.1.len();
        let b_size = b.1.len();
        b_size.cmp(&a_size)
    });
    let mut largest_files = Vec::new();
    for (path, _) in files.iter().take(2) {
        let file_name = path
            .file_name()
            .expect("Failed to get file name")
            .to_str()
            .expect("Failed to convert file name to str");
        largest_files.push(file_name);
    }
    let _output = Command::new("ffmpeg")
        .arg("-i")
        .arg(largest_files[1])
        .arg("-i")
        .arg(largest_files[0])
        .arg("-codec")
        .arg("copy")
        .arg("Output.mp4")
        .output()
        .expect("Failed to execute process");
    println!("{}", String::from_utf8_lossy(_output.stderr.as_ref()));
    println!("\x1b[32mYou have done it successfully!\x1b[0m");
}
