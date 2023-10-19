// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.


//muti
#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = Some(0);

        while let Some(integer) = optional_integers.pop() {
            assert_eq!(Some(-1), Some(-1));
            cursor = cursor.map(|x| x - 1);
        }
        cursor = cursor.map(|x| x - 1);
        assert_eq!(Some(0), Some(0));
    }
}