use rand::Rng;
use super::ObjetivoFn;

#[derive(Clone, Debug)]
pub struct Solucao {
    pub variaveis: Vec<f64>,
    pub resultado: f64,
}

impl Solucao {
    pub fn random(f: ObjetivoFn, espaco_busca: (f64, f64)) -> Self {
        let mut variaveis = Vec::new();
        for _ in 0..f.num_vars {
            let x = rand::thread_rng().gen_range(espaco_busca.0, espaco_busca.1);
            variaveis.push(x);
        }
        let resultado = f.call(&variaveis);
        Solucao {
            variaveis,
            resultado,
        }
    }
    pub fn evaluate(f: ObjetivoFn, x: &Vec<f64>) -> Self {
        let variaveis = x.clone();
        let resultado = f.call(&x);
        Solucao { variaveis, resultado }
    }
}
