mod annealing;
mod bvns;
mod objetivo;
mod solucao;

use objetivo::ObjetivoFn;
use solucao::Solucao;

fn main() {
    println!(
        "{:?}",
        annealing::simulated_annealing(ObjetivoFn::f1(), -100.0, 100.0)
    );
}
