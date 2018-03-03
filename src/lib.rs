#![feature(fnbox, fn_traits, unboxed_closures, specialization)]

pub mod hook;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
