use prelude::*;
use rayon::prelude::*;
use std::{
    fs::{read_dir, File},
    io::{self, Read, Write},
};

fn main() {
    let reqwest_client = reqwest::blocking::Client::new();
    (b'1'..=b'8').into_par_iter().for_each(|kyu| {
        read_dir(unsafe { core::str::from_utf8_unchecked(&[kyu, b'k', b'y', b'u']) })
            .unwrap()
            .par_bridge()
            .for_each(|kata_dir| {
                let kata_dir = kata_dir.unwrap().path().into_os_string();
                check_kata(&reqwest_client, kyu, kata_dir.to_str().unwrap());
            });
    });
}

fn check_kata(reqwest_client: &reqwest::blocking::Client, kyu: u8, kata_dir: &str) {
    let mut path = String::with_capacity(kata_dir.len() + "/benches/bench.rs".len());
    unsafe {
        path.push_str_unchecked(kata_dir);
        path.push_str_unchecked("/src/lib.rs");
    }

    let mut f = File::open(&path).unwrap();
    let mut buf = [0;
        "//! <https://www.codewars.com/kata/53da3dbb4a5168369a0000fe/train/rust>\n\n#![no_std]"
            .len()];
    f.read(&mut buf).unwrap();
    if !(buf.starts_with(b"//! <https://www.codewars.com/kata/")
        && buf.ends_with(b"/train/rust>\n\n#![no_std]"))
    {
        let mut stdout = io::stdout().lock();
        stdout.write(path.as_bytes()).unwrap();
        stdout.write(b" has invalid url or std").unwrap();
        stdout.write(b"\n").unwrap();
    }
    let id = &buf["//! <https://www.codewars.com/kata/".len()
        .."//! <https://www.codewars.com/kata/53da3dbb4a5168369a0000fe".len()];
    let id = id.try_into().unwrap();

    let kata = get_kata(reqwest_client, id).unwrap();

    let kata_kyu = get_kyu(&kata);
    if kyu != kata_kyu {
        let mut stdout = io::stdout().lock();
        stdout.write(kata_dir.as_bytes()).unwrap();
        stdout.write(b" has obsolete kyu: should be ").unwrap();
        stdout.write(&[kata_kyu]).unwrap();
        stdout.write(b"\n").unwrap();
    }

    let slug = get_slug(&kata);
    if slug.as_bytes().contains(&b'\\') {
        return;
    }

    let directory_name = &kata_dir["8kyu".len() + 1..];
    if directory_name != slug {
        let mut stdout = io::stdout().lock();
        stdout.write(kata_dir.as_bytes()).unwrap();
        stdout
            .write(b" has obsolete directory name: should be ")
            .unwrap();
        stdout.write(slug.as_bytes()).unwrap();
        stdout.write(b"\n").unwrap();
    }

    for string in ["/benches/bench.rs", "/tests/test.rs"] {
        path.truncate(kata_dir.len());
        unsafe { path.push_str_unchecked(string) };
        let mut f = match File::open(&path) {
            Ok(f) => f,
            Err(e) if e.kind() == io::ErrorKind::NotFound => continue,
            e => e.unwrap(),
        };
        let mut buf = [0; "#![no_std]".len()];
        f.read(&mut buf).unwrap();
        if buf != *b"#![no_std]" {
            let mut stdout = io::stdout().lock();
            stdout.write(path.as_bytes()).unwrap();
            stdout.write(b" has std").unwrap();
            stdout.write(b"\n").unwrap();
        }
    }

    path.truncate(kata_dir.len());
    unsafe { path.push_str_unchecked("/Cargo.toml") };
    let (buf, name_pos, name_end) = get_crate_name(&path).unwrap();
    let crate_name =
        unsafe { core::str::from_utf8_unchecked(buf.get_unchecked(name_pos..name_end)) };
    if crate_name.as_bytes() != slug.as_bytes() {
        if (b'0'..=b'9').contains(&slug.as_bytes()[0]) {
            if !(crate_name.starts_with("solution-")
                && &crate_name.as_bytes()["solution-".len()..] == slug.as_bytes())
            {
                let mut stdout = io::stdout().lock();
                stdout.write(kata_dir.as_bytes()).unwrap();
                stdout
                    .write(b" has obsolete crate name: should be solution-")
                    .unwrap();
                stdout.write(slug.as_bytes()).unwrap();
                stdout.write(b"\n").unwrap();
            }
        } else {
            let mut stdout = io::stdout().lock();
            stdout.write(kata_dir.as_bytes()).unwrap();
            stdout
                .write(b" has obsolete crate name: should be ")
                .unwrap();
            stdout.write(slug.as_bytes()).unwrap();
            stdout.write(b"\n").unwrap();
        }
    }
}

fn get_crate_name(path: &str) -> io::Result<([u8; 128], usize, usize)> {
    let mut f = File::open(path)?;
    let mut buf = [0; 128];
    f.read(&mut buf)?;

    let name_pos = "[package]\nname = \"".len();
    let name_end = name_pos + buf[name_pos..].iter().position(|&b| b == b'"').unwrap();

    Ok((buf, name_pos, name_end))
}

fn get_kata(reqwest_client: &reqwest::blocking::Client, id: [u8; 24]) -> reqwest::Result<String> {
    let mut url =
        [0; "https://www.codewars.com/api/v1/code-challenges/5917f22dd2563a36a200009c".len()];
    url[.."https://www.codewars.com/api/v1/code-challenges/".len()]
        .copy_from_slice(b"https://www.codewars.com/api/v1/code-challenges/");
    url["https://www.codewars.com/api/v1/code-challenges/".len()..].copy_from_slice(&id);
    let url = unsafe { core::str::from_utf8_unchecked(&url) };
    reqwest_client.get(url).send()?.text()
}

fn get_kyu(kata: &str) -> u8 {
    let id_pos = kata.find("rank\":{\"id\"").unwrap() + "rank\":{\"id\":-".len();
    kata.as_bytes()[id_pos]
}

fn get_slug(kata: &str) -> &str {
    let slug_pos = kata.find("slug\":").unwrap() + "slug\":\"".len();
    let slug_end = slug_pos
        + kata.as_bytes()[slug_pos..]
            .iter()
            .position(|&b| b == b'"')
            .unwrap();
    unsafe { kata.get_unchecked(slug_pos..slug_end) }
}
