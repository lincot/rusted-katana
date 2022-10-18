#![no_std]
#![feature(test)]

extern crate test;
use counting_ascii_art_squares::count_squares;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(count_squares(black_box(&[
            "+--+  +----+",
            "|  |  |    |    +-+",
            "|  |  +----+    | |",
            "+--+            +-+",
        ])));
        black_box(count_squares(black_box(&["+-----+", "|     |", "+-----+"])));
        black_box(count_squares(black_box(&[
            "+-+-+", "| | |", "+-+-+", "| | |", "+-+-+",
        ])));
        black_box(count_squares(black_box(&[
            "+---+", "|   |", "|   |", "|   |", "|   |", "|   |", "|   |", "|   |", "+---+",
        ])));
        black_box(count_squares(black_box(&[
            "+---+", "|   |", "|   |", "|   |", "+---+",
        ])));
        black_box(count_squares(black_box(&[
            "+---+", "|   |", "|  ++--+", "|  ||  |", "+--++  |", "   |   |", "   +---+",
        ])));
        black_box(count_squares(black_box(&[
            "   +---+", "   |   |", "+--++  |", "|  ||  |", "|  ++--+", "|   |", "+---+",
        ])));
        black_box(count_squares(black_box(&[
            "+---+",
            "|   |",
            "|   |  +---+",
            "|   |  |   |",
            "+---+  |   |",
            "       |   |",
            "       +---+",
        ])));
        black_box(count_squares(black_box(&[
            "+---+---+",
            "|   |   |",
            "|   |   |",
            "|   |   |",
            "+---+---+",
            "|   |   |",
            "|   |   |",
            "|   |   |",
            "+---+---+",
        ])));
        black_box(count_squares(black_box(&[
            "+----+--+",
            "|    |  |",
            "|    |  |",
            "|    |  |",
            "+----+--+",
            "|    |  |",
            "|    |  |",
            "|    |  |",
            "+----+--+",
        ])));
        black_box(count_squares(black_box(&[
            "+---+---+",
            "|   |   |",
            "|   |   |",
            "|   |   |",
            "|   |   |",
            "+---+---+",
            "|   |   |",
            "|   |   |",
            "+---+---+",
        ])));
        black_box(count_squares(black_box(&[
            "+---+---+",
            "|   |   |",
            "| +-+-+ |",
            "| | | | |",
            "+-+-+-+-+",
            "| | | | |",
            "| +-+-+ |",
            "|   |   |",
            "+---+---+",
        ])));
        black_box(count_squares(black_box(&[
            "  +---+",
            "  |   |",
            "  |   |  +--+",
            "+-+-+ |  |  |",
            "| +-+-+  |  |",
            "+---+    +--+",
        ])));
        black_box(count_squares(black_box(&[
            "+---+",
            "|   |",
            "|   |",
            "+--+|+--+",
            "+--++|  |",
            "+--+-+--+",
            "   | |",
            "   | |",
            "   +-+",
        ])));
        black_box(count_squares(black_box(&[
            "+---------+--+",
            "|  +---+  |  |",
            "|  |   |  |  |",
            "|  |   |  +--+",
            "|  |   |     |",
            "|  +---+     |",
            "|            |",
            "|            |",
            "|            |",
            "|            |",
            "|    +---+---+",
            "|    |   |   |",
            "|    |   |   |",
            "+----+---+---+",
            "     +---+",
        ])));
        black_box(count_squares(black_box(&["++", "++"])));
        black_box(count_squares(black_box(&["+"])));
        black_box(count_squares(black_box(&[
            "   +--+",
            "   |  |",
            "   |  |",
            "+--+--+--+",
            "|  |  |  |",
            "|  |  |  |",
            "+--+--+--+",
            "   |  |",
            "   |  |",
            "   +--+",
        ])));
        black_box(count_squares(black_box(&[
            "+--+  +--+",
            "|  |  |  |",
            "|  |  |  |",
            "+--+--+--+",
            "   |  |",
            "   |  |",
            "+--+--+--+",
            "|  |  |  |",
            "|  |  |  |",
            "+--+  +--+",
        ])));
        black_box(count_squares(black_box(&[
            "   +--+  +--+",
            "   |  |  |  |",
            "   |  |  |  |",
            "+--+--+--+--+",
            "|  |  |  |",
            "|  |  |  |",
            "+--+--+--+--+",
            "   |  |  |  |",
            "   |  |  |  |",
            "+--+--+--+--+",
            "|  |  |  |",
            "|  |  |  |",
            "+--+  +--+",
        ])));
        black_box(count_squares(black_box(&[
            "+-+ +-+", "| | | |", "+-+ +-+", "", "+-+ +-+", "| | | |", "+-+ +-+",
        ])));
        black_box(count_squares(black_box(&[
            "+-+---+", "| |   |", "| |   |", "+-+-+-+", "| | | |", "| | | |", "+-+ +-+",
        ])));
        black_box(count_squares(black_box(&[
            "++++++++", "++++++++", "++++++++", "++++++++", "++++++++", "++++++++", "++++++++",
            "++++++++",
        ])));
        black_box(count_squares(black_box(&[
            "",
            "    +--+",
            " +--++ |   +-+",
            " |  || |   | |",
            " |  ++-+---+-+",
            " |   | |   |",
            " +---+-+   |",
            "       |   |",
            "       +---+",
            "",
            "+---+",
            "|   |",
            "|   |",
            "|   |",
            "+---+",
        ])));
    });
}
