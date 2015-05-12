#![feature(collections)]

pub use self::traits::{Something, Entity, Expendable};
pub use self::livings::{Cat, Dog, Human, Goat, God, Satan, Goat666};

#[test]
fn it_works() {
    // Test chaos theory, because the Rust hell need it.
    assert!(666 == 666);
}

#[test]
fn summon_satan() {
    let c = Cat::new("Félix");
    let d = Dog::new("Not Félix");
    let h = Human::new("Jésus");
    let g = Goat::new("It's just a goat !");

    let gg = God::new("God", "Lord of the world");
    let gs = Satan::new("Satan", "King of hell and goat's killer");
    let bg = Goat666::new("Goat ?!", "Oh my god ! It's a goat !");

    c.sacrifice_to(&gs);
    d.sacrifice_to(&gg);
    h.sacrifice_to(&bg);
    g.sacrifice_to(&gs);
}

mod macros;
mod traits;
mod livings;