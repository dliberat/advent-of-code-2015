use std::cmp::Reverse;
use std::fs::File;
use std::io::{ Lines, BufReader };

#[derive(Debug)]
struct Boss {
    hit_points: u32,
    damage: u32,
    armor: u32,
}

#[derive(Debug, Clone)]
struct Item {
    _name: String,
    cost: u32,
    damage: u32,
    armor: u32,
}

#[derive(Debug)]
struct PlayerStats {
    attack: u32,
    defense: u32,
}

#[derive(Debug, Clone)]
struct PlayerEquipment {
    weapon: Item,
    armor: Vec<Item>,
    rings: Vec<Item>,
}

impl PlayerEquipment {
    fn new(weapon: Item, armor: Vec<Item>, rings: Vec<Item>) -> Self {
        assert!(armor.len() < 2);
        assert!(rings.len() < 3);
        Self{weapon, armor, rings}
    }

    fn to_stats(&self) -> PlayerStats {
        assert!(self.armor.len() < 2);
        assert!(self.rings.len() < 3);

        let mut attack = self.weapon.damage;
        let mut defense = self.weapon.armor;
        for a in self.armor.iter() {
            attack += a.damage;
            defense += a.armor;
        }
        for r in self.rings.iter() {
            attack += r.damage;
            defense += r.armor;
        }
        PlayerStats{attack, defense}
    }

    fn cost(&self) -> u32 {
        let mut ttl = self.weapon.cost;
        for a in self.armor.iter() {
            ttl += a.cost;
        }
        for r in self.rings.iter() {
            ttl += r.cost;
        }
        return ttl;
    }
}

#[derive(PartialEq, Debug)]

enum Combatant {
    PLAYER,
    BOSS,
}

pub(crate) fn solve(input: Lines<BufReader<File>>) {

    let boss = parse_input(input);



    let part1 = part_1(&boss);
    println!("Part 1: Least amount of money we can spend and still win: {}", part1);


    let part2 = part_2(&boss);
    println!("Part 2: Most amount of money we can spend and still lose: {}", part2);
}


fn part_1(boss: &Boss) -> u32 {
    let (weapons_inventory, armor_inventory, ring_inventory) = item_shop_inventory();
    let mut combos = all_item_combos(weapons_inventory, armor_inventory, ring_inventory);
    combos.sort_by_cached_key(|x| x.cost());

    for combo in combos {
        let winner = simulate_battle(100, &combo.to_stats(), &boss);
        if winner == Combatant::PLAYER {
            return combo.cost();
        }
    }

    panic!("Battle cannot be won!");
}

fn part_2(boss: &Boss) -> u32 {
    let (weapons_inventory, armor_inventory, ring_inventory) = item_shop_inventory();
    let mut combos = all_item_combos(weapons_inventory, armor_inventory, ring_inventory);
    combos.sort_by_cached_key(|x| Reverse(x.cost()));

    for combo in combos {
        let winner = simulate_battle(100, &combo.to_stats(), &boss);
        if winner == Combatant::BOSS {
            return combo.cost();
        }
    }

    panic!("Battle cannot be won!");
}

fn simulate_battle(player_hp: u32, player_stats: &PlayerStats, boss: &Boss) -> Combatant {
    let mut player_hp = player_hp;
    let mut boss_hp = boss.hit_points;

    let player_damage;
    if player_stats.attack > boss.armor {
        player_damage = player_stats.attack - boss.armor;
    } else {
        player_damage = 1;
    }

    let boss_damage;
    if boss.damage > player_stats.defense {
        boss_damage = boss.damage - player_stats.defense;
    } else {
        boss_damage = 1;
    }

    loop {
        if player_damage < boss_hp {
            boss_hp -= player_damage;
        } else {
            return Combatant::PLAYER;
        }

        if boss_damage < player_hp {
            player_hp -= boss_damage;
        } else {
            return Combatant::BOSS;
        }
    }

}

