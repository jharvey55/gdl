
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

impl<T, V: Solution> Contender<T, V> {
    pub fn new(random_solution: fn() -> V) -> Self {
        let solution = random_solution();
        let fitness = solution.calc_fitness();
        Contender{fitness, solution}
    }

    pub fn get_fitness(&self) -> f64 {
        self.fitness.clone()
    }

    pub fn get_solution(&self) -> V {
        self.solution.clone()
    }
}

impl<T, V: Solution> From<V> for Contender<T, V> {
    fn from(solution: V) -> Self {
        let fitness = solution.calc_fitness();
        Contender{fitness, solution}
    }
}

pub struct Population<T, V: Solution> {
    members: Vec<Contender<T, V>>,
    eval_count: i64,
    pop_size: usize,
}

impl<T,V: Solution>  From<Vec<Contender<T, V>>> for Population<T, V> {
    fn from(members: Vec<Contender<T, V>>) -> Self {
        let pop_size = members.size();
        let eval_count = 0;
        Population { members, eval_count, pop_size}
    }
}