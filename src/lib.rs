#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    use super::module_a;
    #[test]
    fn test_add() {
        assert_eq!(0,module_a::add(0,0));
        assert_eq!(1,module_a::add(1,0));
        assert_eq!(1,module_a::add(0,1));
        assert_eq!(2,module_a::add(1,1));
    }
    #[test]
    fn asset_sample() {
        assert!(true);
        assert_eq!(true,true);
        assert_ne!(true,false);
        //assert_eq!(true,false, "panic! value=({}, {})", true, false);
    }
    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("expected panic");
    }
}

pub mod module_a;
pub mod module_b;