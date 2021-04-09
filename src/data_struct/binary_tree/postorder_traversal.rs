use crate::data_struct::binary_tree::TreeNode;

fn recursion_postorder_traversal(node: Option<&Box<TreeNode>>) {
    if node.is_none() {
        return;
    }
    recursion_postorder_traversal(node.unwrap().left.as_ref());
    recursion_postorder_traversal(node.unwrap().right.as_ref());
    println!("{:#?}", node.unwrap().val);
}

fn postorder_traversal(mut node: Option<&Box<TreeNode>>) {
    if node.is_none() {
        return;
    }
    let mut stack = vec![];
    let mut last_node = None;
    // let mut node = root;
    while node.is_some() || !stack.is_empty() {
        while node.is_some() {
            stack.push(node.unwrap());
            node = node.unwrap().left.as_ref()
        }

        let tmp = stack[stack.len() - 1];
        if tmp.right.is_none() || tmp.right.as_ref() == last_node {
            stack.pop();
            println!("{:#?}", tmp.val);
            last_node = Some(tmp)
        } else {
            node = tmp.right.as_ref()
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
        recursion_postorder_traversal(data.as_ref());
    }

    #[test]
    fn test2() {
        let data = Some(Box::new(get_demo_data()));
        postorder_traversal(data.as_ref());
    }
}
