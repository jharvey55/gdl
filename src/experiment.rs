use super::contenders::*;

// Optimizer functions

pub fn random_generation<V>(pop: &mut Population<V>) {
    let mut new_pop: Vec<V> = Vec::new();

    for i in 1..pop.pop_size() {
        new_pop.push(V::random_solution());
        pop.increment_eval_count();
    }

    pop.repopulate(new_pop);
}


// technically speaking, this is an implementation of beam search
pub fn rmhc_generation<V>(pop: &mut Population<V>) {
    // let mut new_pop: Vec<V> = Vec::new();

    for mut member in pop.members.iter() {
        member = V::create_mutant(&member);
        pop.increment_eval_count();
    }
}