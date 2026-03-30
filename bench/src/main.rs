//! Benchmarks Codewars solutions against the most upvoted solutions. This code
//! uses naive and stupidly optimized parsing but it works.
#![feature(sync_unsafe_cell)]
#![feature(strip_circumfix)]
#![feature(write_all_vectored)]

use core::{cell::SyncUnsafeCell, mem::MaybeUninit};
use digital::prelude::*;
use reqwest::header::{COOKIE, HeaderMap, HeaderValue, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{self, File},
    io::{self, IoSlice, Read, StdoutLock, Write, stderr, stdin, stdout},
    path::{Path, PathBuf},
    process::{Command, Stdio, exit},
};
use unchecked_std::prelude::*;

/// The number of times to benchmark each solution.
const N: usize = 5;

// yes, I'm writing this just to leak memory
static REQWEST_CLIENT: SyncUnsafeCell<MaybeUninit<reqwest::blocking::Client>> =
    SyncUnsafeCell::new(MaybeUninit::uninit());

const YELLOW: &[u8] = b"\x1b[33m";
const RESET: &[u8] = b"\x1b[0m";

fn main() {
    unsafe { *REQWEST_CLIENT.get() = MaybeUninit::new(build_reqwest_client()) };
    let reqwest_client = unsafe { (*REQWEST_CLIENT.get()).assume_init_ref() };

    let current_dir = env::current_dir().unwrap();
    let bench_rs = current_dir.join("benches/bench.rs");
    if !bench_rs.is_file() {
        stderr()
            .lock()
            .write_all(b"Benchmarks not found\n")
            .unwrap();
        exit(1);
    }

    let solutions_page = env::args().nth(1).map_or_else(
        || {
            let kata_id = get_kata_id(&mut File::open("src/lib.rs").unwrap());
            let solutions_url = build_solutions_url(&kata_id);
            let solutions_url = unsafe { core::str::from_utf8_unchecked(&solutions_url) };
            SolutionsPage::new(fetch_page(reqwest_client, solutions_url), None)
        },
        // This option is for cases when none of the initially loaded most upvoted
        // solutions pass tests/benchmarks. `8kyu/hello-name-or-world` is probably the
        // only such case.
        |url| SolutionsPage::new(fetch_page(reqwest_client, &url), Some(url)),
    );

    let mut stdout = stdout().lock();
    let crate_name = get_crate_name(&current_dir).unwrap();
    for most_upvoted_solution in solutions_page {
        if bench_solution(
            &mut stdout,
            most_upvoted_solution,
            &current_dir,
            &bench_rs,
            &crate_name,
        ) {
            return;
        }
    }
    panic!(
        "Found no solutions that pass tests and benches and are not authored by rusted katana contributors"
    );
}

fn bench_solution(
    stdout: &mut StdoutLock<'_>,
    mut most_upvoted_solution: SolutionInfo,
    current_dir: &Path,
    bench_rs: &Path,
    crate_name: &str,
) -> bool {
    show_code(stdout, &most_upvoted_solution.code).unwrap();
    if !prompt_approve(
        stdout,
        "Is this most upvoted solution safe to benchmark? [y/N] ",
    )
    .unwrap()
    {
        exit(1);
    }

    let preloaded_code = if most_upvoted_solution.code.contains("preloaded::") {
        Some(
            fs::read_to_string(current_dir.join("src/preloaded.rs"))
                .expect("Failed to read src/preloaded.rs"),
        )
    } else {
        None
    };

    let (cargo_toml, has_rand) = make_cargo_toml(
        current_dir,
        bench_rs,
        crate_name,
        &most_upvoted_solution.code,
        preloaded_code.as_deref(),
    )
    .unwrap();
    most_upvoted_solution.code = make_runnable(
        &most_upvoted_solution.code,
        preloaded_code.as_deref(),
        has_rand,
    );

    stdout.write_all(b"Benchmarking...\n").unwrap();
    stdout.flush().unwrap();

    let replacers = [
        TempFileReplacer::try_new(
            current_dir.join("src/lib.rs"),
            most_upvoted_solution.code.as_bytes(),
        )
        .unwrap(),
        TempFileReplacer::try_new(current_dir.join("Cargo.toml"), cargo_toml.as_bytes()).unwrap(),
    ];

    if Path::new("tests").is_dir() {
        let out = Command::new("cargo").arg("test").output().unwrap();
        if !out.status.success() {
            let mut stderr = stderr().lock();
            stderr.write_all(&out.stderr).unwrap();
            stderr.flush().unwrap();
            return false;
        }
    }

    let Some(most_upvoted_result) = bench_n_times(current_dir).unwrap() else {
        return false;
    };
    drop(replacers);
    let rusted_katana_result = bench_n_times(current_dir).unwrap().unwrap();

    write_results_json(
        stdout,
        current_dir,
        &most_upvoted_solution.author,
        &most_upvoted_solution.url,
        &rusted_katana_result,
        &most_upvoted_result,
    )
    .unwrap();

    stdout.write_all(b"Wrote benches/results.json\n").unwrap();
    true
}

