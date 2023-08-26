use std::fs::File;
use std::io::{ Lines, BufReader };

use itertools::Itertools;

type Recipe = (u32, u32, u32, u32);

#[derive(Debug)]
#[derive(PartialEq)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32
}


pub(crate) fn solve(input: Lines<BufReader<File>>) {
    /* 
    Simple solution that iterates over every possible combination
    of ingredients (a "recipe") that adds up to 100 teaspoons, and returns 
    the top score. Constraints from the problem statement are hardcoded into the
    code here, so this solution will not work for different inputs.

    The total number of possible recipes is 156k
    (https://math.stackexchange.com/questions/2592528/compositions-of-4-positive-integers-summing-up-to-100).

    We can avoid performing calculations on recipes that will result in a score of 0
    by observing that recipes have constraints. If these constraints are not met,
    the recipe will be guaranteed to have a score of 0.

    Consider the following input matrix, multiplying a recipe vector.

                        INPUT                  RECIPE
                 spr  pb  fros  sug
    capacity   [  5   -1    0   -1  ]   [ amt_sprinkles    ]
    durability [ -1    3   -1    0  ]   [ amt_peanutbutter ]
    flavor     [  0    0    4    0  ]   [ amt_frosting     ]
    texture    [  0    0    0    2  ]   [ amt_sugar        ]

    The resulting matrix contains the scores for each property for that 
    particular recipe. If any of the scores are negative, they must be 
    converted into 0, and thus the total score will also be 0.

    So, recipes must be constrained in such a way that the proportions
    of ingredients would not result in a negative score for any property.
    In the first row (capacity), as long as the total amount of peanut butter
    and sugar is less than 5x the amount of sprinkles, the calories score
    will always be positive. Also, the amount of sprinkles must be nonzero,
    since it is the only ingredient that contributes positively to the
    capacity score.

    In the second row, the amount of sprinkles and frosting must be less than
    3x the amount of peanut butter in order for the durability score
    to always be positive. Also, the amount of peanut butter must be nonzero
    since it is the only ingredient that contributes positively to the capacity score.

    Flavor will always be positive as long as the amount of frosting is non-zero.

    Texture will always be positive as long as the amount of sugar is non-zero.
    */

    let mut ingredients: Vec<Ingredient> = Vec::new();

    for line in input {
        let ingredient = parse_line(line.unwrap());
        ingredients.push(ingredient);
    }

    let valid_recipes = generate_valid_combinations_for_part_1();
    println!("Count of valid recipes: {}", valid_recipes.len());

    let mut part_1 = 0;
    let mut part_2 = 0;

    for recipe in valid_recipes {
        let (p1, p2) = recipe_score(recipe, &ingredients);
        if p1 > part_1 {
            part_1 = p1;
        }
        if p2 > part_2 {
            part_2 = p2;
        }
    }

    println!("Part 1: Total score of highest-scoring cookie: {}", part_1);
    println!("Part 2: Total score of highest-scoring cookie: {}", part_2);
}

fn recipe_score(recipe: Recipe, ingredients: &Vec<Ingredient>) -> (u32, u32) {
    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;
    let mut calories = 0;


    for ingredient in ingredients {
        let mut amount = recipe.0;
        if ingredient.name == String::from("PeanutButter") {
            amount = recipe.1;
        } else if ingredient.name == String::from("Frosting") {
            amount = recipe.2;
        } else if ingredient.name == String::from("Sugar") {
            amount = recipe.3;
        }
        let amount = i32::try_from(amount).unwrap();
        capacity += amount * ingredient.capacity;
        durability += amount * ingredient.durability;
        flavor += amount * ingredient.flavor;
        texture += amount * ingredient.texture;
        calories += amount * ingredient.calories;
    }

    let total = capacity * durability * flavor * texture;
    if total < 0 {
        return (0, 0);
    } else if calories != 500 {
        return (u32::try_from(total).unwrap(), 0);
    } else {
        let t = u32::try_from(total).unwrap();
        return (t, t);
    }
}


fn generate_valid_combinations_for_part_1() -> Vec<Recipe> {
    let mut valid_recipes: Vec<Recipe> = Vec::new();

    for sprinkles in 1..101 {
        for peanut_butter in 1..101 {
            for frosting in 1..101 {
                for sugar in 1..101 {

                    // constraints
                    if sprinkles + peanut_butter + frosting + sugar != 100 {
                        continue
                    }
                    if peanut_butter + sugar >= 5 * sprinkles {
                        continue
                    }
                    if sprinkles + frosting >= 3 * peanut_butter {
                        continue
                    }
                    // all ingredient amounts must be nonzero

                    valid_recipes.push((sprinkles, peanut_butter, frosting, sugar));
                }
            }
        }
    }

    return valid_recipes;
}


fn parse_line(line: String) -> Ingredient {
    let line = line.replace(",", "");
    let line = line.replace(":", "");
    let s = line.split(" ").collect_vec();

    let name = String::from(s[0]);
    let capacity = s[2].parse::<i32>().unwrap();
    let durability = s[4].parse::<i32>().unwrap();
    let flavor = s[6].parse::<i32>().unwrap();
    let texture = s[8].parse::<i32>().unwrap();
    let calories = s[10].parse::<i32>().unwrap();

    return Ingredient { name, capacity, durability, flavor, texture, calories};
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let line = String::from("Sprinkles: capacity 5, durability -1, flavor 0, texture 0, calories 5");
        let actual = parse_line(line);
        let expected = Ingredient {
            name: String::from("Sprinkles"),
            capacity: 5,
            durability: -1,
            flavor: 0,
            texture: 0,
            calories: 5,
        };
        assert!(expected == actual);
    }
}
