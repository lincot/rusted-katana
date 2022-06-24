use std::{fs::File, io::Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut kyu_path = *b"7kyu";

    for k in 1..=8 {
        kyu_path[0] = b'0' + k;
        let rd = std::fs::read_dir(unsafe { std::str::from_utf8_unchecked(&kyu_path) });
        let rd = if let Ok(rd) = rd {
            rd
        } else {
            continue;
        };
        println!("checking {} kyu", k);
        for d in rd {
            let d = d?.path().into_os_string();
            let d = d.to_str().unwrap();
            let mut path = format!("{}/src/lib.rs", d);

            let id = get_id(&path)?;
            let kata = get_kata(id)?;

            let kyu = get_kyu(&kata);
            if k != kyu {
                eprintln!("{:?} has obsolete kyu: should be {}", d, kyu);
            }

            let slug = get_slug(&kata);
            if slug.as_bytes().contains(&b'\\') {
                continue;
            }

            let directory_name = &d[kyu_path.len() + 1..];
            if directory_name != slug {
                eprintln!(
                    "{:?} has obsolete directory name: should be \"{}\"",
                    d, slug
                );
            }

            unsafe {
                let slice = path.as_bytes_mut().get_unchecked_mut(d.len()..);
                if b"/src/lib.rs".len() != slice.len() {
                    std::hint::unreachable_unchecked();
                }
                slice.copy_from_slice(b"/Cargo.toml");
            }
            let crate_name = get_crate_name(&path)?;
            if crate_name.as_bytes() != slug.as_bytes() {
                if (b'0'..=b'9').contains(&slug.as_bytes()[0]) {
                    if !(crate_name.starts_with("solution-")
                        && &crate_name.as_bytes()["solution-".len()..] == slug.as_bytes())
                    {
                        eprintln!(
                            "{:?} has obsolete crate name: should be \"solution-{}\"",
                            d, slug
                        );
                    }
                } else {
                    eprintln!("{:?} has obsolete crate name: should be \"{}\"", d, slug);
                }
            }
        }
    }

    Ok(())
}

fn get_id(path: &str) -> std::io::Result<[u8; 24]> {
    let mut f = File::open(path)?;
    let mut buf =
        [0; "//! <https://www.codewars.com/kata/53da3dbb4a5168369a0000fe/train/rust>".len()];
    f.read_exact(&mut buf)?;
    assert!(
        buf.starts_with(b"//! <https://www.codewars.com/kata/") && buf.ends_with(b"/train/rust>"),
        "{} has invalid url",
        path
    );

    let id = &buf["//! <https://www.codewars.com/kata/".len()
        .."//! <https://www.codewars.com/kata/53da3dbb4a5168369a0000fe".len()];
    Ok(id.try_into().unwrap())
}

fn get_crate_name(path: &str) -> std::io::Result<Box<str>> {
    let mut f = File::open(path)?;
    let mut buf = [0; 128];
    f.read(&mut buf)?;

    let name_pos = buf.iter().position(|&b| b == b'"').unwrap() + 1;
    let name_end = name_pos + buf[name_pos..].iter().position(|&b| b == b'"').unwrap();

    Ok(Box::from(unsafe {
        std::str::from_utf8_unchecked(&buf[name_pos..name_end])
    }))
}

fn get_kata(id: [u8; 24]) -> reqwest::Result<String> {
    let mut url =
        [0; "https://www.codewars.com/api/v1/code-challenges/5917f22dd2563a36a200009c".len()];
    url[.."https://www.codewars.com/api/v1/code-challenges/".len()]
        .copy_from_slice(b"https://www.codewars.com/api/v1/code-challenges/");
    url["https://www.codewars.com/api/v1/code-challenges/".len()..].copy_from_slice(&id);
    let url = unsafe { std::str::from_utf8_unchecked(&url) };
    reqwest::blocking::get(url)?.text()
}

fn get_kyu(kata: &str) -> u8 {
    let id_pos = kata.find("rank\":{\"id\"").unwrap() + "rank\":{\"id\":-".len();
    kata.as_bytes()[id_pos] - b'0'
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
