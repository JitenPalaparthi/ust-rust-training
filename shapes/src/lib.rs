pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greet(){
    println!("Hello shapes package")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
