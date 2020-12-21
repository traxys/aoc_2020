use crate::DayContext;
use std::collections::{HashMap, HashSet};

type Input = Vec<Product>;

fn get_allergens(products: &Input) -> HashMap<String, String> {
    let mut allergen_candidates = HashMap::new();
    for product in products {
        for allergen in &product.allergens {
            match allergen_candidates.get_mut(allergen) {
                None => {
                    allergen_candidates.insert(allergen.clone(), product.ingredients.clone());
                }
                Some(candidates) => {
                    *candidates = candidates
                        .intersection(&product.ingredients)
                        .cloned()
                        .collect();
                }
            }
        }
    }
    let mut allergens = HashMap::new();
    while !allergen_candidates.is_empty() {
        let mut found = Vec::new();
        for (allergen, candidates) in &allergen_candidates {
            if candidates.len() == 1 {
                found.push((
                    allergen.to_owned(),
                    candidates.iter().nth(0).cloned().unwrap(),
                ));
            }
        }

        if found.is_empty() {
            panic!("Could not reduce");
        }
        for (name, ingredient) in found {
            allergen_candidates.remove(&name);
            for (_, candidates) in &mut allergen_candidates {
                candidates.remove(&ingredient);
            }
            allergens.insert(name, ingredient);
        }
    }
    allergens
}

fn non_allergenic(products: &Input, allergens: &HashMap<String, String>) -> HashSet<String> {
    let mut all_products = HashSet::new();
    for product in products {
        all_products = all_products.union(&product.ingredients).cloned().collect();
    }
    for allergen in allergens.values() {
        all_products.remove(allergen);
    }

    all_products
}

pub fn part_1(products: Input) -> color_eyre::Result<String> {
    let allergens = get_allergens(&products);
    let non_allergens = non_allergenic(&products, &allergens);

    let amount: usize = products
        .iter()
        .map(|product| product.ingredients.intersection(&non_allergens).count())
        .sum();

    Ok(format!("Non allergenic appearences: {}", amount))
}

pub fn part_2(products: Input) -> color_eyre::Result<String> {
    let mut allergens: Vec<_> = get_allergens(&products).into_iter().collect();
    allergens.sort_by(|(a1, _), (a2, _)| a1.cmp(a2));
    let mut allergens_list = allergens[0].1.to_owned();
    for (_, allergen) in &allergens[1..] {
        allergens_list += ",";
        allergens_list += allergen;
    }

    Ok(format!("Allergen list is: {}", allergens_list))
}

#[derive(Debug)]
pub struct Product {
    ingredients: HashSet<String>,
    allergens: Vec<String>,
}

#[cfg(test)]
mod test {
    use super::Product;
    use std::collections::HashSet;

    fn load_example() -> Vec<Product> {
        let input = r#"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"#;

        input
            .lines()
            .map(|line| line.parse())
            .collect::<Result<_, _>>()
            .unwrap()
    }

    #[test]
    fn example_non_allergenic() {
        let products = load_example();
        let allergens = super::get_allergens(&products);
        let non_all = super::non_allergenic(&products, &allergens);

        let mut non_all_check = HashSet::new();
        non_all_check.insert("kfcds".to_owned());
        non_all_check.insert("nhms".to_owned());
        non_all_check.insert("sbzzf".to_owned());
        non_all_check.insert("trh".to_owned());
        assert_eq!(non_all, non_all_check)
    }
}

impl std::str::FromStr for Product {
    type Err = color_eyre::eyre::Error;

    fn from_str(line: &str) -> color_eyre::Result<Self> {
        let (ingredients, allergens) = crate::split_string_separator(line, '(')
            .ok_or_else(|| color_eyre::eyre::eyre!("Malformed ingrediens list"))?;
        let allergens = allergens.trim_end_matches(')');
        let allergens = allergens.trim_start_matches("contains ");

        Ok(Product {
            ingredients: ingredients
                .split_ascii_whitespace()
                .map(|s| s.to_owned())
                .collect(),
            allergens: allergens.split(',').map(|s| s.trim().to_owned()).collect(),
        })
    }
}

pub fn parsing(context: &mut DayContext) -> color_eyre::Result<Input> {
    context.parse_lines(|line| line.parse())
}

pub fn execute(context: &mut DayContext) -> color_eyre::Result<()> {
    let input = parsing(context)?;
    context.execute(input, part_1, part_2)
}