fn build_reqwest_client() -> reqwest::blocking::Client {
    let session_id = env::var("CODEWARS_SESSION_ID")
        .expect("set CODEWARS_SESSION_ID to the _session_id value from your browser cookies");
    assert_eq!(session_id.len(), 32);

    let mut cookie = [0; "_session_id=".len() + 32];
    cookie[.."_session_id=".len()].copy_from_slice(b"_session_id=");
    cookie["_session_id=".len()..].copy_from_slice(session_id.as_bytes());

    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_bytes(&cookie).unwrap());
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("rusted-katana-bench/0.1"),
    );
    reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap()
}

fn get_kata_id(f: &mut File) -> [u8; 24] {
    let mut buf =
        [0; "//! <https://www.codewars.com/kata/53da3dbb4a5168369a0000fe/train/rust>".len()];
    f.read(&mut buf).unwrap();
    assert!(
        buf.starts_with(b"//! <https://www.codewars.com/kata/") && buf.ends_with(b"/train/rust>"),
        "kata has an invalid url"
    );
    let id = &buf["//! <https://www.codewars.com/kata/".len()
        .."//! <https://www.codewars.com/kata/53da3dbb4a5168369a0000fe".len()];
    id.try_into().unwrap()
}

fn build_solutions_url(id: &[u8; 24]) -> [u8; 69] {
    let mut url =
        [0; "https://www.codewars.com/kata/595bbea8a930ac0b91000130/solutions/rust".len()];
    url[.."https://www.codewars.com/kata/".len()]
        .copy_from_slice(b"https://www.codewars.com/kata/");
    let url_len = url.len();
    url[url_len - "/solutions/rust".len()..].copy_from_slice(b"/solutions/rust");
    url["https://www.codewars.com/kata/".len()
        .."https://www.codewars.com/kata/595bbea8a930ac0b91000130".len()]
        .copy_from_slice(id);
    url
}

const RUSTED_KATANA_CONTRIBUTORS: &[&str] = &["lincot"];

struct SolutionsPage {
    text: String,
    cursor: usize,
    solution_url: Option<String>,
}

impl SolutionsPage {
    fn new(text: String, solution_url: Option<String>) -> Self {
        const ICON_MOON_USER: &str = "<i class=\"icon-moon-user";
        // me
        let cursor = text.find(ICON_MOON_USER).unwrap() + ICON_MOON_USER.len();
        // kata author
        let cursor = cursor + text[cursor..].find(ICON_MOON_USER).unwrap() + ICON_MOON_USER.len();
        Self {
            text,
            cursor,
            solution_url,
        }
    }
}

impl Iterator for SolutionsPage {
    type Item = SolutionInfo;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let text = &self.text[self.cursor..];

            let moon_user_pos = text.find("<i class=\"icon-moon-user \"></i>");
            let moon_users_pos = text.find("<i class=\"icon-moon-users \"></i>");
            let authors_pos = match (moon_user_pos, moon_users_pos) {
                (Some(pos), None) | (None, Some(pos)) => pos,
                (Some(pos0), Some(pos1)) => pos0.min(pos1),
                (None, None) => return None,
            };
            let author_name_end = authors_pos + text[authors_pos..].find("</a>").unwrap();
            let author_name_pos = text[..author_name_end].rfind("\">").unwrap() + "\">".len();
            let author_name = text[author_name_pos..author_name_end].into();
            if RUSTED_KATANA_CONTRIBUTORS.iter().any(|&x| x == author_name) {
                self.cursor += author_name_end;
                continue;
            }

