//! <https://www.codewars.com/kata/57e5279b7cf1aea5cf000359/train/rust>

pub use preloaded::TreeNode;

mod preloaded;

pub fn max_sum(tree: Option<&TreeNode>) -> i32 {
    tree.map_or(0, max_sum_)
}

fn max_sum_(tree: &TreeNode) -> i32 {
    tree.value
        + match (&tree.left, &tree.right) {
            (None, None) => 0,
            (Some(left), None) => max_sum_(left.as_ref()),
            (None, Some(right)) => max_sum_(right.as_ref()),
            (Some(left), Some(right)) => max_sum_(left.as_ref()).max(max_sum_(right.as_ref())),
        }
}
