pub struct TreeNode {
    pub value: i32,
    pub left: Option<Box<Self>>,
    pub right: Option<Box<Self>>,
}

impl TreeNode {
    pub const fn new(value: i32) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    pub fn with_left(mut self, child: Self) -> Self {
        self.left = Some(Box::new(child));
        self
    }

    pub fn with_right(mut self, child: Self) -> Self {
        self.right = Some(Box::new(child));
        self
    }
}
