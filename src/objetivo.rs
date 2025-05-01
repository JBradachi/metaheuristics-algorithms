// Define as funções objetivo usadas no trabalho. São duas funções: uma simples,
// que recebe dois parâmetros, e uma mais complicada, que recebe quatro. Para
// poder tratar essas funções de maneira genérica e uniforme, ambas são
// implementadas recebendo referências a vetores. A estrutura ObjetivoFn
// encapsula um ponteiro para função com o número de variáveis que a função
// espera dentro do vetor

use core::f64;
use std::f64::consts::PI;

fn f1(x: &Vec<f64>) -> f64 {
    let x1 = x[0];
    let x2 = x[1];
    let dist = (x1 * x1 + x2 * x2).sqrt();
    1.0 - (2.0 * PI * dist).cos() + 0.1 * dist
}

fn f2(x: &Vec<f64>) -> f64 {
    let x1 = x[0];
    let x2 = x[1];
    let x3 = x[2];
    let x4 = x[3];
    100.0 * (x1 * x1 - x2).powi(2)
        + (x1 - 1.0).powi(2)
        + (x3 - 1.0).powi(2)
        + 90.0 * (x3 * x3 - x4).powi(2)
        + 10.1 * ((x2 - 1.0).powi(2) + (x4 - 1.0).powi(2))
        + 19.8 * (x2 - 1.0) * (x4 - 1.0)
}

// Funcao concava (ver imagens)
fn f3(x: &Vec<f64>) -> f64 {
    let x1 = x[0];
    let x2 = x[1];
    (x1 + 2.0) * (x1 + 2.0) + (x2 + 2.0) * (x2 + 2.0)
}

// Regiao de pingo d´agua (ver imagens)
fn f4(x: &Vec<f64>) -> f64 {
    let x1 = x[0];
    let x2 = x[1];
    let raio = ((x1 * x1) + (x2 * x2)).sqrt();
    let numerador = 2.0 * (5.0 + raio).sin();
    let denominador = 1.0 + raio / 10.0;
    numerador / denominador
}
// Regiao sen_cos (ver imagens)
fn f5(x: &Vec<f64>) -> f64 {
    let x1 = x[0];
    let x2 = x[1];
    let expr1 = (2.0 * x1).sin() / x1;
    let expr2 = (2.0 * x2).sin() / x2;
    let raio = ((x1 * x1) + (x2 * x2)).sqrt();
    let den = 1.0 + raio / 5.0;
    (-expr1 - expr2) / den
}

#[derive(Clone, Copy)]
pub struct ObjetivoFn {
    pub f: fn(&Vec<f64>) -> f64,
    pub num_vars: u32,
}

impl ObjetivoFn {
    pub fn f1() -> Self {
        ObjetivoFn { f: f1, num_vars: 2 }
    }

    pub fn f2() -> Self {
        ObjetivoFn { f: f2, num_vars: 4 }
    }

    pub fn f3() -> Self {
        ObjetivoFn { f: f3, num_vars: 2 }
    }

    pub fn f4() -> Self {
        ObjetivoFn { f: f4, num_vars: 2 }
    }

    pub fn f5() -> Self {
        ObjetivoFn { f: f5, num_vars: 2 }
    }

    pub fn call(&self, x: &Vec<f64>) -> f64 {
        let f = self.f;
        f(x)
    }
}
