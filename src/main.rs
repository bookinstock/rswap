use std::env;
// use std::io;
use std::fs;


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let file_name_1 = &args[1];
    let file_name_2 = &args[2];
    let file_name_tmp = &format!("{}_tmp", file_name_1)[..];

    fs::rename(file_name_1, file_name_tmp)?; // Rename a.txt to b.txt
    fs::rename(file_name_2, file_name_1)?;
    fs::rename(file_name_tmp, file_name_2)?;
    Ok(())
}