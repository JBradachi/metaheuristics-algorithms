use metaheuristics::{
    bvns,
    annealing,
    hillclimb,
    objetivo::ObjetivoFn,
    solucao::EspacoBusca
};

fn main() {
    let obj1 = ObjetivoFn::f1();
    let esp_a = EspacoBusca::new(100.0);
    let esp_b = EspacoBusca::new(20.0);

    let obj2 = ObjetivoFn::f2();
    let esp_c = EspacoBusca::new(10.0);
    let esp_d = EspacoBusca::new(2.0);

    // Para coletar os dados que o trabalho exige, precisamos rodar cada
    // algoritmo (entre BVNS e SA) para cada uma das funções objetivo, com
    // cada espaço de busca associado
    imprime_algoritmos(obj1, 1, esp_a, 'a');
    imprime_algoritmos(obj1, 1, esp_b, 'b');

    imprime_algoritmos(obj2, 2, esp_c, 'c');
    imprime_algoritmos(obj2, 2, esp_d, 'd');
}

const N: i32 = 30;

fn imprime_algoritmos(obj: ObjetivoFn, n: i32, esp: EspacoBusca, ch: char) {
    println!("Para f{} com o espaço de busca {}", n, ch);
    println!("HC:");
    for _ in 0..N {
        let res = hillclimb::hillclimb_puro(obj, esp);
        println!("{}", res.resultado);
    }
    println!();

    println!("BVNS:");
    for _ in 0..N {
        let res = bvns::bvns(12, 20, esp, obj);
        println!("{}", res.resultado);
    }
    println!();

    println!("SA:");
    for _ in 0..N {
        let res = annealing::simulated_annealing(obj, esp);
        println!("{}", res.resultado);
    }
    println!();
}
