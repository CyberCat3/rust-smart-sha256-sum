use std::{io, env, fs, fs::File};
use sha2::{Sha256, Digest};
use regex::Regex;
use std::io::Write;

fn main() {
    // Get the the launch arguments.
    let args: Vec<String> = env::args().collect();

    // The first argument is the path of the executable, so we need to check if there are less than 3.
    if args.len() < 3 {
        eprintln!("You need to pass 2 arguments.");
        eprintln!("arg1: path to checksums file");
        eprintln!("arg2: path to source file");
        return;
    }

    // Get the paths from the args.
    let checksums_path = &args[1];
    let source_path = &args[2];

    let checksums_content =
        if fs::metadata(checksums_path).is_ok() {
            fs::read_to_string(checksums_path).expect("Couldn't read checksums file.")
        } else { String::new() };

    // regex explanation:
    // [\da-f]{64}    match a 64 character hex string
    //  {2}           match 2 spaces
    // {}             replaced by format, matches source_file.
    let pattern = {
        let raw_regex = r"([\da-f]{64}) {2}";
        let concatenated = format!("{}({})", raw_regex, source_path);
        Regex::new(concatenated.as_str()).unwrap()
    };

    println!("Calculating hash...");

    let hash = {
        let mut source_file = File::open(source_path).unwrap();
        hash_file(&mut source_file).unwrap()
    };

    println!("\n{}  {}\n", hash, source_path);

    let updated_checksums_content =  if pattern.is_match(checksums_content.as_str()) {
        println!("Checksums file already contains a checksum for this file. Updating...");

        let replacement_str = format!("{}  $2", hash);
        String::from(pattern.replace(checksums_content.as_str(), replacement_str.as_str()))

    } else {
        println!("Checksums file doesn't contain a checksum for this file. Adding...");

        format!("{}\n{}  {}\n", checksums_content.trim_end(), hash, source_path)

    };

    File::create(checksums_path).unwrap().write(updated_checksums_content.as_bytes()).expect("Couldn't write hashes to file.");

    println!("Updated checksums file.");
}

fn hash_file(file: &mut File) -> io::Result<String> {
    // Create digest
    let mut sha256 = Sha256::new();

    // Copy the data from the source_file to the digest,
    // this streams the data over without storing the file in RAM in its entirety,
    // so it it's safe for big files
    io::copy(file, &mut sha256)?;

    // format the hash as a hex-string and return it.
    Ok(format!("{:x}", sha256.result()))
}


