

pub enum DamageRes {
    NoDamage,
    Default(u32),
    Dead
}

pub trait Attacker {
    fn get_attack(&self) -> u32;

    fn to_attacker(&self) -> SimpleAttacker {
        SimpleAttacker {
            attack: self.get_attack()
        }
    }
}
pub trait Defender {
    fn get_defence(&self) -> u32;
    fn damage(&mut self, u32) -> DamageRes;

    fn got_hit<A: Attacker>(&mut self, attacker: &A) -> DamageRes {
        let attack = attacker.get_attack();
        let defence = self.get_defence();
        println!("Debug: Attacker: {}, Defender: {}", attack, defence);
        if defence >= attack {
            return DamageRes::NoDamage;
        }
        let diff = attack - defence;
        self.damage(diff)
    }
}

pub trait Fighter: Attacker + Defender {}

pub struct SimpleAttacker {
    attack: u32
}
impl Attacker for SimpleAttacker {
    fn get_attack(&self) -> u32 { self.attack }
}