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
        unimplemented!("Cast a spell of cost {mana_cost}")
    }
}
