use once_cell::sync::Lazy;
use sonyflake::Sonyflake;

pub static SNOWFLAKE: Lazy<Sonyflake> = Lazy::new(|| Sonyflake::new().unwrap());

/// Initializes the snowflake generator.
pub fn next_id() -> crate::Result<String> {
    SNOWFLAKE
        .next_id()
        .map(|id| id.to_string())
        .map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_id() {
        for index in 0..100 {
            let id = next_id().unwrap();
            println!("id{index}: {}", id);
            assert_eq!(id.len(), 18);
        }
    }
}
