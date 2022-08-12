use crate::data_struct::binary_tree::TreeNode;

fn recursion_preorder_traversal(node: Option<&Box<TreeNode>>) {
    if node.is_none() {
        return;
    }
    println!("{:#?}", node.unwrap().val);
    recursion_preorder_traversal(node.unwrap().left.as_ref());
    recursion_preorder_traversal(node.unwrap().right.as_ref());
}

fn preorder_traversal(node: Option<&Box<TreeNode>>) {
    if node.is_none() {
        return;
    }
    let mut stack = vec![node.unwrap()];
    while !stack.is_empty() {
        if let Some(e) = stack.pop() {
            if e.right.is_some() {
                stack.push(e.right.as_ref().unwrap());
            }
            if e.left.is_some() {
                stack.push(e.left.as_ref().unwrap())
            }
            println!("{:#?}", e.val);
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
        recursion_preorder_traversal(data.as_ref());
    }

    #[test]
    fn test2() {
        let data = Some(Box::new(get_demo_data()));
        preorder_traversal(data.as_ref());
    }
}
