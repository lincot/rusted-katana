#![feature(test)]

extern crate test;
use fun_with_trees_max_sum::{TreeNode, max_sum};
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        max_sum(black_box(Some(
            &TreeNode::new(5)
                .with_left(
                    TreeNode::new(4)
                        .with_left(TreeNode::new(-80))
                        .with_right(TreeNode::new(-60)),
                )
                .with_right(TreeNode::new(10).with_left(TreeNode::new(-90))),
        )))
    });
}
