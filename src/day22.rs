use std::collections::VecDeque;
use std::fs::File;
use std::io::{ Lines, BufReader };
use self::PlayerAction::*;

const MIN_SPELL_COST: u32 = 53;
const MAGIC_MISSILE_COST: u32 = 53;
const DRAIN_COST: u32 = 73;
const SHIELD_COST: u32 = 113;
const POISON_COST: u32 = 173;
const RECHARGE_COST: u32 = 229;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
enum PlayerAction {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl PlayerAction {
    fn iterator() -> impl Iterator<Item = PlayerAction> {
        [MagicMissile, Drain, Shield, Poison, Recharge].iter().copied()
    }
}

#[derive(Debug, Clone)]
struct Boss {
    hit_points: u32,
    damage: u32,
}


#[derive(Debug, Clone)]
struct Player {
    mana: u32,
    hit_points: u32,
    armor: u32,

    shield_effect: u32,
    poison_effect: u32,
    recharge_effect: u32,

    mana_spent: u32,
}


impl Player {
    fn new(mana: u32, hit_points: u32) -> Self {
        Player{
            mana, 
            hit_points,
            armor: 0,
            shield_effect: 0,
            poison_effect: 0,
            recharge_effect: 0,
            mana_spent:0
        }
    }

    fn take_turn(&mut self, action: PlayerAction) -> u32 {
        match action {
            MagicMissile => self.cast_magic_missile(),
            Drain => self.cast_drain(),
            Shield => self.cast_shield(),
            Poison => self.cast_poison(),
            Recharge => self.cast_recharge(),
        }
    }

    fn cast_magic_missile(&mut self) -> u32 {
        self.mana -= 53;
        self.mana_spent += 53;
        4
    }

    fn cast_drain(&mut self) -> u32 {
        self.mana -= 73;
        self.mana_spent += 73;
        self.hit_points += 2;
        2
    }

    fn cast_shield(&mut self) -> u32 {
        assert!(self.shield_effect == 0);
        self.mana -= SHIELD_COST;
        self.mana_spent += SHIELD_COST;
        self.shield_effect = 6;
        self.armor += 7;
        0
    }

    fn cast_poison(&mut self) -> u32 {
        assert!(self.poison_effect == 0);
        self.mana -= POISON_COST;
        self.mana_spent += POISON_COST;
        self.poison_effect = 6;
        0
    }

    fn cast_recharge(&mut self) -> u32 {
        assert!(self.recharge_effect == 0);
        self.mana -= RECHARGE_COST;
        self.mana_spent += RECHARGE_COST;
        self.recharge_effect = 5;
        0
    }

    fn can_perform_action(&self, action: PlayerAction) -> bool {
        match action {
            MagicMissile => self.mana >= MAGIC_MISSILE_COST,
            Drain => self.mana >= DRAIN_COST,
            Shield => self.shield_effect <= 1 && self.mana >= SHIELD_COST,
            Poison => self.poison_effect <= 1 && self.mana >= POISON_COST,
            Recharge => self.recharge_effect <= 1 && self.mana >= RECHARGE_COST,
        }
    }

}

#[derive(PartialEq, Debug)]
enum Combatant {
    PLAYER,
    BOSS,
}

#[derive(Clone, Debug)]
struct Game {
    player: Player,
    boss: Boss,
    hard_mode: bool,
}

impl Game {
    fn winner(&self) -> Option<Combatant> {
        if self.player.hit_points == 0 {
            return Option::Some(Combatant::BOSS);
        }
        if self.boss.hit_points == 0 {
            return Option::Some(Combatant::PLAYER);
        }
        return Option::None;
    }

    fn apply_effects(&mut self) {
        if self.player.shield_effect > 0 {
            self.player.shield_effect -= 1;
            if self.player.shield_effect == 0 {
                self.player.armor -= 7;
            }
        }
        if self.player.poison_effect > 0 {
            self.boss.hit_points -= 3;
            self.player.poison_effect -= 1;
        }
        if self.player.recharge_effect > 0 {
            self.player.mana += 101;
            self.player.recharge_effect -= 1;
        }
    }

    fn player_turn(&mut self, action: PlayerAction) -> Option<Combatant> {
        if self.hard_mode {
            self.player.hit_points -= 1;
            if self.player.hit_points <= 0 {
                return Option::Some(Combatant::BOSS);
            }
        }

        self.apply_effects();
        let winner = self.winner();
        if winner.is_some() {
            return winner;
        } else if self.player.mana < MIN_SPELL_COST {
            return Option::Some(Combatant::BOSS);
        }

        let damage = self.player.take_turn(action);
        if damage > self.boss.hit_points {
            self.boss.hit_points = 0;
        } else {
            self.boss.hit_points -= damage;
        }
        self.winner()
    }

