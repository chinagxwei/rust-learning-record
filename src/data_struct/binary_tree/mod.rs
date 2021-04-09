mod preorder_traversal;
mod inorder_traversal;
mod postorder_traversal;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct TreeNode {
    val: u32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}


fn get_demo_data() -> TreeNode {
    TreeNode {
        val: 15,
        left: Some(
            Box::from(TreeNode {
                val: 6,
                left: Some(Box::from(TreeNode {
                    val: 4,
                    left: Some(Box::from(TreeNode {
                        val: 2,
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::from(TreeNode {
                        val: 5,
                        left: None,
                        right: None,
                    })),
                })),
                right: Some(Box::from(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                })),
            })
        ),
        right: Some(Box::from(TreeNode {
            val: 23,
            left: Some(Box::from(TreeNode {
                val: 19,
                left: None,
                right: None,
            })),
            right: Some(Box::from(TreeNode {
                val: 71,
                left: None,
                right: None,
            })),
        })),
    }
}
