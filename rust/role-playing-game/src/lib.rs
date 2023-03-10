#[derive(Debug, Copy, Clone)]
pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if 0 == self.health {
            let mut player = *self;
            player.health = 100;
            if 10 <= player.level {
                player.mana = Some(100);
            }
            Some(player)
        } else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if let Some(mana) = self.mana {
            if mana_cost <= mana {
                self.mana = Some(mana - mana_cost);
                mana_cost * 2
            } else {
                0
            }
        } else {
            if mana_cost > self.health {
                self.health = 0;
            } else {
                self.health -= mana_cost;
            }
            0
        }
    }
}