    fn boss_turn(&mut self) -> Option<Combatant> {
        self.apply_effects();
        let winner = self.winner();
        if winner.is_some() {
            return winner;
        }

        let dmg = match self.boss.damage > self.player.armor {
            true => self.boss.damage - self.player.armor,
            false => 1,
        };

        match dmg <= self.player.hit_points {
            true => self.player.hit_points -= dmg,
            false => self.player.hit_points = 0,
        }
            
        self.winner()
    }
}

pub(crate) fn solve(input: Lines<BufReader<File>>) {

    let boss = parse_input(input);

    let part1 = part_1(&boss);
    println!("Part 1: Least amount of mana we can spend and still win: {}", part1);


    let part2 = part_2(&boss);
    println!("Part 2: Least amount of mana we can spend and still win: {}", part2);
}


fn part_1(boss: &Boss) -> u32 {
    let player = Player::new(500, 50);
    let game = Game{player, boss: boss.clone(), hard_mode: false};
    let mut lowest_mana_cost_of_victory = u32::MAX;

    let mut queue: VecDeque<Game> = VecDeque::new();
    queue.push_back(game);


    while !queue.is_empty() {
        let current_state = queue.pop_front().unwrap();

        for action in PlayerAction::iterator() {
            if !current_state.player.can_perform_action(action) {
                continue;
            }

            let mut g = current_state.clone();
            let winner = g.player_turn(action);

            if g.player.mana_spent >= lowest_mana_cost_of_victory {
                continue;
            }

            if winner.is_some() {
                let winner = winner.unwrap();
                if winner == Combatant::PLAYER && 
                    g.player.mana_spent < lowest_mana_cost_of_victory {
                        lowest_mana_cost_of_victory = g.player.mana_spent;
                    }
                continue;
            }
            let winner = g.boss_turn();

            if winner.is_some() {
                let winner = winner.unwrap();
                if winner == Combatant::PLAYER && 
                    g.player.mana_spent < lowest_mana_cost_of_victory {
                        lowest_mana_cost_of_victory = g.player.mana_spent;
                    }
                continue;
            }

            queue.push_back(g);
        }
    }
    
    return lowest_mana_cost_of_victory;
}

fn part_2(boss: &Boss) -> u32 {
    let player = Player::new(500, 50);
    let game = Game{player, boss: boss.clone(), hard_mode: true};
    let mut lowest_mana_cost_of_victory = u32::MAX;

    let mut queue: VecDeque<Game> = VecDeque::new();
    queue.push_back(game);


    while !queue.is_empty() {
        let current_state = queue.pop_front().unwrap();

        for action in PlayerAction::iterator() {
            if !current_state.player.can_perform_action(action) {
                continue;
            }

            let mut g = current_state.clone();
            let winner = g.player_turn(action);

            if g.player.mana_spent >= lowest_mana_cost_of_victory {
                continue;
            }

            if winner.is_some() {
                let winner = winner.unwrap();
                if winner == Combatant::PLAYER && 
                    g.player.mana_spent < lowest_mana_cost_of_victory {
                        lowest_mana_cost_of_victory = g.player.mana_spent;
                    }
                continue;
            }
            let winner = g.boss_turn();

            if winner.is_some() {
                let winner = winner.unwrap();
                if winner == Combatant::PLAYER && 
                    g.player.mana_spent < lowest_mana_cost_of_victory {
                        lowest_mana_cost_of_victory = g.player.mana_spent;
                    }
                continue;
            }

            queue.push_back(g);
        }
    }
    
    return lowest_mana_cost_of_victory;
}


fn parse_input(input: Lines<BufReader<File>>) -> Boss {
    let mut hit_points = 0;
    let mut damage = 0;

    for line in input {
        let v = line.unwrap();
        let split: Vec<&str> = v.split(": ").collect();
        let key = split[0];
        let val = split[1].parse::<u32>().unwrap();

        match key {
            "Hit Points" => hit_points = val,
            "Damage" => damage = val,
            _ => panic!("Unexpected input: {}", key),
        }    
    }
    Boss{hit_points, damage}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cast_magic_missile() {
        let player = Player::new(250, 10);
        let boss = Boss{hit_points: 13, damage: 8};
        let mut game = Game{player, boss, hard_mode: false};

        let result = game.player_turn(PlayerAction::MagicMissile);
        assert!(result.is_none());
        assert_eq!(9, game.boss.hit_points);
        assert_eq!(10, game.player.hit_points);
        assert_eq!((250-MAGIC_MISSILE_COST), game.player.mana);
        assert_eq!(MAGIC_MISSILE_COST, game.player.mana_spent);
        assert_eq!(0, game.player.shield_effect);
        assert_eq!(0, game.player.poison_effect);
        assert_eq!(0, game.player.recharge_effect);
    }

    #[test]
    fn test_cast_magic_missile_hard_mode() {
        let player = Player::new(250, 10);
        let boss = Boss{hit_points: 13, damage: 8};
        let mut game = Game{player, boss, hard_mode: true};

        let result = game.player_turn(PlayerAction::MagicMissile);
        assert!(result.is_none());
        assert_eq!(9, game.boss.hit_points);
        assert_eq!(9, game.player.hit_points);
        assert_eq!((250-MAGIC_MISSILE_COST), game.player.mana);
        assert_eq!(MAGIC_MISSILE_COST, game.player.mana_spent);
        assert_eq!(0, game.player.shield_effect);
        assert_eq!(0, game.player.poison_effect);
        assert_eq!(0, game.player.recharge_effect);
    }

    #[test]
    fn test_example_1() {
        let player = Player::new(250, 10);
        let boss = Boss{hit_points: 13, damage: 8};
        let mut game = Game{player, boss, hard_mode: false};

        let result = game.player_turn(PlayerAction::Poison);
        assert!(result.is_none());
        assert_eq!(13, game.boss.hit_points);
        assert_eq!(10, game.player.hit_points);
        assert_eq!(77, game.player.mana);
        assert_eq!(POISON_COST, game.player.mana_spent);
        assert_eq!(0, game.player.shield_effect);
        assert_eq!(6, game.player.poison_effect);
        assert_eq!(0, game.player.recharge_effect);

        let result = game.boss_turn();
        assert!(result.is_none(), "game ended unexpectedly {:?}", game);
        assert_eq!(10, game.boss.hit_points);
        assert_eq!(2, game.player.hit_points);
        assert_eq!(77, game.player.mana);
        assert_eq!(POISON_COST, game.player.mana_spent);
        assert_eq!(0, game.player.shield_effect);
        assert_eq!(5, game.player.poison_effect);
        assert_eq!(0, game.player.recharge_effect);

        let result = game.player_turn(PlayerAction::MagicMissile);
        assert!(result.is_none(), "game ended unexpectedly {:?}", game);
        assert_eq!(3, game.boss.hit_points);
        assert_eq!(2, game.player.hit_points);
        assert_eq!(24, game.player.mana);
        assert_eq!(POISON_COST+MAGIC_MISSILE_COST, game.player.mana_spent);
        assert_eq!(0, game.player.shield_effect);
        assert_eq!(4, game.player.poison_effect);
        assert_eq!(0, game.player.recharge_effect);

        let result = game.boss_turn();
        assert!(result.is_some());
        assert_eq!(Combatant::PLAYER, result.unwrap());
        assert_eq!(0, game.boss.hit_points);
        assert_eq!(2, game.player.hit_points);
        assert_eq!(24, game.player.mana);
        assert_eq!(POISON_COST+MAGIC_MISSILE_COST, game.player.mana_spent);
        assert_eq!(0, game.player.shield_effect);
        assert_eq!(3, game.player.poison_effect);
        assert_eq!(0, game.player.recharge_effect);

    }

    #[test]
    fn test_recharge_hard_mode() {
        let player = Player::new(500, 10);
        let boss = Boss{hit_points: 13, damage: 8};
        let mut game = Game{player, boss, hard_mode: true};

        let result = game.player_turn(PlayerAction::Recharge);
        assert!(result.is_none());
        assert_eq!(13, game.boss.hit_points);
        assert_eq!(9, game.player.hit_points);
        assert_eq!((500-RECHARGE_COST), game.player.mana);
        assert_eq!(RECHARGE_COST, game.player.mana_spent);
        assert_eq!(0, game.player.shield_effect);
        assert_eq!(0, game.player.poison_effect);
        assert_eq!(5, game.player.recharge_effect);

        let result = game.boss_turn();
        assert!(result.is_none());
        assert_eq!(13, game.boss.hit_points);
        assert_eq!(1, game.player.hit_points);
        assert_eq!((500-RECHARGE_COST+101), game.player.mana);
        assert_eq!(RECHARGE_COST, game.player.mana_spent);
        assert_eq!(0, game.player.shield_effect);
        assert_eq!(0, game.player.poison_effect);
        assert_eq!(4, game.player.recharge_effect);

        let result = game.player_turn(PlayerAction::MagicMissile);
        assert!(result.is_some());
        assert_eq!(Combatant::BOSS, result.unwrap());
        assert_eq!(13, game.boss.hit_points);
        assert_eq!(0, game.player.hit_points);
        assert_eq!((500-RECHARGE_COST+101), game.player.mana);
        assert_eq!(RECHARGE_COST, game.player.mana_spent);
        assert_eq!(0, game.player.shield_effect);
        assert_eq!(0, game.player.poison_effect);
        assert_eq!(4, game.player.recharge_effect);
    }
}
