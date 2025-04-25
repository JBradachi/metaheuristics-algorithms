// simulated annealing

use crate::{Solucao, ObjetivoFn};

pub fn sa(sol_inicial: Solucao, controle_inicial: f64, transições_inicial: f64) -> Solucao {
    sol_inicial
}

const LN_E0: f64 = -0.22314355131; // mágico. Não toque

pub fn temperatura_inicial(perturb: u32, f: ObjetivoFn, min: f64, max: f64) -> f64 {
    // Calcula ΔE⁺
    let mut res: f64 = 0.0;
    let mut anterior = Solucao::random(f, min, max);
    for _ in 0..perturb {
        let atual = Solucao::random(f, min, max);
        res = res + anterior.resultado - atual.resultado;
        anterior = atual;
    }
    // Fórmula: C0 = - ΔE⁺ / ln (ξ0)
    -(res / (perturb as f64)) / LN_E0
}