fn all_item_combos(weapons_inventory: Vec<Item>, armor_inventory: Vec<Item>, ring_inventory: Vec<Item>) -> Vec<PlayerEquipment> {
    // all possibilities for choosing either zero, one, or two rings.
    let mut ring_choices: Vec<Vec<Item>> = Vec::new();

    ring_choices.push(vec!());
    for i in 0..ring_inventory.len() {
        let ring1 = ring_inventory.get(i).unwrap();
        let ring1 = (*ring1).clone();

        ring_choices.push(vec!(ring1.clone()));

        for j in (i+1)..ring_inventory.len() {
            let ring2 = ring_inventory.get(j).unwrap();
            let ring2 = (*ring2).clone();

            ring_choices.push(vec!(ring1.clone(), ring2));
        }
    }

    // all possibilities for item purchases
    let mut combos: Vec<PlayerEquipment> = Vec::new();

    for w in weapons_inventory.iter() {
        // You must buy exactly one weapon
        let weapon = (*w).clone();

        // 1 weapon. 0 armor. 0, 1, or 2 rings.
        for r in ring_choices.iter() {
            combos.push(PlayerEquipment::new(weapon.clone(), vec!() ,(*r).clone()))
        }

        // 1 weapon. 1 armor. 0, 1, or 2 rings.
        for a in armor_inventory.iter() {
            let armor = vec!((*a).clone());

            for r in ring_choices.iter() {
                combos.push(PlayerEquipment::new(weapon.clone(),armor.clone(),(*r).clone()))
            }
        }
    }

    return combos;
}

fn parse_input(input: Lines<BufReader<File>>) -> Boss {
    let mut hit_points = 0;
    let mut damage = 0;
    let mut armor = 0;

    for line in input {
        let v = line.unwrap();
        let split: Vec<&str> = v.split(": ").collect();
        let key = split[0];
        let val = split[1].parse::<u32>().unwrap();

        match key {
            "Hit Points" => hit_points = val,
            "Damage" => damage = val,
            "Armor" => armor = val,
            _ => panic!("Unexpected input: {}", key),
        }    
    }
    Boss{hit_points, damage, armor}
}

fn item_shop_inventory() -> (Vec<Item>, Vec<Item>, Vec<Item>) {
    let weapons = vec!(
        Item{
            _name: String::from("Dagger"),
            cost: 8,
            damage: 4,
            armor: 0},
        Item{
            _name: String::from("Shortsword"),
            cost: 10,
            damage: 5,
            armor: 0},
        Item{
            _name: String::from("Warhammer"),
            cost: 25,
            damage: 6,
            armor: 0},
        Item{
            _name: String::from("Longsword"),
            cost: 40,
            damage: 7,
            armor: 0},
        Item{
            _name: String::from("Greataxe"),
            cost: 74,
            damage: 8,
            armor: 0},
        );
    let armor = vec!(Item{
            _name: String::from("Leather"),
            cost: 13,
            damage: 0,
            armor: 1},
        Item{
            _name: String::from("Chainmail"),
            cost: 31,
            damage: 0,
            armor: 2},
        Item{
            _name: String::from("Splintmail"),
            cost: 53,
            damage: 0,
            armor: 3},
        Item{
            _name: String::from("Bandedmail"),
            cost: 75,
            damage: 0,
            armor: 4},
        Item{
            _name: String::from("Platemail"),
            cost: 102,
            damage: 0,
            armor: 5});
    let rings = vec!(Item{
            _name: String::from("Damage +1"),
            cost: 25,
            damage: 1,
            armor: 0},
        Item{
            _name: String::from("Damage +2"),
            cost: 50,
            damage: 2,
            armor: 0},
        Item{
            _name: String::from("Damage +3"),
            cost: 100,
            damage: 3,
            armor: 0},
        Item{
            _name: String::from("Defense +1"),
            cost: 20,
            damage: 0,
            armor: 1},
        Item{
            _name: String::from("Defense +2"),
            cost: 40,
            damage: 0,
            armor: 2},
        Item{
            _name: String::from("Defense +3"),
            cost: 80,
            damage: 0,
            armor: 3},
    );
    return (weapons, armor, rings);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate_battle_player_wins() {
        let player_stats = PlayerStats{attack: 5, defense: 5};
        let player_hp = 8;

        let boss = Boss{damage: 7, armor: 2, hit_points: 12};

        let result = simulate_battle(player_hp, &player_stats, &boss);
        assert_eq!(Combatant::PLAYER, result);
    }

    #[test]
    fn test_simulate_battle_boss_wins() {
        let player_stats = PlayerStats{attack: 5, defense: 5};
        let player_hp = 8;

        let boss = Boss{damage: 7, armor: 2, hit_points: 13};

        let result = simulate_battle(player_hp, &player_stats, &boss);
        assert_eq!(Combatant::BOSS, result);
    }

}
