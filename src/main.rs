extern crate rand;

use std::f64::consts::PI;
use rand::Rng;
use std::time::Instant;

fn main() {
    println!("zorza é paia");

    f1(5.0, 4.0);
    
    println!("{}", f2(5.0, 5.0, 5.0, 5.0));
    
    let solucao_inicialf1 = gera_solucao_inicial(f1, 0.0, 100.0);

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

// Algoritmos que usam metaheurísticas

fn bvns(solucao_inicial: Solucao, vizinhanca_max: u64, tempo_max: u64){
    
    let mut tempo:u64 = 0;
    let mut sol_otima:Solucao = solucao_inicial;

    while tempo < tempo_max {
        
        let mut vizinhanca:u64 = 0;
        let start = Instant::now();
        
        while vizinhanca <= vizinhanca_max {
            // gera solução candidata com variação Shake(sol_otima, vizinhanca)
            // let sol_candidata:Solucao = varia_bvns(sol_otima, vizinhanca);

            // faz uma busca local nessa solução candidata 
            // let sol_candidata_melhorada:Solucao = intensifica_bvns(sol_candidata);

            // faz a mudança de vizinhança

            
        }
        let duracao = start.elapsed();
        tempo = tempo + duracao.as_secs(); 
    }
}

// Estruturas auxiliares
struct Solucao{
    variaveis: Vec<f64>,
    resultado: f64,
}

// Funções auxiliares (TODO: deixar genérico) 

fn gera_solucao_inicial(f: fn(f64, f64) -> f64, xmin: f64, xmax: f64) -> Solucao {
    let x1 = rand::thread_rng().gen_range(xmin, xmax);
    let x2 = rand::thread_rng().gen_range(xmin, xmax);
    let x = Solucao {
        variaveis: vec![x1, x2],
        resultado: f(x1, x2),
    };
    x
}

fn varia_bvns(sol_otima: Solucao, vizinhanca:u64) -> Solucao {
    // TODO: fazer função de variação
    sol_otima
}

fn intensifica_bvns(sol_candidata:Solucao) -> Solucao {
    // TODO: fazer o hill climb
    sol_candidata
}

fn muda_vizinhanca(sol_otima: Solucao, sol_candidata_melhorada:Solucao, vizinhanca:u64) -> (Solucao, u64) {
    // TODO: muda vizinhanca
    (sol_otima, vizinhanca)
}