

pub enum DamageRes {
    NoDamage,
    Default(u32),
    Dead
}

pub trait Fight {
    fn get_attack(&self) -> u32;
    fn get_defence(&self) -> u32;
    fn damage(&mut self, u32) -> DamageRes;

    fn got_hit<F: Fight>(&mut self, other: &F) -> DamageRes {
        let attack = other.get_attack();
        let defence = self.get_defence();
        println!("Attacker: {}, Defender: {}", attack, defence);
        if defence >= attack {
            return DamageRes::NoDamage;
        }
        let diff = attack - defence;
        self.damage(diff)
    }
}