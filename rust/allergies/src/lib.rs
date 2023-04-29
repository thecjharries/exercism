pub struct Allergies;

#[derive(Debug, PartialEq, Eq)]
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
        unimplemented!("Given the '{score}' score, construct a new Allergies struct.");
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        unimplemented!("Determine if the patient is allergic to the '{allergen:?}' allergen.");
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        unimplemented!("Return the list of allergens contained within the score with which the Allergies struct was made.");
    }
}
