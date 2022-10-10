#![crate_name = "ripgross"]
#![crate_type = "rlib"]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod Style;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
