use anyhow::Result;

const SPELLS: [Spell; 5] = [
    Spell {
        t: SpellType::MagicMissile,
        cost: 53,
        duration: 1,
        damage: 4,
        heal: 0,
        armor: 0,
        mana_restore: 0,
    },
    Spell {
        t: SpellType::Drain,
        cost: 73,
        duration: 1,
        damage: 2,
        heal: 2,
        armor: 0,
        mana_restore: 0,
    },
    Spell {
        t: SpellType::Shield,
        cost: 113,
        duration: 6,
        armor: 7,
        damage: 0,
        heal: 0,
        mana_restore: 0,
    },
    Spell {
        t: SpellType::Poison,
        cost: 173,
        duration: 6,
        damage: 3,
        heal: 0,
        armor: 0,
        mana_restore: 0,
    },
    Spell {
        t: SpellType::Recharge,
        cost: 229,
        duration: 5,
        mana_restore: 101,
        damage: 0,
        heal: 0,
        armor: 0,
    },
];
fn main() -> Result<()> {
    let player = Character::player();
    let boss = Character::boss();
    let active_spells: Vec<Spell> = Vec::new();

    let result = players_turn(player, boss, active_spells);

    println!("{:?}", result);

    Ok(())
}

fn players_turn(mut player: Character, mut boss: Character, active_spells: Vec<Spell>) -> Option<i32> {
    // hard mode
    player.hp -= 1;
    if player.hp <= 0 {
        return None;
    }

    let active_spells: Vec<Spell> = activate_spells(&mut player, &mut boss, active_spells);

    if boss.hp <= 0 {
        return Some(0);
    }

    let mut min_cost: Option<i32> = None;

    for spell in SPELLS {
        if active_spells.iter().any(|s| s.t == spell.t) {
            continue
        }

        if spell.cost > player.mana {
            continue
        }

        let mut cloned_player = player.clone();
        cloned_player.mana -= spell.cost;

        let mut new_active_spells = active_spells.clone();
        new_active_spells.push(spell.clone());

        let cost = bosses_turn(cloned_player, boss.clone(), new_active_spells).map(|cost| cost + spell.cost);
        min_cost = match (min_cost, cost) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (a, b) => a.or(b)
        }
    }

    min_cost
}

fn bosses_turn(mut player: Character, mut boss: Character, active_spells: Vec<Spell>) -> Option<i32> {
    player.armor = 0;

    let active_spells: Vec<Spell> = activate_spells(&mut player, &mut boss, active_spells);

    if boss.hp <= 0 {
        return Some(0);
    }

    player.hp -= (boss.damage - player.armor).max(1);

    if player.hp <= 0 {
        return None
    }

    players_turn(player, boss, active_spells)
}

fn activate_spells(player: &mut Character, boss: &mut Character, active_spells: Vec<Spell>) -> Vec<Spell> {
    active_spells
        .into_iter()
        .map(|s| {
            player.hp += s.heal;
            player.armor += s.armor;
            player.mana += s.mana_restore;

            boss.hp -= s.damage;

            Spell {
                duration: s.duration - 1,
                ..s
            }
        })
        .filter(|s| s.duration > 0)
        .collect()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum SpellType {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

#[derive(Debug, Clone, Copy)]
struct Spell {
    t: SpellType,
    cost: i32,
    duration: i32,
    damage: i32,
    heal: i32,
    armor: i32,
    mana_restore: i32,
}

#[derive(Debug, Clone, Copy)]
struct Character {
    hp: i32,
    armor: i32,
    damage: i32,
    mana: i32,
}

impl Character {
    fn player() -> Self {
        Self {
            hp: 50,
            armor: 0,
            damage: 0,
            mana: 500,
        }
    }

    fn boss() -> Self {
        Self {
            hp: 71,
            armor: 0,
            damage: 10,
            mana: 0,
        }
    }
}
