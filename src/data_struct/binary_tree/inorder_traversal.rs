use crate::data_struct::binary_tree::TreeNode;

fn recursion_inorder_traversal(node: Option<&Box<TreeNode>>) {
    if node.is_none() {
        return;
    }
    recursion_inorder_traversal(node.unwrap().left.as_ref());
    println!("{:#?}", node.unwrap().val);
    recursion_inorder_traversal(node.unwrap().right.as_ref());
}

fn inorder_traversal(mut node: Option<&Box<TreeNode>>) {
    if node.is_none() {
        return;
    }
    let mut stack = vec![];
    while !stack.is_empty() || node.is_some() {
        if let Some(e) = node {
            stack.push(e);
            node = e.left.as_ref();
        } else {
            node = stack.pop();
            println!("{:#?}", node.unwrap().val);
            node = node.unwrap().right.as_ref();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_struct::binary_tree::get_demo_data;

    #[test]
    fn test() {
        let data = Some(Box::new(get_demo_data()));
        recursion_inorder_traversal(data.as_ref());
    }

    #[test]
    fn test2() {
        let data = Some(Box::new(get_demo_data()));
        inorder_traversal(data.as_ref())
    }
}
