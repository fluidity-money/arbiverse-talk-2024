#[derive(Debug, Clone)]
pub enum Creature {
    Donkey,
    Dog,
    Human,
}

pub fn is_pet(c: &Creature) -> bool {
    match *c {
        Creature::Donkey => true,
        Creature::Dog | Creature::Human => false,
    }
}

#[cfg(test)]
use proptest::prelude::*;

#[cfg(test)]
proptest! {
    #[test]
    fn test_is_pet(
        animal in prop_oneof![
            Just(Creature::Donkey),
            Just(Creature::Dog),
            Just(Creature::Human)
        ])
    {
        assert_eq!(is_pet(&animal), match animal {
            Creature::Donkey | Creature::Dog => true,
            Creature::Human => false
        });
    }
}
