mod solucao;
mod bvns;
mod sa;
extern crate rand;

use std::f64::consts::PI;
use solucao::Solucao;

fn main() {
    println!("zorza é paia");

    let solucao_inicialf1:Solucao = Solucao::gera_solucao_inicial(f1, 0.0, 24.0);
    solucao_inicialf1.mostra_solucao();
    
    let res = bvns::bvns(solucao_inicialf1, 12, 5);
    res.mostra_solucao();
}

// Funções objetivo

fn f1(x1: f64, x2:f64) -> f64{
    let dist = f64::sqrt(x1*x1+x2*x2);
    1.0 - f64::cos(2.0 * PI * dist) + 0.1 * dist
}

fn f2(x1: f64, x2: f64, x3: f64, x4: f64) -> f64{
    100.0 * (x1*x1 - x2).powi(2) + (x1 - 1.0).powi(2) +
    (x3 - 1.0).powi(2) + 90.0 * (x3*x3 - x4).powi(2) + 
    10.1 * ((x2 - 1.0).powi(2) + (x4 - 1.0).powi(2)) + 
    19.8 * (x2 - 1.0) * (x4 - 1.0) 
}

