use super::Monster;

pub enum MonsterType {
    Bat,
    Other
}

impl Monster {
    pub fn construct_from_type(m_type: MonsterType) -> Self {
        match m_type {
            MonsterType::Bat => bat_monster_stats(),
            MonsterType::Other => todo!(),
        }
    }
}

fn bat_monster_stats()->Monster {
    Monster {
        hp: 10.,
        damage: 20.,
        speed: 25.
    }
}