            let solution_code_pos = author_name_end
                + text[author_name_end..].find("<code data").unwrap()
                + "<code data-language=\"rust\">".len();
            let solution_code_end =
                solution_code_pos + text[solution_code_pos..].find('<').unwrap();
            let solution_code =
                html_escape::decode_html_entities(&text[solution_code_pos..solution_code_end])
                    .into();

            let solution_url = self.solution_url.take().unwrap_or_else(|| {
                let solution_url_end =
                    solution_code_end + text[solution_code_end..].find("\">Link").unwrap();
                let solution_url_pos = 1 + text[..solution_url_end].rfind('\"').unwrap();
                let solution_url_path = &text[solution_url_pos..solution_url_end];
                let mut solution_url =
                    String::with_capacity("codewars.com".len() + solution_url_path.len());
                unsafe {
                    solution_url.push_str_unchecked("codewars.com");
                    solution_url.push_str_unchecked(solution_url_path);
                }
                solution_url
            });

            self.cursor += solution_code_end;
            return Some(SolutionInfo {
                code: solution_code,
                author: author_name,
                url: solution_url,
            });
        }
    }
}

struct SolutionInfo {
    code: String,
    author: String,
    url: String,
}

fn fetch_page(reqwest_client: &reqwest::blocking::Client, url: &str) -> String {
    loop {
        let response = reqwest_client.get(url).send();
        match response {
            Ok(response) => {
                if response.status().is_success() {
                    break response.text().unwrap();
                } else if ![429, 502].contains(&response.status().as_u16()) {
                    panic!("{}", response.status());
                }
            }
            Err(e) => {
                panic!("{e}");
            }
        }
    }
}

fn show_code(stdout: &mut StdoutLock<'_>, code: &str) -> io::Result<()> {
    if Command::new("bat")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
    {
        let mut child = Command::new("bat")
            .args(["--language=rust", "--style=plain", "-"])
            .stdin(Stdio::piped())
            .spawn()?;

        child.stdin.as_mut().unwrap().write_all(code.as_bytes())?;
        let _ = child.wait()?;
        return Ok(());
    }
    let mut stderr = stderr().lock();
    stderr
        .write_all(b"bat not found, fallback to less (no syntax highlighting)\n")
        .unwrap();
    stderr.flush()?;

    if Command::new("less")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
    {
        let mut child = Command::new("less")
            .arg("-F")
            .stdin(Stdio::piped())
            .spawn()?;

        child.stdin.as_mut().unwrap().write_all(code.as_bytes())?;
        let _ = child.wait()?;
        return Ok(());
    }
    stderr.write_all(b"less not found...\n").unwrap();
    stderr.flush()?;

    stdout
        .write_all_vectored(&mut [IoSlice::new(code.as_bytes()), IoSlice::new(b"\n")])
        .unwrap();

    Ok(())
}

fn prompt_approve(stdout: &mut StdoutLock<'_>, text: &str) -> io::Result<bool> {
    loop {
        stdout.write_all(text.as_bytes())?;
        stdout.flush()?;

        let mut line = String::new();
        stdin().read_line(&mut line)?;
        let line = line.trim();
        if matches!(line, "y" | "Y" | "yes" | "YES") {
            return Ok(true);
        } else if matches!(line, "" | "n" | "N" | "no" | "NO") {
            return Ok(false);
        }
    }
}

fn get_crate_name(current_dir: &Path) -> io::Result<String> {
    let cargo_toml = fs::read_to_string(current_dir.join("Cargo.toml"))?;
    let name_start = "[package]\nname = \"".len();
    let name_end = name_start
        + cargo_toml.as_bytes()[name_start..]
            .iter()
            .position(|&x| x == b'"')
            .unwrap();
    Ok(cargo_toml[name_start..name_end].into())
}

