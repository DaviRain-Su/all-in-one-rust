//! # error handler example
//!
//! Rust 中的错误处理逻辑主要依赖于两种类型：`Result` 和 `Option`。这两种类型在 Rust 中被广泛用于处理可能会失败的操作。
//!
//! - `Result`: 这是一个枚举类型，有两个可能的变体：`Ok` 和 `Err`。当一个函数或操作可能会失败时，它应该返回一个 `Result` 类型。如果操作成功，它返回 `Ok` 变体，包含成功的结果。如果操作失败，它返回 `Err` 变体，包含一个错误消息或错误类型。
//!
//! - `Option`: 这也是一个枚举类型，有两个可能的变体：`Some` 和 `None`。当一个函数或操作可能不会返回结果时（例如，查找字典中的键），它应该返回一个 `Option` 类型。如果操作找到了结果，它返回 `Some` 变体，包含找到的值。如果没有找到结果，它返回 `None` 变体。
//!
//! 在 Rust 中处理这两种类型的常见模式是使用 `match` 语句或者 `?` 操作符。
//!
//! - `match` 语句允许你检查 `Result` 或 `Option` 的变体，并根据变体执行不同的操作。这是一个非常明确的错误处理方式，但有时可能会导致代码过于冗长。
//!
//! - `?` 操作符可以用在返回 `Result` 或 `Option` 的函数中。如果 `Result` 是 `Ok` 或 `Option` 是 `Some`，它将解包并返回结果。如果 `Result` 是 `Err` 或 `Option` 是 `None`，它将立即从当前函数返回错误或 None。这是一种更简洁的错误处理方式，但可能会使错误处理的逻辑不那么明显。
//!
//! 这就是 Rust 中的错误处理逻辑。通过结合使用 `Result`、`Option`、`match` 语句和 `?` 操作符，你可以创建强大且灵活的错误处理逻辑，同时保持代码的简洁和可读性。
use std::collections::HashMap;

/// 下面是一个使用 `Option` 和 `?` 操作符的例子。假设我们有一个哈希映射，我们需要找出一个特定的键的值，然后对该值进行某种操作。
///
/// 在上述例子中，如果键在映射中找到，`get` 方法将返回 `Some(value)`，然后 `?` 操作符将解包 `Some` 并将 `value` 传递给下一行代码。如果键没有在映射中找到，`get` 方法将返回 `None`，然后 `?` 操作符将立即从函数返回 `None`。
///
///注意，使用 `?` 操作符的函数必须返回 `Result` 或 `Option` 类型。这是因为 `?` 需要一个地方来返回错误或 `None`。如果函数返回一个非 `Result` 或 `Option` 类型，编译器会报错。
pub fn process_value(map: &HashMap<String, i32>, key: &str) -> Option<i32> {
    let value = map.get(key)?;

    // 如果找到了键，我们就对值进行某种操作，例如加一
    Some(value + 1)
}

pub fn process_value_result(map: &HashMap<String, i32>, key: &str) -> Result<i32, String> {
    let value = map.get(key).ok_or::<String>("empty error".into())?;

    // 如果找到了键，我们就对值进行某种操作，例如加一
    Ok(value + 1)
}

/// 下面是一个使用 `Resul`t 和 `?` 操作符的例子。假设我们有一个函数，它尝试将一个字符串解析为整数。
///
/// 在上述例子中，`parse` 方法尝试将字符串 `s` 解析为一个整数。如果解析成功，`parse` 返回 `Ok(number)`，然后 ? 操作符解包 `Ok` 并将 `number` 传递给下一行代码。如果解析失败，`parse` 返回 `Err`，然后 `?` 操作符立即从函数返回该错误。
pub fn parse_integer(s: &str) -> Result<i32, std::num::ParseIntError> {
    let number = s.parse::<i32>()?;
    Ok(number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_value() {
        let mut map = HashMap::new();
        map.insert("one".to_string(), 1);
        map.insert("two".to_string(), 2);

        assert_eq!(process_value(&map, "one"), Some(2));
        assert_eq!(process_value(&map, "two"), Some(3));
        assert_eq!(process_value(&map, "three"), None);
    }

    #[test]
    fn test_process_value_result() {
        let map = HashMap::new();
        assert!(process_value_result(&map, "three").is_err());
        println!("error : {:?}", process_value_result(&map, "three"));
    }

    #[test]
    fn test_parse_integer() {
        assert_eq!(parse_integer("10"), Ok(10));
        assert!(parse_integer("a").is_err());
    }
}
