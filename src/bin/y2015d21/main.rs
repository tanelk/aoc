use std::iter;
use std::ops::AddAssign;

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let weapons = [
        Item::offensive(8, 4),
        Item::offensive(10, 5),
        Item::offensive(25, 6),
        Item::offensive(40, 7),
        Item::offensive(74, 8),
    ];
    let armors = [
        Item::defensive(13, 1),
        Item::defensive(31, 2),
        Item::defensive(53, 3),
        Item::defensive(75, 4),
        Item::defensive(102, 5),
    ];
    let rings = [
        Item::offensive(25, 1),
        Item::offensive(50, 2),
        Item::offensive(100, 3),
        Item::defensive(20, 1),
        Item::defensive(40, 2),
        Item::defensive(80, 3),
    ];

    // Must pick 1 weapon, 0 or 1 armor and 0-2 rings
    let armors_iter = armors.iter().map(Some).chain(iter::once(None));
    let rings_iter = rings
        .iter()
        .map(Some)
        .chain(iter::once(None))
        .chain(iter::once(None))
        .combinations(2);

    let cheapest = weapons
        .iter()
        .cartesian_product(armors_iter)
        .cartesian_product(rings_iter)
        .map(|((w, a), r)| {
            let mut res = *w;

            if let Some(a) = a {
                res += *a;
            }

            r.into_iter().flatten().for_each(|r| {
                res += *r;
            });

            res
        })
        .sorted_by_key(|i| i.cost)
        .find(|i| {
            let mut me = Character::new(100, *i);
            let mut boss = Character::new(
                108,
                Item {
                    cost: 0,
                    damage: 8,
                    armor: 2,
                },
            );

            loop {
                me.attack(&boss);

                if boss.hp <= 0 {
                    return true;
                }

                boss.attack(&me);

                if me.hp <= 0 {
                    return false;
                }
            }
        });

    println!("Cheapest option: {:?}", cheapest);

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Item {
    cost: i32,
    damage: i32,
    armor: i32,
}

impl Item {
    fn offensive(cost: i32, damage: i32) -> Self {
        Self {
            cost,
            damage,
            armor: 0,
        }
    }

    fn defensive(cost: i32, armor: i32) -> Self {
        Self {
            cost,
            damage: 0,
            armor,
        }
    }
}

impl AddAssign for Item {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            cost: self.cost + rhs.cost,
            damage: self.damage + rhs.damage,
            armor: self.armor + rhs.armor,
        }
    }
}

struct Character {
    hp: i32,
    equipment: Item,
}

impl Character {
    fn new(hp: i32, equipment: Item) -> Self {
        Character { hp, equipment }
    }

    fn attack(&mut self, other: &Self) {
        self.hp -= (other.equipment.damage - self.equipment.armor).max(1);
    }
}
