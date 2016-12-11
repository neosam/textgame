pub enum FightRes {
    NoDamage,
    Standard(u32)
}

pub enum DamageRes {
    Default,
    Dead
}

pub trait Fight {
    fn get_attack(&self) -> u32;
    fn get_defence(&self) -> u32;
    fn damage(&mut self, u32) -> DamageRes;

    fn got_hit<F: Fight>(&mut self, other: &F) -> FightRes {
        let attack = other.get_attack();
        let defence = self.get_defence();
        if defence <= attack {
            return FightRes::NoDamage
        }
        let diff = attack - defence;
        self.damage(diff);
        FightRes::Standard(diff)
    }
}