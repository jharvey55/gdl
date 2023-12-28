use super::contenders::*;

// Optimizer functions

enum GenMethod<V> where V: Contender,
{
    Random,
    RMHC,
    GA,
    Custom((String, fn(pop: &mut Population<V>)))
}

pub fn random_generation<V>(pop: &mut Population<V>) {
    for mut member in pop.members.iter() {
        member = V::random_solution();
        pop.increment_eval_count();
    }
}


// technically speaking, this is an implementation of beam search
pub fn rmhc_generation<V>(pop: &mut Population<V>) {
    // let mut new_pop: Vec<V> = Vec::new();

    for mut member in pop.members.iter() {
        member = V::create_mutant(&member);
        pop.increment_eval_count();
    }
}