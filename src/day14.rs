use std::collections::HashMap;
use std::fs;
use std::io::BufRead;

const INPUT_PATH: &str = "day14.input.txt";
const ORE: &str = "ORE";

#[derive(Debug, PartialEq)]
struct Ingredient {
    name: String,
    count: u64,
}

impl Ingredient {
    fn from_string(string: &str) -> Ingredient {
        let parts = string.split(' ').collect::<Vec<_>>();
        let count = parts[0]
            .parse::<u64>()
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

fn count_fuel_made(ore_limit: u64, recipes: &HashMap<String, Recipe>) -> u64 {
    let single_fuel_cost = get_ore_cost(&recipes, 1);

    let mut min = ore_limit / single_fuel_cost;
    let mut max = 2 * min;

    loop {
        let target = (min + max) / 2;
        let cost = get_ore_cost(&recipes, target);
        let cost_of_plus_one = get_ore_cost(&recipes, target + 1);

        if cost <= ore_limit && cost_of_plus_one > ore_limit {
            break target;
        }

        if cost < ore_limit {
            min = target;
        } else {
            max = target;
        }
    }
}

fn get_ore_cost(recipes: &HashMap<String, Recipe>, fuel_count: u64) -> u64 {
    let mut elements = HashMap::new();

    elements.insert("FUEL".to_string(), fuel_count);
    elements = reduce_to_ore(&recipes, elements);

    *elements.get(ORE).expect("ore entry not found")
}

fn only_need_ore(need: &HashMap<String, u64>) -> bool {
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
    elements: HashMap<String, u64>,
) -> HashMap<String, u64> {
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
    let fuel_cost = get_ore_cost(&recipes, 1);

    println!("Ore needed: {}", fuel_cost);
}

pub fn part2() {
    let input = fs::read(INPUT_PATH).expect("count not read input file");
    let recipes = parse_input(&input);
    let fuel_created = count_fuel_made(1_000_000_000_000, &recipes);

    println!("Fuel created: {}", fuel_created);
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;

    #[test]
    fn count_fuel_made_works() {
        let input = b"157 ORE => 5 NZVS\n165 ORE => 6 DCFZ\n44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n179 ORE => 7 PSHF\n177 ORE => 5 HKGWZ\n7 DCFZ, 7 PSHF => 2 XJWVT\n165 ORE => 2 GPVTF\n3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        let recipes = parse_input(input);
        let fuel_created = count_fuel_made(1_000_000_000_000, &recipes);

        assert_eq!(82_892_753, fuel_created);
    }
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

    #[test]
    fn reduce_to_ore_works_3() {
        let input = b"157 ORE => 5 NZVS\n165 ORE => 6 DCFZ\n44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n179 ORE => 7 PSHF\n177 ORE => 5 HKGWZ\n7 DCFZ, 7 PSHF => 2 XJWVT\n165 ORE => 2 GPVTF\n3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        let recipes = parse_input(input);
        let mut elements = HashMap::new();

        elements.insert("FUEL".to_string(), 1);

        elements = reduce_to_ore(&recipes, elements);

        assert_eq!(13312, *elements.get(ORE).expect("ore entry not found"));
    }
}
