#![feature(write_all_vectored)]

use std::{
    fs::{self, File, read_dir},
    io::{IoSlice, Write, stderr, stdout},
    path::Path,
};

use digital::prelude::*;
use plotters::prelude::*;
use serde::Deserialize;

#[derive(Default)]
struct Agg {
    speedups: Vec<f64>,
    n: u64,
    sum_my_ns: f64,
    sum_other_ns: f64,
}

#[derive(Deserialize)]
struct JsonBenchResult {
    rusted_katana_medians_ns: [f64; 5],
    most_upvoted_medians_ns: [f64; 5],
}

#[derive(Deserialize)]
struct JsonResults {
    benches: Box<[JsonBenchResult]>,
}

fn main() {
    let mut agg = Agg {
        speedups: Vec::with_capacity(2000),
        ..Default::default()
    };

    for kyu in b'1'..=b'8' {
        let dir_path = [kyu, b'k', b'y', b'u'];
        for kata_dir in read_dir(unsafe { core::str::from_utf8_unchecked(&dir_path) }).unwrap() {
            let kata_dir = kata_dir.unwrap().path();

            let benches_dir = kata_dir.join("benches");
            if !benches_dir.is_dir() {
                stderr()
                    .lock()
                    .write_all_vectored(&mut [
                        IoSlice::new(kata_dir.as_os_str().as_encoded_bytes()),
                        IoSlice::new(b": missing benches/\n"),
                    ])
                    .unwrap();
                continue;
            }

            let results = benches_dir.join("results.json");
            let Ok(results) = fs::read_to_string(results) else {
                stderr()
                    .lock()
                    .write_all_vectored(&mut [
                        IoSlice::new(kata_dir.as_os_str().as_encoded_bytes()),
                        IoSlice::new(b": missing benches/results.json/\n"),
                    ])
                    .unwrap();
                continue;
            };

            parse_results_json(&mut agg, &results, &kata_dir);
        }
    }

    let ews = agg.sum_other_ns / agg.sum_my_ns;

    let rounded_speedup = ((ews + 0.5) as usize).to_heapless_string();
    stdout()
        .lock()
        .write_all_vectored(&mut [
            IoSlice::new(b"Benchmark count: "),
            IoSlice::new(agg.n.to_heapless_string().as_bytes()),
            IoSlice::new(b"\nTotal time speedup: "),
            IoSlice::new(rounded_speedup.as_bytes()),
            IoSlice::new("×\n".as_bytes()),
        ])
        .unwrap();

    let readme = fs::read_to_string("README.md").unwrap();
    let speedup_end = readme.find("×** less").unwrap();
    let speedup_start = readme[..speedup_end].rfind('*').unwrap() + 1;

    let mut readme_file = File::create("README.md").unwrap();
    readme_file
        .write_all_vectored(&mut [
            IoSlice::new(&readme.as_bytes()[..speedup_start]),
            IoSlice::new(rounded_speedup.as_bytes()),
            IoSlice::new(&readme.as_bytes()[speedup_end..]),
        ])
        .unwrap();

    draw_speedup_histogram(&agg.speedups, "speedups.svg");
}

fn parse_results_json(agg: &mut Agg, s: &str, kata_dir: &Path) {
    let results: JsonResults = serde_json::from_str(s).unwrap();
    for bench in results.benches {
        let my_median_ns = median(bench.rusted_katana_medians_ns);
        let other_median_ns = median(bench.most_upvoted_medians_ns);

        let speedup = other_median_ns / my_median_ns;
        if speedup < 1. {
            stderr()
                .lock()
                .write_all_vectored(&mut [
                    IoSlice::new(kata_dir.as_os_str().as_encoded_bytes()),
                    IoSlice::new(b": no speedup over the most upvoted solution\n"),
                ])
                .unwrap();
        }

        agg.n += 1;
        agg.sum_my_ns += my_median_ns;
        agg.sum_other_ns += other_median_ns;
        agg.speedups.push(speedup);
    }
}

fn median<const N: usize>(mut v: [f64; N]) -> f64 {
    const {
        assert!(N % 2 == 1);
    }
    v.sort_unstable_by(f64::total_cmp);
    v[N / 2]
}

fn draw_speedup_histogram(speedups: &[f64], output_path: &str) {
    let bg_col = RGBColor(34, 34, 34);
    let bar_col = RGBColor(187, 67, 44);
    let text_col = RGBColor(221, 221, 221);
    let grid_col = RGBColor(64, 64, 64);

    let root = SVGBackend::new(output_path, (1200, 500)).into_drawing_area();
    root.fill(&bg_col).unwrap();

    let log_speedups: Vec<f64> = speedups.iter().map(|s| s.log10()).collect();
    let x_min = log_speedups
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min)
        .floor();
    let x_max = log_speedups
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max)
        .ceil();

    let n_bins = 60;
    let bin_width = (x_max - x_min) / n_bins as f64;
    let mut bins = vec![0; n_bins];
    for &v in &log_speedups {
        let i = ((v - x_min) / bin_width) as usize;
        bins[i.min(n_bins - 1)] += 1;
    }
    let y_max = *bins.iter().max().unwrap() as f64 * 1.1;

    let mut chart = ChartBuilder::on(&root)
        .margin(40)
        .x_label_area_size(50)
        .y_label_area_size(60)
        .build_cartesian_2d(x_min..x_max, 0f64..y_max)
        .unwrap();

    chart
        .configure_mesh()
        .disable_x_mesh()
        .y_max_light_lines(4)
        .light_line_style(grid_col.mix(0.4))
        .bold_line_style(grid_col.mix(0.6))
        .axis_style(grid_col)
        .x_label_style(("FiraCode", 16).into_font().with_color(text_col))
        .y_label_style(("FiraCode", 16).into_font().with_color(text_col))
        .x_label_formatter(&|&v| {
            if v >= x_max {
                return String::new();
            }
            let v = v as usize;
            let x = '×';
            let mut res = String::with_capacity(1 + v + v / 3 + x.len_utf8());
            res.push('1');
            let mut i = 0;
            while i < v {
                if (v - i).is_multiple_of(3) {
                    res.push('\'');
                }
                res.push('0');
                i += 1;
            }
            res.push(x);
            res
        })
        .y_label_formatter(&|&v| (v as u16).to_string())
        .x_desc("Speedup (log-scale)")
        .y_desc("Number of benchmarks")
        .draw()
        .unwrap();

    chart
        .draw_series(bins.iter().enumerate().map(|(i, &count)| {
            let x0 = bin_width.mul_add(i as _, x_min);
            let x1 = x0 + bin_width * 0.9;
            Rectangle::new([(x0, 0.), (x1, count as f64)], bar_col.filled())
        }))
        .unwrap();

    root.present().unwrap();
}
