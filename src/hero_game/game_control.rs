use crate::hero_game::basic_struct::ActiveEntitiesInfo;

pub fn game_start(){
    let mut hero = ActiveEntitiesInfo::new("kin");
    println!("Hero is: {:#?}", hero);

    let mut enemy = ActiveEntitiesInfo::new("Slime");

    while enemy.alive(){
        hero.attack(&mut enemy);
    }

    println!("Success kill enemy: {:#?}", enemy);
}