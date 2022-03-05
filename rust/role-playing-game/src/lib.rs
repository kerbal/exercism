// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            let mut revive_mana: Option<u32> = None;
            if self.level >= 10 {
                revive_mana = Some(100);
            }
            return Some(Player {
                level: self.level,
                mana: revive_mana,
                health: 100
            })
        }
        return None;
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                if self.health >= mana_cost {
                    self.health -= mana_cost;
                } else {
                    self.health = 0;
                }
                0
            },
            Some(mana) if mana >= mana_cost => {
                self.mana = Some(mana as u32 - mana_cost);
                mana_cost * 2
            },
            Some(mana) => {
                0
            }
        }
    }
}