fn make_runnable(code: &str, preloaded_code: Option<&str>, has_rand: bool) -> String {
    const MOD_PRELOADED: &str = "mod preloaded;";
    const PREFIX: &str =
        "pub use a::*;use fully_pub::fully_pub;#[fully_pub(recursive)]#[macro_use]mod a{";
    const PRELOADED_PREFIX: &str = "\npub use preloaded::*;mod preloaded{";
    const PRELOADED_SUFFIX: &str = "}";
    const SUFFIX: &str = "\n}";
    const RAND_IMPORT: &str = "rand::";
    const RAND08_IMPORT: &str = "rand08::";
    const PRELOADED_IMPORT: &str = "use preloaded::";
    const PUB_PRELOADED_IMPORT: &str = "pub use preloaded::";
    const MACRO_EXPORT: &str = "#[macro_export]";
    const MACRO_RULES: &str = "macro_rules!";

    // 7kyu/greatest-common-divisor seems to be the only case of re-export
    if code == "use num::integer::gcd;" {
        return "pub use num::integer::gcd;".into();
    }

    const {
        assert!(
            (MACRO_EXPORT.len() + MACRO_RULES.len()) * PRELOADED_IMPORT.len()
                > PUB_PRELOADED_IMPORT.len() * MACRO_RULES.len()
                && (MACRO_EXPORT.len() + MACRO_RULES.len()) * RAND_IMPORT.len()
                    > RAND08_IMPORT.len() * MACRO_RULES.len()
        );
    }
    let mut res = String::with_capacity(
        PREFIX.len()
            + SUFFIX.len()
            + code.len()
            + code.len() / MACRO_RULES.len() * MACRO_EXPORT.len()
            + preloaded_code.as_ref().map_or(0, |preloaded_code| {
                PRELOADED_PREFIX.len() + PRELOADED_SUFFIX.len() - MOD_PRELOADED.len()
                    + preloaded_code.len()
                    + preloaded_code.len() / MACRO_RULES.len() * MACRO_EXPORT.len()
            }),
    );
    unsafe {
        res.push_str_unchecked(PREFIX);
        let code = code.as_bytes();
        let mut i = 0;
        while i < code.len() {
            if has_rand && code[i..].starts_with(RAND_IMPORT.as_bytes()) {
                res.push_str_unchecked(RAND08_IMPORT);
                i += RAND_IMPORT.len();
                continue;
            }
            if preloaded_code.is_some() {
                if code[i..].starts_with(PRELOADED_IMPORT.as_bytes()) {
                    res.push_str_unchecked(PUB_PRELOADED_IMPORT);
                    i += PRELOADED_IMPORT.len();
                    continue;
                }
                if code[i..].starts_with(MOD_PRELOADED.as_bytes()) {
                    i += MOD_PRELOADED.len();
                    continue;
                }
            }
            if replace_macro_use(&mut res, code, &mut i) {
                continue;
            }
            if code[i..].starts_with(MACRO_RULES.as_bytes()) {
                res.push_str_unchecked(MACRO_EXPORT);
                res.push_str_unchecked(MACRO_RULES);
                i += MACRO_RULES.len();
                continue;
            }
            res.as_mut_vec().push_unchecked(code[i]);
            i += 1;
        }
        if let Some(preloaded_code) = preloaded_code {
            res.push_str_unchecked(PRELOADED_PREFIX);
            let preloaded_code = preloaded_code.as_bytes();
            let mut i = 0;
            while i < preloaded_code.len() {
                if has_rand && preloaded_code[i..].starts_with(RAND_IMPORT.as_bytes()) {
                    res.push_str_unchecked(RAND08_IMPORT);
                    i += RAND_IMPORT.len();
                    continue;
                }
                if replace_macro_use(&mut res, preloaded_code, &mut i) {
                    continue;
                }
                if preloaded_code[i..].starts_with(MACRO_RULES.as_bytes()) {
                    res.push_str_unchecked(MACRO_EXPORT);
                    res.push_str_unchecked(MACRO_RULES);
                    i += MACRO_RULES.len();
                    continue;
                }
                res.as_mut_vec().push_unchecked(preloaded_code[i]);
                i += 1;
            }
            res.push_str_unchecked(PRELOADED_SUFFIX);
        }
        res.push_str_unchecked(SUFFIX);
    }
    res
}

