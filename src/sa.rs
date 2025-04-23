// simulated annealing

use crate::Solucao;
use rand::{distributions::{range, Range}, Rng};

pub fn sa(sol_inicial: Solucao, controle_inicial: f64, transições_inicial: f64) -> Solucao {
    sol_inicial
}

// O controle inicial é passado nos slides, coloquei o calculo aqui, checar depois
pub fn gera_crtl_inicial(num_perturbações: u32, xmin: f64, xmax: f64, f: fn(f64, f64) -> f64) -> f64{
    let mut     res: f64 = 0.0;
    let mut x0: f64; let mut x1: f64;
    let mut fo_anterior: f64; let mut fo_atual: f64;
    
    // calcula ΔE⁺
    x0 = rand::thread_rng().gen_range(xmin, xmax);
    x1 = rand::thread_rng().gen_range(xmin, xmax);
    fo_anterior = f(x0,x1);
    for x in 0..num_perturbações {
        x1 = rand::thread_rng().gen_range(xmin, xmax);
        x0 = rand::thread_rng().gen_range(xmin, xmax);
        fo_atual = f(x0,x1);
        res = res + fo_anterior - fo_atual;
        fo_anterior = fo_atual;
    }

    // C0 = - ΔE⁺ / ln (ξ0)
    -(res / (num_perturbações as f64)) / -0.22314355131
}