use core::mem::MaybeUninit;
use my_prelude::prelude::*;
use std::{
    fs::File,
    io::{self, Read, Write},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout().lock();

    let mut kyu_path = *b"7kyu";

    for k in b'1'..=b'8' {
        kyu_path[0] = k;
        let rd = std::fs::read_dir(unsafe { core::str::from_utf8_unchecked(&kyu_path) })?;
        stdout.write(b"checking ")?;
        stdout.write(&[k])?;
        stdout.write(b" kyu\n")?;
        stdout.flush()?;
        for d in rd {
            let d = d?.path().into_os_string();
            let d = d.to_str().unwrap();
            let mut path = String::with_capacity(d.len() + "/src/lib.rs".len());
            unsafe {
                path.push_str_unchecked(d);
                path.push_str_unchecked("/src/lib.rs");
            }

            let mut f = File::open(&path)?;
            let mut buf = [0;
                "//! <https://www.codewars.com/kata/53da3dbb4a5168369a0000fe/train/rust>".len()];
            f.read(&mut buf)?;
            if !(buf.starts_with(b"//! <https://www.codewars.com/kata/")
                && buf.ends_with(b"/train/rust>"))
            {
                stdout.write(path.as_bytes())?;
                stdout.write(b" has invalid url")?;
                stdout.write(b"\n")?;
                stdout.flush()?;
            }
            let id = &buf["//! <https://www.codewars.com/kata/".len()
                .."//! <https://www.codewars.com/kata/53da3dbb4a5168369a0000fe".len()];
            let id = id.try_into().unwrap();

            let (kata, kata_len) = get_kata(id)?;
            let kata = unsafe { core::str::from_utf8_unchecked(kata.get_unchecked(..kata_len)) };

            let kyu = get_kyu(kata);
            if k != kyu {
                stdout.write(d.as_bytes())?;
                stdout.write(b" has obsolete kyu: should be ")?;
                stdout.write(&[kyu])?;
                stdout.write(b"\n")?;
                stdout.flush()?;
            }

            let slug = get_slug(kata);
            if slug.as_bytes().contains(&b'\\') {
                continue;
            }

            let directory_name = &d[kyu_path.len() + 1..];
            if directory_name != slug {
                stdout.write(d.as_bytes())?;
                stdout.write(b" has obsolete directory name: should be ")?;
                stdout.write(slug.as_bytes())?;
                stdout.write(b"\n")?;
                stdout.flush()?;
            }

            unsafe {
                let slice = path.as_bytes_mut().get_unchecked_mut(d.len()..);
                if b"/src/lib.rs".len() != slice.len() {
                    core::hint::unreachable_unchecked();
                }
                slice.copy_from_slice(b"/Cargo.toml");
            }
            let (buf, name_pos, name_end) = get_crate_name(&path)?;
            let crate_name =
                unsafe { core::str::from_utf8_unchecked(buf.get_unchecked(name_pos..name_end)) };
            if crate_name.as_bytes() != slug.as_bytes() {
                if (b'0'..=b'9').contains(&slug.as_bytes()[0]) {
                    if !(crate_name.starts_with("solution-")
                        && &crate_name.as_bytes()["solution-".len()..] == slug.as_bytes())
                    {
                        stdout.write(d.as_bytes())?;
                        stdout.write(b" has obsolete crate name: should be \"solution-")?;
                        stdout.write(slug.as_bytes())?;
                        stdout.write(b"\"\n")?;
                        stdout.flush()?;
                    }
                } else {
                    stdout.write(d.as_bytes())?;
                    stdout.write(b" has obsolete crate name: should be \"")?;
                    stdout.write(slug.as_bytes())?;
                    stdout.write(b"\"\n")?;
                    stdout.flush()?;
                }
            }
        }
    }

    Ok(())
}

fn get_crate_name(path: &str) -> std::io::Result<([u8; 128], usize, usize)> {
    let mut f = File::open(path)?;
    let mut buf = unsafe { MaybeUninit::<[u8; 128]>::uninit().assume_init() };
    f.read(&mut buf)?;

    let name_pos = "[package]\nname = \"".len();
    let name_end = name_pos + buf[name_pos..].iter().position(|&b| b == b'"').unwrap();

    Ok((buf, name_pos, name_end))
}

fn get_kata(id: [u8; 24]) -> attohttpc::Result<([u8; 1024], usize)> {
    let mut url =
        [0; "https://www.codewars.com/api/v1/code-challenges/5917f22dd2563a36a200009c".len()];
    url[.."https://www.codewars.com/api/v1/code-challenges/".len()]
        .copy_from_slice(b"https://www.codewars.com/api/v1/code-challenges/");
    url["https://www.codewars.com/api/v1/code-challenges/".len()..].copy_from_slice(&id);
    let url = unsafe { core::str::from_utf8_unchecked(&url) };
    let mut buf = unsafe { MaybeUninit::<[u8; 1024]>::uninit().assume_init() };
    let mut response = attohttpc::get(url).send()?;
    let mut written = response.read(&mut buf)?;
    if written < buf.len() {
        written += response.read(&mut buf[written..])?;
    }

    Ok((buf, written))
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
