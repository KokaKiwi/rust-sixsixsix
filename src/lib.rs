pub use self::traits::{Something, Entity, Expendable};
pub use self::livings::{Cat, Dog, Human, Goat, michaelsproul, Kitty};
pub use self::livings::{God, Satan, Goat666, Manishearth};

mod macros;
pub mod traits;
pub mod livings;

#[test]
fn it_works() {
    // Test chaos theory, because the Rust hell need it.
    assert!(666 == 666);
}

#[test]
fn summon_satan() {
    let c = Cat::new("Felix");
    let d = Dog::new("Not Felix");
    let h = Human::new("Jesus");
    let g = Goat::new("It's just a goat !");
    let m = michaelsproul::new("I'm a Rust contributor");
    let kittens = vec![Kitty::new("Felix"), Kitty::new("Rosti"), Kitty::new("Fungus"), Kitty::new("Blue waffle")];

    let gg = God::new("God", "Lord of the world");
    let gs = Satan::new("Satan", "King of hell and goat's killer");
    let bg = Goat666::new("Goat ?!", "Oh my god ! It's a goat !");
    let mh = Manishearth::new("Manishearth", "YOU DARE WAKE ME FROM MY SLUMBER??!");

    c.sacrifice_to(&gs);
    d.sacrifice_to(&gg);
    h.sacrifice_to(&bg);
    g.sacrifice_to(&gs);
    m.sacrifice_to(&mh);
    kittens.sacrifice_to(&gg);
}
