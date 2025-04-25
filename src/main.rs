mod bvns;
mod annealing;
mod solucao;
mod objetivo;

use solucao::Solucao;
use objetivo::ObjetivoFn;

fn main() {
    let solucao_inicialf1 = Solucao::random(ObjetivoFn::f1(), 0.0, 24.0);
    println!("{:?}", solucao_inicialf1);

    let res = bvns::bvns(&solucao_inicialf1, 12, 5);
    println!("{:?}", res);
    println!("{}", annealing::temperatura_inicial(10, ObjetivoFn::f1(), 0.0, 24.0));
}
