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

    #[test]
    fn should_be_able_to_map_with_noop() {
        struct Noop;
        impl<T> Mapper<T> for Noop {
            type Output = T;
            fn map(value: T) -> Self::Output {
                value
            }
        }

        assert_eq!((1, 2, 3).map(Noop), (1, 2, 3))
    }

    #[test]
    fn should_be_able_to_map_to_string() {
        struct Stringify;
        impl<T: ToString> Mapper<T> for Stringify {
            type Output = String;
            fn map(value: T) -> Self::Output {
                value.to_string()
            }
        }

        assert_eq!(
            (1, 2, 3).map(Stringify),
            ("1".to_string(), "2".to_string(), "3".to_string())
        )
    }

    #[test]
    fn should_be_able_to_sum() {
        struct Sum;
        impl<T: Into<u32>> Reducer<T, u32> for Sum {
            fn reduce(current: T, accumulated: u32) -> u32 {
                let value = current.into();
                value + accumulated
            }
        }
    }
}
