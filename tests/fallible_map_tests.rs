use fallible_map::{
    FallibleMapExt,
    FallibleMapIteratorExt,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_map_option() {
        let some_number: Option<i32> = Some(2);

        let result: Result<Option<_>, String> = some_number.try_map(|num| {
            if num % 2 == 0 {
                Ok(num * 2)
            } else {
                Err("Odd number".to_string())
            }
        });

        assert_eq!(result, Ok(Some(4)));

        let none_number: Option<i32> = None;

        let result: Result<Option<i32>, String> = none_number.try_map(|num| {
            if num % 2 == 0 {
                Ok(num * 2)
            } else {
                Err("Odd number".to_string())
            }
        });

        assert_eq!(result, Ok(None));
    }

    #[test]
    fn test_try_and_then_option() {
        let some_number: Option<i32> = Some(2);

        let result: Result<Option<_>, String> = some_number.try_and_then(|num| {
            if num % 2 == 0 {
                Ok(Some(num * 2))
            } else {
                Err("Odd number".to_string())
            }
        });

        assert_eq!(result, Ok(Some(4)));

        let none_number: Option<i32> = None;

        let result = none_number.try_and_then(|num| {
            if num % 2 == 0 {
                Ok(Some(num * 2))
            } else {
                Err("Odd number".to_string())
            }
        });

        assert_eq!(result, Ok(None));
    }

    #[test]
    fn test_try_unwrap_or_option() {
        let some_number: Option<i32> = Some(2);

        let result: Result<_, String> = some_number.try_unwrap_or(|| Ok(42));

        assert_eq!(result, Ok(2));

        let none_number: Option<i32> = None;

        let result: Result<_, String> = none_number.try_unwrap_or(|| Ok(42));

        assert_eq!(result, Ok(42));
    }

    #[test]
    fn test_try_map_iterator() {
        let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
        let mapped_numbers: Result<Vec<_>, String> = numbers.into_iter().try_map(|x| {
            if x % 2 == 0 {
                Ok(x * 2)
            } else {
                Err(format!("Failed to process {}", x))
            }
        }).collect();

        assert_eq!(mapped_numbers, Err("Failed to process 1".to_string()));

        let even_numbers: Vec<i32> = vec![2, 4, 6];
        let mapped_even_numbers: Result<Vec<_>, String> = even_numbers.into_iter().try_map(|x| Ok(x * 2)).collect();

        assert_eq!(mapped_even_numbers, Ok(vec![4, 8, 12]));
    }

    #[test]
    fn test_full_usage_example() -> Result<(), String> {
        // FallibleMapExt with Option
        let some_number: Option<i32> = Some(2);
        let result: Result<Option<_>, String> = some_number.try_map(|num| {
            if num % 2 == 0 {
                Ok(num * 2)
            } else {
                Err("Odd number".to_string())
            }
        });

        assert_eq!(result, Ok(Some(4)));

        // FallibleMapIteratorExt with Iterator
        let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
        let mapped_numbers: Result<Vec<_>, String> = numbers.into_iter().try_map(|x| {
            if x % 2 == 0 {
                Ok(x * 2)
            } else {
                Err(format!("Failed to process {}", x))
            }
        }).collect();

        match mapped_numbers {
            Ok(_) => panic!("Expected error"),
            Err(e) => assert_eq!(e, "Failed to process 1"),
        }

        // FallibleMapExt with try_and_then
        let some_number: Option<i32> = Some(2);
        let result: Result<Option<_>, String> = some_number.try_and_then(|num| {
            if num % 2 == 0 {
                Ok(Some(num * 2))
            } else {
                Err("Odd number".to_string())
            }
        });

        assert_eq!(result, Ok(Some(4)));

        Ok(())
    }
}
