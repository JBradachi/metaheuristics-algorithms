use metaheuristics::{
    annealing,
    bvns::bvns,
    objetivo::ObjetivoFn,
    solucao::{EspacoBusca, Solucao},
};

fn main() {
    let espaco_busca = EspacoBusca::new(-100.0, 100.0);
    let objetivo = ObjetivoFn::f1();
    let solucao_inicial = Solucao::random(objetivo, espaco_busca);
    println!("{:?}", solucao_inicial);

    let solucao_otima = bvns(&solucao_inicial, 12, 20, espaco_busca, objetivo);

    println!("{:?}", solucao_otima);
    println!(
        "{:?}",
        annealing::simulated_annealing(ObjetivoFn::f1(), espaco_busca)
    );
}
