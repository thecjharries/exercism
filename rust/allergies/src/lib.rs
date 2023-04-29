use enum_iterator::{all, Sequence};

#[derive(Debug, PartialEq, Eq)]
pub struct Allergies(Vec<Allergen>);

#[derive(Debug, PartialEq, Eq, Sequence)]
#[repr(u32)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let mut allergens = Vec::new();
        for allergen in all::<Allergen>() {
            if score & allergen as u32 != 0 {
                allergens.push(allergen);
            }
        }
        Allergies(allergens)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        unimplemented!("Return the list of allergens contained within the score with which the Allergies struct was made.");
    }
}
