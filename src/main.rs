mod bvns;
mod annealing;
mod hillclimb;
mod solucao;
mod objetivo;

use bvns::bvns;
use solucao::Solucao;
use objetivo::ObjetivoFn;

fn main() {
    let espaco_busca = (-100.0, 100.0);
    let solucao_inicial = Solucao::random(ObjetivoFn::f5(), espaco_busca);
    println!("{:?}", solucao_inicial);

    let solucao_otima = bvns(&solucao_inicial, 12, 10, espaco_busca, ObjetivoFn::f5());

    println!("{:?}", solucao_otima);

    //let res = bvns::bvns(&solucao_inicialf1, 12, 5);
    //println!("{:?}", res);
    //println!("{}", annealing::temperatura_inicial(10, ObjetivoFn::f1(), 0.0, 24.0));
}