unsafe fn replace_macro_use(res: &mut String, code: &[u8], i: &mut usize) -> bool {
    const MACRO_USE: &str = "#[macro_use]";

    if !code[*i..].starts_with(MACRO_USE.as_bytes()) {
        return false;
    }

    let semicolon_pos = *i
        + MACRO_USE.len()
        + code[*i + MACRO_USE.len()..]
            .iter()
            .position(|&b| b == b';')
            .unwrap();
    let mod_name_end = *i
        + code[*i..semicolon_pos]
            .iter()
            .rposition(|b| !b.is_ascii_whitespace())
            .unwrap()
        + 1;
    let mod_name_start = *i
        + code[*i..mod_name_end]
            .iter()
            .rposition(u8::is_ascii_whitespace)
            .unwrap()
        + 1;
    unsafe {
        assert!(core::str::from_utf8_unchecked(code)[*i..mod_name_start].contains("extern crate"));
        res.push_str_unchecked("use ");
        res.push_str_unchecked(core::str::from_utf8_unchecked(
            &code[mod_name_start..mod_name_end],
        ));
        res.push_str_unchecked("::*;");
    }
    *i = semicolon_pos + 1;

    true
}

fn make_cargo_toml(
    current_dir: &Path,
    bench_rs: &Path,
    crate_name: &str,
    code: &str,
    preloaded_code: Option<&str>,
) -> io::Result<(String, bool)> {
    const DEV_DEPS: &str = r#"[dev-dependencies]
rand = { version = "0.10.0", default-features = false, features = ["std"] }
rand_pcg = "0.10.1"
"#;

    let current_cargo_toml = fs::read_to_string(current_dir.join("Cargo.toml"))?;
    let workspace_cargo_toml = fs::read_to_string(current_dir.join("../../Cargo.toml"))?;

    let profile_release_pos = workspace_cargo_toml.find("[profile.release]").unwrap();
    let profile_release_end = profile_release_pos
        + "[profile.release]".len()
        + workspace_cargo_toml[profile_release_pos + "[profile.release]".len()..]
            .find('[')
            .unwrap_or_else(|| {
                workspace_cargo_toml[profile_release_pos + "[profile.release]".len()..].len()
            });
    let profile_release = &workspace_cargo_toml[profile_release_pos..profile_release_end];

    let mut in_dev_deps = false;
    let mut dev_dep_names = Vec::new();
    let mut has_dev_unchecked_std = false;
    for line in current_cargo_toml.lines() {
        if line.starts_with("[dev-dependencies]") {
            in_dev_deps = true;
            continue;
        }
        if line.starts_with('[') {
            in_dev_deps = false;
            continue;
        }
        if in_dev_deps && let Some(equal_pos) = line.find(" = ") {
            let dep_name = &line[..equal_pos];
            if dep_name == "unchecked-std" {
                has_dev_unchecked_std = true;
            }
            dev_dep_names.push(dep_name);
        }
    }
    if !has_dev_unchecked_std && fs::read_to_string(bench_rs)?.contains("unchecked_std") {
        dev_dep_names.push("unchecked-std");
    }

    let dev_dep_lines: Box<[_]> = dev_dep_names
        .iter()
        .map(|dep| {
            for line in workspace_cargo_toml.lines() {
                if let Some(rest) = line.strip_prefix(dep)
                    && rest.starts_with(" = ")
                {
                    return line;
                }
            }
            panic!("dev-dependency \"{dep}\" not found in workspace Cargo.toml");
        })
        .collect();

    let mut res = String::with_capacity(
        r#"[package]
name = ""
edition = "2021"
[dependencies]
fully_pub = "0.1"
bit-set = "0.5.3"
chrono = "0.4.23"
either = "1.8.0"
fancy-regex = "0.10.0"
futures = "0.3.25"
im = "15.1.0"
itertools = "0.10.5"
lazy_static = "1.4.0"
num = { version = "0.4.0", features = ["rand"] }
num-bigint = "*"
num-complex = "*"
num-integer = "*"
num-iter = "*"
num-rational = "*"
num-traits = "*"
once_cell = "1.16.0"
rand08 = { package = "rand", version = "0.8.5" }
regex = "1.7.0"
serde = { version = "1.0.150", features = ["derive"] }
serde_json = "1.0.89"
text_io = "0.1.12"
thiserror = "1.0.37"
tokio = { version = "1.23.0", features = ["full"] }
tokio-util = { version = "0.7.4", features = ["full"] }
"#
        .len()
            + DEV_DEPS.len()
            + if dev_dep_lines.is_empty() {
                0
            } else {
                "[dev-dependencies]\n".len()
            }
            + dev_dep_lines
                .iter()
                .map(|line| line.len() + 1)
                .sum::<usize>()
            + crate_name.len()
            + profile_release.len(),
    );
    let mut has_rand = false;
    unsafe {
        res.push_str_unchecked("[package]\nname = \"");
        res.push_str_unchecked(crate_name);
        res.push_str_unchecked("\"\nedition = \"2021\"\n[dependencies]\nfully_pub = \"0.1\"\n");
        // https://docs.codewars.com/languages/rust/
        for (dep_name, import_name, version) in [
            ("bit-set", "bit_set", "\"0.5.3\""),
            ("chrono", "chrono", "\"0.4.23\""),
            ("either", "either", "\"1.8.0\""),
            ("fancy-regex", "fancy_regex", "\"0.10.0\""),
            ("futures", "futures", "\"0.3.25\""),
            ("im", "im", "\"15.1.0\""),
            ("itertools", "itertools", "\"0.10.5\""),
            ("lazy_static", "lazy_static", "\"1.4.0\""),
            (
                "num",
                "num",
                "{ version = \"0.4.0\", features = [\"rand\"] }",
            ),
            ("once_cell", "once_cell", "\"1.16.0\""),
            (
                "rand08",
                "rand",
                "{ package = \"rand\", version = \"0.8.5\" }",
            ),
            ("regex", "regex", "\"1.7.0\""),
            (
                "serde",
                "serde",
                "{ version = \"1.0.150\", features = [\"derive\"] }",
            ),
            ("serde_json", "serde_json", "\"1.0.89\""),
            ("text_io", "text_io", "\"0.1.12\""),
            ("thiserror", "thiserror", "\"1.0.37\""),
            (
                "tokio",
                "tokio",
                "{ version = \"1.23.0\", features = [\"full\"] }",
            ),
            (
                "tokio-util",
                "tokio_util",
                "{ version = \"0.7.4\", features = [\"full\"] }",
            ),
        ] {
            if code.contains(import_name)
                || preloaded_code.is_some_and(|code| code.contains(import_name))
            {
                res.push_str_unchecked(dep_name);
                res.push_str_unchecked(" = ");
                res.push_str_unchecked(version);
                res.push_str_unchecked("\n");
                if import_name == "rand" {
                    has_rand = true;
                }
                if import_name == "num" {
                    for dep_name in [
                        "num-bigint",
                        "num-complex",
                        "num-integer",
                        "num-iter",
                        "num-rational",
                        "num-traits",
                    ] {
                        res.push_str_unchecked(dep_name);
                        res.push_str_unchecked(" = \"*\"\n");
                    }
                }
            }
        }
        if !dev_dep_lines.is_empty() {
            res.push_str_unchecked("[dev-dependencies]\n");
        }
        for line in dev_dep_lines {
            res.push_str_unchecked(line);
            res.push_unchecked('\n');
        }
        res.push_str_unchecked(profile_release);
    }
    Ok((res, has_rand))
}

