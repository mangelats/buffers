use tuple_utils_proc_macros::tuple_ext_impl;

tuple_ext_impl!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_able_to_pluck_values() {
        let start = (123u32,);
        let result = start.pluck();
        assert_eq!(result.0, 123u32);
        assert_eq!(result.1, ());
    }

    #[test]
    fn should_be_able_to_pluck_values_and_keep_the_rest() {
        let start = (123u32, 'a', "abc");
        let result = start.pluck();
        assert_eq!(result.0, 123u32);
        assert_eq!(result.1, ('a', "abc"));
    }
}
