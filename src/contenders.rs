
pub trait Solution {
    type Fitness;
    type Item;

    // Constructors
    fn random_solution() -> Self;


    // dependent constructors
    fn new(source: &Self::Item) -> Self;

    fn create_mutant(parent: &Self) -> Self;
    fn crossover(parent1: &Self, parent2: &Self) -> (Self, Self);


    // expected functionality
    fn create_log_string(&self) -> String;
    fn calc_fitness(&self) -> Self::Fitness;


    // Optional/Default Functionality
    fn dominates(&self, other: &Self) -> bool {
        panic!("Trait not implemented for this struct");
    }

    fn calc_similarity(&self, other: &Self) -> f64 {
        panic!("Trait not implemented for this struct")
    }

    fn fast_similarity(&self, other: &Self) -> f64 {
        panic!("Trait not implemented for this struct")
    }
}

pub struct Contender <T, V: Solution> {
    fitness: T,
    solution: V,
}