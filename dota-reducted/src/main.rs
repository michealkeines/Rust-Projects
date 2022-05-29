use rand;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug)]
struct Invoker {}

#[derive(Debug)]
struct Drow {}

#[derive(Debug)]
struct Lina {}

#[derive(Debug)]
enum Thing {
    Sword,
    Trinket
}

trait Enchanter: std::fmt::Debug {
    fn competency(&self) -> f64;

    fn enchant(&self, thing: &mut Thing) {
        let probability_of_success = self.competency();
        let spell_is_successful = rand::thread_rng()
            .gen_bool(probability_of_success);
        
        print!("The {:?} mutters incoherently.", self);

        if spell_is_successful {
            println!("The {:?} glows Brighlty.", thing);
        } else {
            println!("The {:?} fizzes, then turns into a worthless\
             trinket.", thing);
            *thing = Thing::Trinket
        }
    }
}

impl Enchanter for Invoker {
    fn competency(&self) -> f64 {
        0.5
    }
}

impl Enchanter for Drow {
    fn competency(&self) -> f64 {
        0.95
    }
}

impl Enchanter for Lina {
    fn competency(&self) -> f64 {
        0.8
    }
}

fn main() {
    let mut it = Thing::Sword;

    let inovker = Invoker {};
    let drow = Drow {};
    let lina = Lina {};

    let party: Vec<&dyn Enchanter> = vec![&inovker, &drow, &lina];
    let caster = party.choose(&mut rand::thread_rng()).unwrap();

    caster.enchant(&mut it);
}