#![allow(invalid_value)]
#![feature(sync_unsafe_cell)]

use core::{cmp::Ordering, mem::MaybeUninit};
use digital::{MaxLenBase10, WriteNumUnchecked};
use image::{Rgb, RgbImage};
use imageproc::{
    drawing::{draw_filled_rect_mut, draw_text_mut},
    rect::Rect,
};
use rusttype::{Font, Scale};
use std::{
    fs,
    io::{self, Write},
    thread,
};

struct SendPtr<T>(*const T);
impl<T> SendPtr<T> {
    const unsafe fn deref_ptr(&self) -> &T {
        &*self.0
    }
}
unsafe impl<T> Send for SendPtr<T> {}

struct SendMutPtr<T>(*mut T);
impl<T> SendMutPtr<T> {
    unsafe fn assign(self, val: T) {
        *self.0 = val;
    }
}
unsafe impl<T> Send for SendMutPtr<T> {}

struct ProgressBars<'a> {
    image: RgbImage,
    font: &'a Font<'a>,
    bar_height: u32,
    vertical_padding: u32,
    font_width: u32,
    font_height: u32,
    scale: Scale,
}

impl<'a> ProgressBars<'a> {
    fn new(font: &'a Font, width: u32, bar_height: u32, vertical_padding: u32) -> Self {
        let mut image = RgbImage::new(
            width,
            (bar_height + vertical_padding) * 8 - vertical_padding,
        );
        let font_width = bar_height * 2 / 5;

        for i in 1..8 {
            let y = ((bar_height + vertical_padding) * i - vertical_padding) as _;
            draw_filled_rect_mut(
                &mut image,
                Rect::at(0, y).of_size(width, vertical_padding),
                Rgb([34, 34, 34]),
            );
        }

        Self {
            image,
            font,
            bar_height,
            vertical_padding,
            font_width,
            font_height: font_width * 2,
            scale: Scale::uniform((font_width * 2) as _),
        }
    }

    fn write_progress(mut self, progresses: &[u32; 8], limits: &[u32; 8]) -> RgbImage {
        let width = self.image.width();

        for (i, (&progress, &limit)) in (0..).zip(progresses.iter().zip(limits)) {
            let y = ((self.bar_height + self.vertical_padding) * i as u32) as _;

            let progress_width = progress * width / limit;
            draw_filled_rect_mut(
                &mut self.image,
                Rect::at(0, y).of_size(progress_width, self.bar_height),
                Rgb([187, 67, 44]),
            );
            match width.cmp(&progress_width) {
                Ordering::Greater => draw_filled_rect_mut(
                    &mut self.image,
                    Rect::at(progress_width as _, y)
                        .of_size(width - progress_width, self.bar_height),
                    Rgb([64, 64, 64]),
                ),
                Ordering::Less => {
                    let mut stdout = io::stdout().lock();
                    let mut text = *b"0 kyu probably has retired katas\n";
                    text[0] = i + b'1';
                    stdout.write_all(&text).unwrap();
                }
                Ordering::Equal => {}
            }

            let mut kyu_text = *b"0 kyu";
            kyu_text[0] = i + b'1';
            let kyu_text = unsafe { core::str::from_utf8_unchecked(&kyu_text) };
            draw_text_mut(
                &mut self.image,
                Rgb([255, 255, 255]),
                (self.font_width / 2) as _,
                y + ((self.bar_height - self.font_height) / 2) as i32,
                self.scale,
                self.font,
                kyu_text,
            );

            let mut progress_text = heapless::String::<{ 2 * u32::MAX_LEN_BASE10 + 1 }>::new();
            unsafe {
                progress_text.write_num_unchecked(progress, 10, false, false);
                progress_text.as_mut_vec().push_unchecked(b'/');
                progress_text.write_num_unchecked(limit, 10, false, false);
            }
            draw_text_mut(
                &mut self.image,
                Rgb([255, 255, 255]),
                ((width - progress_text.len() as u32 * self.font_width) / 2) as _,
                y + ((self.bar_height - self.font_height) / 2) as i32,
                self.scale,
                self.font,
                &progress_text,
            );
        }

        self.image
    }
}

fn get_kata_amount(reqwest_client: &reqwest::blocking::Client, kyu: u8) -> reqwest::Result<u32> {
    let mut url = *b"https://www.codewars.com/kata/search/rust?q=&r[]=-8&xids=completed&beta=false&order_by=published_at%20asc";
    url["https://www.codewars.com/kata/search/rust?q=&r[]=-".len()] = b'0' + kyu;
    let url = unsafe { core::str::from_utf8_unchecked(&url) };
    let page = reqwest_client.get(url).send()?.text()?;
    let number_end = page.find(" Kata Found<").unwrap();
    let number_start = number_end
        - unsafe { page.as_bytes().get_unchecked(..number_end) }
            .iter()
            .rev()
            .position(|&b| b == b'>')
            .unwrap();

    Ok(unsafe { page.get_unchecked(number_start..number_end) }
        .parse()
        .unwrap())
}

fn main() {
    let reqwest_client = Box::leak(Box::new(reqwest::blocking::Client::new()));

    let limits = Box::leak(Box::new(unsafe {
        MaybeUninit::<[u32; 8]>::uninit().assume_init()
    }));
    let network_tasks = [1, 2, 3, 4, 5, 6, 7, 8].map(|kyu| {
        let p = SendMutPtr(core::ptr::from_mut(&mut limits[(kyu - 1) as usize]));
        let r = SendPtr(core::ptr::from_ref(&reqwest_client));
        thread::spawn(move || unsafe { p.assign(get_kata_amount(r.deref_ptr(), kyu).unwrap()) })
    });

    let mut progresses = unsafe { MaybeUninit::<[u32; 8]>::uninit().assume_init() };
    for kyu in 1..=8 {
        let mut folder = *b"8kyu";
        folder[0] = b'0' + kyu;
        let folder = unsafe { core::str::from_utf8_unchecked(&folder) };
        progresses[(kyu - 1) as usize] = fs::read_dir(folder).unwrap().count() as _;
    }

    let font =
        Font::try_from_bytes(include_bytes!("/usr/share/fonts/TTF/FiraCode-Bold.ttf")).unwrap();
    let progress_bars = ProgressBars::new(&font, 360, 30, 5);

    for task in network_tasks {
        task.join().unwrap();
    }

    progress_bars
        .write_progress(&progresses, limits)
        .save("progress-bars.png")
        .unwrap();
}
