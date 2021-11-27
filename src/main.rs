use std::{
    fs::{self, File},
    io::Write,
    ops::Range,
};

use anyhow::{ensure, Context, Result};
use rand::Rng;
use regex::bytes::Regex;

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    ensure!(args.len() == 2, "Usage: {} <file>", args[0]);

    let mut buf = fs::read(&args[1]).with_context(|| format!("Failed to read {}", &args[1]))?;
    let mut rng = rand::thread_rng();

    let type_desc_pos = Regex::new(r"\.\?A[V|U](?P<data>.+?)@@\x00")?
        .captures_iter(&buf)
        .map(|m| m.name("data").unwrap().range())
        .collect::<Vec<Range<usize>>>();

    let alpha_num = (b'A'..=b'Z')
        .chain(b'a'..=b'z')
        .chain(b'0'..=b'9')
        .collect::<Vec<u8>>();

    for pos in type_desc_pos {
        for byte in &mut buf[pos] {
            if byte != &b'@' {
                *byte = alpha_num[rng.gen_range(0..alpha_num.len())];
            }
        }
    }

    let enc_file_name = format!("{}.enc_rtti.exe", &args[1]);
    File::create(&enc_file_name)
        .with_context(|| format!("Failed to create {}", &enc_file_name))?
        .write_all(&buf)
        .with_context(|| format!("Failed to write to {}", &enc_file_name))?;

    Ok(())
}
