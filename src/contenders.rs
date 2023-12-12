
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

impl<T, V: Solution> Population<T, V> {
    pub fn new(random_solution: fn() -> V, pop_size: usize) -> Self {
        let seed = Contender::new(random_solution);
        let mut members = vec![seed];
        for _i in 1..pop_size {
            members.push(Contender::new(random_solution));
        }
        let eval_count: i64 = i64::from(pop_size);
        Population {members, eval_count, pop_size}
    }

    pub fn rank_by_fitness(&mut self) {
        self.members.sort_by(|a, b| b.get_fitness().partial_cmp(&a.get_fitness()).unwrap());
    }

    pub fn increment_eval_count(&mut self) {
        self.eval_count += 1;
    }

    pub fn get_eval_count(&self) -> i64 {
        self.eval_count
    }

    pub fn get_members(&self) -> Vec<Contender<T, V>> {
        self.members.clone()
    }

    pub fn get_pop_size(&self) -> usize {
        self.pop_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}