struct TempFileReplacer {
    path: PathBuf,
    orig_path: PathBuf,
}

impl TempFileReplacer {
    fn try_new(path: PathBuf, new_contents: &[u8]) -> io::Result<Self> {
        let mut orig_path = path.as_os_str().to_os_string();
        orig_path.push(".orig");
        let orig_path = PathBuf::from(orig_path);

        fs::rename(&path, &orig_path)?;
        fs::write(&path, new_contents)?;

        Ok(Self { path, orig_path })
    }
}

impl Drop for TempFileReplacer {
    fn drop(&mut self) {
        fs::rename(&self.orig_path, &self.path).unwrap();
    }
}

#[derive(Deserialize)]
struct CargoBenchLine {
    name: String,
    median: f64,
}

#[derive(Debug)]
struct BenchNTimesRes {
    name: String,
    medians: [f64; N],
}

fn bench_n_times(working_dir: &Path) -> io::Result<Option<Vec<BenchNTimesRes>>> {
    let mut res = Vec::<BenchNTimesRes>::new();

    for _ in 0..N as u8 {
        let out = Command::new("cargo")
            .current_dir(working_dir)
            .args(["bench", "--", "--format=json", "-Zunstable-options"])
            .output()?;

        if !out.status.success() {
            let mut stderr = stderr().lock();
            stderr.write_all(&out.stderr)?;
            stderr.flush()?;
            return Ok(None);
        }

        for line in out.stdout.split(|&b| b == b'\n') {
            let Ok(bench_result) = serde_json::from_slice::<CargoBenchLine>(line) else {
                continue;
            };
            assert_ne!(bench_result.median, 0.);

            if let Some(entry) = res.iter_mut().find(|x| x.name == bench_result.name) {
                *entry.medians[1..]
                    .iter_mut()
                    .find(|&&mut x| x == 0.)
                    .unwrap() = bench_result.median;
            } else {
                let mut medians = [0.; N];
                medians[0] = bench_result.median;
                res.push(BenchNTimesRes {
                    name: bench_result.name,
                    medians,
                });
            }
        }
    }

    Ok(Some(res))
}

