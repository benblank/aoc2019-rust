use std::collections::HashMap;
use std::fs;
use std::io::BufRead;

const INPUT_PATH: &str = "day14.input.txt";
const ORE: &str = "ORE";

#[derive(Debug, PartialEq)]
struct Ingredient {
    name: String,
    count: u32,
}

impl Ingredient {
    fn from_string(string: &str) -> Ingredient {
        let parts = string.split(' ').collect::<Vec<_>>();
        let count = parts[0]
            .parse::<u32>()
            .unwrap_or_else(|_| panic!("invalid ingredient count '{}'", parts[0]));
        let name = parts[1].to_string();

        Ingredient { name, count }
    }
}

#[derive(Debug, PartialEq)]
struct Recipe {
    result: Ingredient,
    ingredients: Vec<Ingredient>,
}

impl Recipe {
    fn from_line(line: &str) -> Recipe {
        let parts = line.split(" => ").collect::<Vec<_>>();
        let result = Ingredient::from_string(parts[1]);
        let ingredients = parts[0]
            .split(", ")
            .map(|ingredient| Ingredient::from_string(ingredient))
            .collect::<Vec<_>>();

        Recipe {
            result,
            ingredients,
        }
    }
}

fn only_need_ore(need: &HashMap<String, u32>) -> bool {
    let elements = need.keys().collect::<Vec<_>>();

    elements.len() == 1 && elements[0] == ORE
}

fn parse_input(input: &[u8]) -> HashMap<String, Recipe> {
    let mut recipes = HashMap::new();

    for line in input.lines() {
        let recipe = Recipe::from_line(&line.expect("input line not valid UTF-8"));

        recipes.insert(recipe.result.name.clone(), recipe);
    }

    recipes
}

fn reduce_to_ore(
    recipes: &HashMap<String, Recipe>,
    elements: HashMap<String, u32>,
) -> HashMap<String, u32> {
    let mut elements = elements;
    let mut extras = HashMap::new();

    while !only_need_ore(&elements) {
        elements = elements
            .iter()
            .flat_map(|(element, count)| {
                if element == ORE {
                    vec![(element.clone(), *count)]
                } else {
                    let recipe = recipes
                        .get(element)
                        .unwrap_or_else(|| panic!("no recipe for '{}'", element));

                    let needed = {
                        let extra = extras.entry(element.clone()).or_default();

                        if *extra >= *count {
                            *extra -= count;

                            0
                        } else {
                            let remaining = *count - *extra;

                            *extra = 0;

                            remaining
                        }
                    };

                    if needed == 0 {
                        Vec::new()
                    } else {
                        let batches = (needed + recipe.result.count - 1) / recipe.result.count;

                        extras
                            .entry(element.clone())
                            .and_modify(|c| *c += recipe.result.count * batches - needed)
                            .or_insert(recipe.result.count * batches - needed);

                        recipe
                            .ingredients
                            .iter()
                            .map(move |ingredient| {
                                (ingredient.name.clone(), ingredient.count * batches)
                            })
                            .collect::<Vec<_>>()
                    }
                }
            })
            .fold(HashMap::new(), |mut elements, (element, count)| {
                elements
                    .entry(element)
                    .and_modify(|value| *value += count)
                    .or_insert(count);

                elements
            });
    }

    elements
}

pub fn part1() {
    let input = fs::read(INPUT_PATH).expect("count not read input file");
    let recipes = parse_input(&input);
    let mut elements = HashMap::new();

    elements.insert("FUEL".to_string(), 1);

    elements = reduce_to_ore(&recipes, elements);

    println!(
        "Ore needed: {}",
        elements.get(ORE).expect("ore entry not found")
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;

    #[test]
    fn parse_input_works() {
        let input = b"10 ORE => 10 A\n1 ORE => 1 B\n7 A, 1 B => 1 C\n7 A, 1 C => 1 D\n7 A, 1 D => 1 E\n7 A, 1 E => 1 FUEL";
        let recipes = hashmap! {
            "A".to_string() => Recipe {
                result: Ingredient {
                    name: "A".to_string(),
                    count: 10,
                },

                ingredients: vec![
                    Ingredient {
                        name: "ORE".to_string(),
                        count: 10,
                    }
                ],
            },

            "B".to_string() => Recipe {
                result: Ingredient {
                    name: "B".to_string(),
                    count: 1,
                },

                ingredients: vec![
                    Ingredient {
                        name: "ORE".to_string(),
                        count: 1,
                    }
                ],
            },

            "C".to_string() => Recipe {
                result: Ingredient {
                    name: "C".to_string(),
                    count: 1,
                },

                ingredients: vec![
                    Ingredient {
                        name: "A".to_string(),
                        count: 7,
                    },

                    Ingredient {
                        name: "B".to_string(),
                        count: 1,
                    }
                ],
            },

            "D".to_string() => Recipe {
                result: Ingredient {
                    name: "D".to_string(),
                    count: 1,
                },

                ingredients: vec![
                    Ingredient {
                        name: "A".to_string(),
                        count: 7,
                    },

                    Ingredient {
                        name: "C".to_string(),
                        count: 1,
                    }
                ],
            },

            "E".to_string() => Recipe {
                result: Ingredient {
                    name: "E".to_string(),
                    count: 1,
                },

                ingredients: vec![
                    Ingredient {
                        name: "A".to_string(),
                        count: 7,
                    },

                    Ingredient {
                        name: "D".to_string(),
                        count: 1,
                    }
                ],
            },

            "FUEL".to_string() => Recipe {
                result: Ingredient {
                    name: "FUEL".to_string(),
                    count: 1,
                },

                ingredients: vec![
                    Ingredient {
                        name: "A".to_string(),
                        count: 7,
                    },

                    Ingredient {
                        name: "E".to_string(),
                        count: 1,
                    }
                ],
            },
        };

        assert_eq!(recipes, parse_input(input));
    }

    #[test]
    fn reduce_to_ore_works_1() {
        let input = b"10 ORE => 10 A\n1 ORE => 1 B\n7 A, 1 B => 1 C\n7 A, 1 C => 1 D\n7 A, 1 D => 1 E\n7 A, 1 E => 1 FUEL";
        let recipes = parse_input(input);
        let mut elements = HashMap::new();

        elements.insert("FUEL".to_string(), 1);

        elements = reduce_to_ore(&recipes, elements);

        assert_eq!(31, *elements.get(ORE).expect("ore entry not found"));
    }

    #[test]
    fn reduce_to_ore_works_2() {
        let input = b"9 ORE => 2 A\n8 ORE => 3 B\n7 ORE => 5 C\n3 A, 4 B => 1 AB\n5 B, 7 C => 1 BC\n4 C, 1 A => 1 CA\n2 AB, 3 BC, 4 CA => 1 FUEL";
        let recipes = parse_input(input);
        let mut elements = HashMap::new();

        elements.insert("FUEL".to_string(), 1);

        elements = reduce_to_ore(&recipes, elements);

        assert_eq!(165, *elements.get(ORE).expect("ore entry not found"));
    }
}
