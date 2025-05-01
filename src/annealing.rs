// O algoritmo de simulated annealing é análogo ao processo físico de
// recozimento de metais. Trabalha-se com um parâmetro de controle denominado
// temperatura, que determina a probabilidade de que uma solução pior que a
// atual seja aceita. Assim, uma temperatura mais alta permite uma
// diversificação da busca.

use rand::Rng;

use crate::objetivo::ObjetivoFn;
use crate::solucao::{EspacoBusca, Solucao};

const LN_E0: f64 = -0.22314355131; // mágico. Não toque

fn temperatura_inicial(perturb: u32, f: ObjetivoFn, esp: EspacoBusca) -> f64 {
    // Calcula ΔE+
    let mut res: f64 = 0.0;
    let mut anterior = Solucao::random(f, esp);
    for _ in 0..perturb {
        let atual = Solucao::random(f, esp);
        res += (anterior.resultado - atual.resultado).abs();
        anterior = atual;
    }
    // Fórmula: T0 = - ΔE+ / ln (ξ0)
    -(res / (perturb as f64)) / LN_E0
}

fn metropolis(current: &Solucao, candidate: &Solucao, temperatura: f64) -> bool {
    let p = rand::thread_rng().next_f64();
    ((current.resultado - candidate.resultado) / temperatura).exp() > p
}

const TEMP_FATOR: f64 = 0.9;

pub fn simulated_annealing(f: ObjetivoFn, esp: EspacoBusca) -> Solucao {
    let mut s = Solucao::random(f, esp); // solução atual
    let mut best_solution = s.clone(); // melhor solução encontrada
    let mut num_iters = 1000; // número de iterações por temperatura
    let mut temperatura = temperatura_inicial(10, f, esp);

    // Uma temperatura muito baixa sinaliza o fim do algoritmo
    while temperatura >= 0.001 {
        for _ in 0..num_iters {
            let p = s.shake(f, esp);
            if p.resultado < best_solution.resultado {
                best_solution = p.clone();
            }
            // Lógica principal; devemos aplicar o critério de metropolis?
            if p.resultado < s.resultado || metropolis(&s, &p, temperatura) {
                s = p.clone();
            }
        }
        temperatura *= TEMP_FATOR;
        num_iters += 100;
    }
    best_solution
}
