pub mod entities;

use crate::entities::sandwich::Sandwich;

pub fn created_sandwich() -> Sandwich {
    Sandwich::new(
        "my-sand-1".to_string(),
        "My Custom Sandwich".to_string(),
        vec![
            "Bread".to_string(),
            "Cheese".to_string(),
            "Tomato".to_string(),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_created_sandwich() {
        let sand = created_sandwich();
        assert_eq!(sand.name, "My Custom Sandwich");
        assert_eq!(sand.ingredients.len(), 3);
    }
}
