#![feature(sync_unsafe_cell)]

use core::{
    cell::SyncUnsafeCell,
    mem::{forget, MaybeUninit},
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};
use std::{
    fs::{read_dir, File},
    io::{self, Read, Write},
};
use tokio::{sync::Notify, task::JoinSet};
use unchecked_std::prelude::*;

// yes, I'm writing this just to leak the memory
static REQWEST_CLIENT: SyncUnsafeCell<MaybeUninit<reqwest::Client>> =
    SyncUnsafeCell::new(MaybeUninit::uninit());
static BLOCKED: AtomicBool = AtomicBool::new(false);
static UNBLOCK_NOTIFY: Notify = Notify::const_new();

#[tokio::main]
// clippy buggin
#[allow(clippy::needless_return)]
async fn main() {
    unsafe { *REQWEST_CLIENT.get() = MaybeUninit::new(reqwest::Client::new()) };

    let mut tasks = JoinSet::new();

    for kyu in b'1'..=b'8' {
        for kata_dir in
            read_dir(unsafe { core::str::from_utf8_unchecked(&[kyu, b'k', b'y', b'u']) }).unwrap()
        {
            tasks.spawn(async move {
                let kata_dir = kata_dir.unwrap().path().into_os_string();
                check_kata(
                    unsafe { (*REQWEST_CLIENT.get()).assume_init_ref() },
                    kyu,
                    kata_dir.to_str().unwrap(),
                )
                .await;
            });
        }
    }

    while let Some(x) = tasks.join_next().await {
        x.unwrap();
    }

    forget(tasks);
}

async fn check_kata(reqwest_client: &reqwest::Client, kyu: u8, kata_dir: &str) {
    let mut path = String::with_capacity(kata_dir.len() + "/benches/bench.rs".len());
    unsafe {
        path.push_str_unchecked(kata_dir);
        path.push_str_unchecked("/src/lib.rs");
    }

    let mut f = File::open(&path).unwrap();
    let mut buf =
        [0; "//! <https://www.codewars.com/kata/53da3dbb4a5168369a0000fe/train/rust>".len()];
    f.read(&mut buf).unwrap();
    if !(buf.starts_with(b"//! <https://www.codewars.com/kata/") && buf.ends_with(b"/train/rust>"))
    {
        let mut stdout = io::stdout().lock();
        stdout.write(path.as_bytes()).unwrap();
        stdout.write(b" has invalid url\n").unwrap();
    }
    let id = &buf["//! <https://www.codewars.com/kata/".len()
        .."//! <https://www.codewars.com/kata/53da3dbb4a5168369a0000fe".len()];
    let id = id.try_into().unwrap();

    let kata = get_kata(reqwest_client, id).await.unwrap();

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

    path.truncate(kata_dir.len());
    unsafe { path.push_str_unchecked("/Cargo.toml") };
    let (buf, name_pos, name_end) = get_crate_name(&path).unwrap();
    let crate_name =
        unsafe { core::str::from_utf8_unchecked(buf.get_unchecked(name_pos..name_end)) };
    if crate_name.as_bytes() != slug.as_bytes() {
        if slug.as_bytes()[0].is_ascii_digit() {
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

async fn get_kata(reqwest_client: &reqwest::Client, id: [u8; 24]) -> reqwest::Result<String> {
    let mut url =
        [0; "https://www.codewars.com/api/v1/code-challenges/5917f22dd2563a36a200009c".len()];
    url[.."https://www.codewars.com/api/v1/code-challenges/".len()]
        .copy_from_slice(b"https://www.codewars.com/api/v1/code-challenges/");
    url["https://www.codewars.com/api/v1/code-challenges/".len()..].copy_from_slice(&id);
    let url = unsafe { core::str::from_utf8_unchecked(&url) };

    let mut blocked_by_this = false;
    loop {
        if !blocked_by_this && BLOCKED.load(Ordering::Acquire) {
            UNBLOCK_NOTIFY.notified().await;
        }

        let response = reqwest_client.get(url).send().await;
        match response {
            Ok(response) => {
                if response.status().is_success() {
                    if blocked_by_this {
                        BLOCKED.store(false, Ordering::Release);
                        UNBLOCK_NOTIFY.notify_waiters();
                    }
                    return response.text().await;
                } else if ![429, 502].contains(&response.status().as_u16()) {
                    panic!("{}", response.status());
                }
            }
            Err(e) => {
                let mut stderr = io::stderr().lock();
                stderr.write_all(e.to_string().as_bytes()).unwrap();
                stderr.write(b"\n").unwrap();
            }
        }
        if !blocked_by_this {
            blocked_by_this = BLOCKED
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .is_ok();
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
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
