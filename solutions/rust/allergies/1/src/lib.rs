
pub struct Allergies{
    allergies: Vec<Allergen>
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

use Allergen::*;
const ALIST: [Allergen; 8] = [Eggs, Peanuts, Shellfish, Strawberries, Tomatoes, Chocolate, Pollen, Cats];

impl Allergies {
    
    pub fn new(score: u32) -> Self {

        let mut allergens = Vec::new();

        let mut score = score;
        for item in ALIST {
            let index = score % 2;
            if index == 1 {
                allergens.push(item);
            }
            score = (score - index) / 2;
        }

        Allergies { allergies: allergens }

    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergies.clone()
    }
}

