use crate::hero_game::basic_struct::ActiveEntitiesInfo;

impl ActiveEntitiesInfo {
    pub fn new(name: &str) -> ActiveEntitiesInfo {
        ActiveEntitiesInfo {
            name: String::from(name),
            ap: 10,
            dp: 1,
            exp: 10,
            hp: 100,
            level: 1,
        }
    }

    pub fn attack(&mut self, enemy: &mut ActiveEntitiesInfo){
        let damage = self.get_ap() - enemy.get_dp();
        enemy.hp -= damage;
        enemy.hp = std::cmp::max(0, enemy.hp);
    }

    pub fn alive(&self)-> bool{
        self.hp != 0
    }

    pub fn get_ap(&self)-> i32{
        self.ap
    }

    pub fn get_dp(&self)-> i32{
        self.dp
    }
}
