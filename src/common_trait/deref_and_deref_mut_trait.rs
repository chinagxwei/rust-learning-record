use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct DerefExample<T> {
    value: T
}

impl<T> Deref for DerefExample<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        println!("deref");
        &self.value
    }
}

impl<T> DerefMut for DerefExample<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let x = DerefExample { value: 'a' };
        assert_eq!('a', *x);
    }

    #[test]
    fn test_2() {
        let mut x = DerefExample { value: 'a' };
        *x = 'b';
        assert_eq!('b', *x);
    }

    #[test]
    fn test_3() {
        let x = DerefExample { value: 'a' };
        println!("{:?}", x);

        let foo = |x: &char| {};

        //dereference 时指定转换为 &char
        foo(&x);
    }

    #[test]
    fn test_4() {
        let x = DerefExample { value: 'a' };
        println!("{:?}", x);

        let foo = || println!("{:?}", x);

        foo();
    }
}
