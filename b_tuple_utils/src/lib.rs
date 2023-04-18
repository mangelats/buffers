use tuple_utils_proc_macros::tuple_ext_impl;

tuple_ext_impl!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_able_to_pluck_values() {
        use super::Pluck;

        let start = (123u32,);
        let result = start.pluck();
    }
}
