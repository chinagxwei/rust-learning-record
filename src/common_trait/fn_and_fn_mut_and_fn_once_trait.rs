
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use_fn() {
        let count = 1;
        println!("`count point`: {:p}", &count);
        let  inc = || {
            println!("`in block count point`: {:p}", &count);
            println!("`in block count`: {}", count);
        };
        inc();
        println!("{}",count);
    }

    #[test]
    fn test_use_fn_once() {
        let mut count = 0;
        println!("`count point`: {:p}", &count);
        let mut inc = move|| {
            count += 1;
            println!("`in block count point`: {:p}", &count);
            println!("`in block count`: {}", count);
        };
        // let mut inc = || {
        //     let mut count = count.clone();
        //     count += 1;
        //     println!("`in block count point`: {:p}", &count);
        //     println!("`in block count`: {}", count);
        // };
        inc();
        println!("{}",count);
    }
    
    #[test]
    fn test_use_fn_mut() {
        let mut count = 0;
        println!("`count point`: {:p}", &count);
        let mut inc = || {
            count += 1;
            println!("`in block count point`: {:p}", &count);
            println!("`in block count`: {}", count);
        };
        inc();
        println!("{}",count);
    }
}
