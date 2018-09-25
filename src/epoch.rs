use population::LazyUnit;
use rand::distributions::{IndependentSample, Range};
use rand::Rng;
use std::cmp::Ordering;
use unit::Unit;

pub trait Epoch<T>
where
    T: Unit,
{
    fn epoch(&self, active_stack: &mut Vec<LazyUnit<T>>, rng: &mut impl Rng) -> bool;
}

#[derive(Debug)]
pub struct DefaultEpoch {
    breed_factor: f64,
    survival_factor: f64,
}
impl DefaultEpoch {
    pub fn new(breed_factor: f64, survival_factor: f64) -> DefaultEpoch{
        DefaultEpoch{breed_factor, survival_factor}
    }
}

impl Default for DefaultEpoch{
    fn default() -> Self {
        DefaultEpoch::new(0.2, 0.5)
    }
}

impl<T: Unit> Epoch<T> for DefaultEpoch {
    fn epoch(&self, active_stack: &mut Vec<LazyUnit<T>>, rng: &mut impl Rng) -> bool {
        // We want to sort such that highest fitness units are at the
        // end.
        active_stack.sort_by(|a, b| {
            a.lazy_fitness
                .unwrap_or(0.0)
                .partial_cmp(&b.lazy_fitness.unwrap_or(0.0))
                .unwrap_or(Ordering::Equal)
        });

        let units = active_stack;
        let max_size = units.len();
        assert!(!units.is_empty());

        // breed_factor dicates how large a percentage of the population will be
        // able to breed.
        let breed_up_to = (self.breed_factor * (units.len() as f64)) as usize;
        let mut breeders: Vec<LazyUnit<T>> = Vec::new();

        while let Some(unit) = units.pop() {
            breeders.push(unit);
            if breeders.len() == breed_up_to {
                break;
            }
        }
        units.clear();

        // The strongest half of our breeders will survive each epoch. Always at
        // least one.
        let surviving_parents = (breeders.len() as f64 * self.survival_factor).ceil() as usize;

        let pcnt_range = Range::new(0, breeders.len());
        for i in 0..max_size - surviving_parents {
            let rs = pcnt_range.ind_sample(rng);
            units.push(LazyUnit::from(
                breeders[i % breeders.len()]
                    .unit
                    .breed_with(&breeders[rs].unit),
            ));
        }

        // Move our survivors into the new generation.
        units.append(&mut breeders.drain(0..surviving_parents).collect());

        true
    }
}


