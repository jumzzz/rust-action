use rand;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug)]
struct Dwarf {}


#[derive(Debug)]
struct Elf {}


#[derive(Debug)]
struct Human {}

#[derive(Debug)]
enum Thing {
    Sword,
    Trinket,
}

trait Enchanter: std::fmt::Debug {
    fn competency(&self) -> f64;

    fn enchant(&self, thing: &mut Thing) {
        let probability_of_success = self.competency();
        let spell_is_successful = rand::thread_rng()
                                    .gen_bool(probability_of_success);

        
        println!("{:?} yes yes yes. \n", self);

        if spell_is_successful {
            println!("spell is based af = {:?}.", thing);
        } else {
            println!("what a wojak spell = {:?}", thing);
            *thing = Thing::Trinket {};
        }
    }
}

impl Enchanter for Dwarf {
    fn competency(&self) -> f64 {
        0.5
    }
}

impl Enchanter for Elf {
    fn competency(&self) -> f64 {
        0.95
    }
}

impl Enchanter for Human {
    fn competency(&self) -> f64 {
        0.8
    }
}

/// ## Interesting Note:
/// 1. Each polymorphic object must share the same trait. 
/// 2. For each common Trait -> Make an implementation like `impl CommonTrait for others {}`
/// 3. Execute the common "Trait" for each object that has implemented certain traits. 
fn main() {
    let mut it = Thing::Sword;

    let d = Dwarf {};
    let e = Elf {};
    let h = Human {};

    let party: Vec<&dyn Enchanter> = vec![&d, &h, &e];

    // Note: choose is not a default Vec<T> function. This
    // is derived from rand
    let spellcaster = party.choose(&mut rand::thread_rng()).unwrap();

    spellcaster.enchant(&mut it);

}
