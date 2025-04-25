use rand::Rng;
use super::ObjetivoFn;

#[derive(Clone, Debug)]
pub struct Solucao {
    pub variaveis: Vec<f64>,
    pub resultado: f64,
}

impl Solucao {
    pub fn random(f: ObjetivoFn, min: f64, max: f64) -> Self {
        let mut variaveis = Vec::new();
        for _ in 0..f.num_vars {
            let x = rand::thread_rng().gen_range(min, max);
            variaveis.push(x);
        }
        let resultado = f.call(&variaveis);
        Solucao {
            variaveis,
            resultado,
        }
    }
}
