use super::Monster;

pub enum MonsterType {
    Bat,
    Mushroom,
}

impl Monster {
    pub fn construct_from_type(m_type: MonsterType) -> Self {
        match m_type {
            MonsterType::Bat => bat_monster_stats(),
            MonsterType::Mushroom => mushroom_monster_stats(),
        }
    }
}

fn bat_monster_stats() -> Monster {
    Monster {
        hp: 15.0,
        damage: 20.0,
        speed: 25.0,
    }
}

fn mushroom_monster_stats() -> Monster {
    Monster {
        hp: 15.0,
        damage: 20.0,
        speed: 40.0,
    }
}
