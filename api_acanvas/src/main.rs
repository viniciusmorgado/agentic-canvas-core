use domain_acanvas::created_sandwich;

fn main() {
    let sandwich = created_sandwich();
    println!("Created Sandwich:");
    println!("  Name: {}", sandwich.name);
    println!("  Ingredients:");
    for ingredient in &sandwich.ingredients {
        println!("    - {}", ingredient);
    }
}
