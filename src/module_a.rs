use crate::module_b::module_c;
// use super::module_b::module_c; /*この記法でもいける */
mod module_a {
    pub fn add(left:i32, right:i32) -> i32 {
        left + right
    }
}
///This function adds 2 numbers.
///
/// # Example
/// 
/// ```
/// use hello_crates_and_modules::module_a::add;
///
/// add(1,2);
/// ```
pub fn add(left :i32, right:i32) -> i32 {
    left + right
}