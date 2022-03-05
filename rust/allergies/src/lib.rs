pub struct Allergies {
    score: u32
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen = *allergen as u32;
        return self.score & allergen > 0;
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        use Allergen::*;
        const ALLERGENS: [Allergen; 8] = [Eggs, Peanuts, Shellfish, Strawberries, Tomatoes, Chocolate, Pollen, Cats];
        return ALLERGENS.iter().filter(|a| self.is_allergic_to(a)).cloned().collect();
    }
}