#[derive(Serialize)]
struct JsonBenchResult<'a> {
    name: &'a str,
    speedup: f64,
    rusted_katana_medians_ns: [f64; N],
    most_upvoted_medians_ns: [f64; N],
}

#[derive(Serialize)]
struct JsonResults<'a> {
    cpu: &'a str,
    rustc: &'a str,
    os: &'a str,
    most_upvoted_author: &'a str,
    most_upvoted_url: &'a str,
    benches: &'a [JsonBenchResult<'a>],
}

fn write_results_json(
    stdout: &mut StdoutLock<'_>,
    current_dir: &Path,
    most_upvoted_author: &str,
    most_upvoted_url: &str,
    rusted_katana_result: &[BenchNTimesRes],
    most_upvoted_result: &[BenchNTimesRes],
) -> io::Result<()> {
    let f = File::create(current_dir.join("benches/results.json"))?;
    let cpu = get_cpu_model();
    let rustc = Command::new("cargo")
        .args(["rustc", "--", "--version"])
        .output()?
        .stdout;
    let rustc = String::from_utf8_lossy(&rustc);
    let rustc = rustc.strip_circumfix("rustc ", '\n').unwrap();
    let os = std::env::consts::OS;

    let benches: Box<[_]> = rusted_katana_result
        .iter()
        .map(|rusted_katana_result| {
            let name = &rusted_katana_result.name;
            let most_upvoted = most_upvoted_result
                .iter()
                .find(|x| &x.name == name)
                .unwrap();
            let speedup = median(most_upvoted.medians) / median(rusted_katana_result.medians);
            let speedup_100 = (speedup.mul_add(100., 0.5)) as u64;
            let before_dot = (speedup_100 / 100).to_heapless_string();
            let after_dot = [
                (speedup_100 % 100 / 10) as u8 + b'0',
                (speedup_100 % 10) as u8 + b'0',
            ];
            let mut bufs = heapless::Vec::<_, 11>::new();
            bufs.extend([
                IoSlice::new(name.as_bytes()),
                IoSlice::new(b" speedup "),
                IoSlice::new(before_dot.as_bytes()),
                IoSlice::new(b"."),
                IoSlice::new(&after_dot),
                IoSlice::new(b"x"),
            ]);
            if speedup < 1. {
                bufs.extend([
                    IoSlice::new(b", "),
                    IoSlice::new(YELLOW),
                    IoSlice::new(b"less than 1"),
                    IoSlice::new(RESET),
                ]);
            }
            bufs.push(IoSlice::new(b"\n")).unwrap();
            stdout.write_all_vectored(&mut bufs).unwrap();
            JsonBenchResult {
                name,
                speedup,
                rusted_katana_medians_ns: rusted_katana_result.medians,
                most_upvoted_medians_ns: most_upvoted.medians,
            }
        })
        .collect();

    serde_json::to_writer_pretty(
        f,
        &JsonResults {
            cpu: &cpu,
            rustc,
            os,
            most_upvoted_author,
            most_upvoted_url,
            benches: &benches,
        },
    )
    .unwrap();

    Ok(())
}

fn get_cpu_model() -> String {
    if let Ok(s) = fs::read_to_string("/proc/cpuinfo") {
        for line in s.lines() {
            if let Some(rest) = line.strip_prefix("model name	: ") {
                return rest.into();
            }
        }
    }
    "unknown".into()
}

fn median<const N: usize>(mut v: [f64; N]) -> f64 {
    const {
        assert!(N % 2 == 1);
    }
    v.sort_unstable_by(f64::total_cmp);
    v[N / 2]
}
