use crate::hillclimb;
use crate::objetivo::ObjetivoFn;
use crate::solucao::{EspacoBusca, Solucao};
use rand::Rng;
use std::time::Instant;

// const P_ESPACO: f64 = 0.2;

pub fn bvns(
    vizinhanca_max: u64,
    mut tempo_max: u128,
    esp: EspacoBusca,
    fo: ObjetivoFn,
) -> Solucao {
    let mut tempo: u128 = 0;
    // É o raio que será variado entre cada vizinhanca
    let tamanho_passo = 0.01;

    // transforma segundos em ns para fazer a contagem de tempo
    tempo_max = tempo_max * 36000000;

    // Cria estrutura de vizinhancas
    let mut v: Vizinhanca = Vizinhanca::new(fo, vizinhanca_max, tamanho_passo);

    let solucao_inicial = Solucao::random(fo, esp);
    let mut solucao_atual = solucao_inicial.clone();

    while tempo < tempo_max {
        v.reset_estrutura_vizinhanca(); // seta vizinhanca para maior raio

        let start = Instant::now();

        while !v.estruturas_totalmente_exploradas() {
            // gera solução candidata com variação Shake(sol_otima, vizinhanca)
            let sol_candidata: Solucao = varia_solucao(&solucao_atual, esp, &v);

            // faz uma busca local nessa solução candidata
            let sol_candidata_melhorada: Solucao =
                intensifica_solucao(sol_candidata, esp, fo);

            // faz a mudança de vizinhança
            // A funcao já se responsabiliza por alterar os valores das solucoes candidatas e atualizar a vizinnhanca
            muda_vizinhanca(&mut solucao_atual, &sol_candidata_melhorada, &mut v);
        }

        let duracao = start.elapsed();
        tempo = tempo + duracao.as_nanos();
    }
    solucao_atual
}

/// Shake
fn varia_solucao(
    sol_otima: &Solucao,
    espaco_busca: EspacoBusca,
    vizinhanca: &Vizinhanca,
) -> Solucao {
    let solucao_variada = vizinhanca.gera_vizinho_aleatorio(sol_otima, espaco_busca);
    solucao_variada
}

/// Local Search
fn intensifica_solucao(
    sol_candidata: Solucao,
    espaco_busca: EspacoBusca,
    fo: ObjetivoFn,
) -> Solucao {
    hillclimb::hillclimb(sol_candidata, espaco_busca, fo)
}

fn muda_vizinhanca(sol_atual: &mut Solucao, sol_candidata: &Solucao, vizinhanca: &mut Vizinhanca) {
    let resultado_x_atual = sol_atual.resultado;
    let resultado_x_canditato = sol_candidata.resultado;

    if resultado_x_canditato < resultado_x_atual {
        // Atualiza solucao otima
        vizinhanca.reset_estrutura_vizinhanca();
        *sol_atual = sol_candidata.clone();
    } else {
        vizinhanca.atualiza_estrutura_vizinhanca();
    }
}

pub struct Vizinhanca {
    pub funcao_objetivo: ObjetivoFn,
    pub tot_vizinhancas: u64, // diz o total de vizinhancas a serem visitadas
    pub passo_vizinhanca: f64, // diz a distancia do raio entre cada vizinhanca
    pub vizinhanca_atual: f64, // raio da vizinhanca atual
}

impl Vizinhanca {
    pub fn reset_estrutura_vizinhanca(&mut self) {
        self.vizinhanca_atual = self.tot_vizinhancas as f64 * self.passo_vizinhanca;
    }
    pub fn estruturas_totalmente_exploradas(&self) -> bool {
        self.vizinhanca_atual == 0.0
    }
    pub fn atualiza_estrutura_vizinhanca(&mut self) {
        self.vizinhanca_atual = self.vizinhanca_atual - self.passo_vizinhanca;
        if self.vizinhanca_atual < 0.0 {
            self.vizinhanca_atual = 0.0
        }
    }
    pub fn new(funcao_objetivo: ObjetivoFn, tot_vizinhancas: u64, passo_vizinhanca: f64) -> Self {
        let vizinhanca_atual = tot_vizinhancas as f64 * passo_vizinhanca;
        Vizinhanca {
            funcao_objetivo,
            tot_vizinhancas,
            passo_vizinhanca,
            vizinhanca_atual,
        }
    }

    pub fn gera_vizinho_aleatorio(&self, solucao: &Solucao, espaco_busca: EspacoBusca) -> Solucao {
        let mut variaveis: Vec<f64> = Vec::new();
        let size = solucao.variaveis.len();

        variaveis.resize(size, 0.0);

        // Lógica muito complicada ou coisa assim
        let mut na_caixa = false;
        for elemen in variaveis.iter_mut() {
            loop {
                let variacao = rand::thread_rng().gen_range(0.0, self.vizinhanca_atual);
                let neg_or_pos = rand::thread_rng().gen_range(-1.0, 1.0);

                *elemen += if neg_or_pos <= 0.0 {
                    variacao
                } else {
                    self.vizinhanca_atual - variacao
                };
                if espaco_busca.min <= *elemen && *elemen <= espaco_busca.max {
                    if -self.vizinhanca_atual + self.passo_vizinhanca <= *elemen
                        || *elemen >= self.vizinhanca_atual - self.passo_vizinhanca
                    {
                        na_caixa = true
                    };
                    break;
                }
            }
        }
        if !na_caixa {
            // Pulo do gato. Comentário finíssimo
            let var = rand::thread_rng().gen_range(0, size);
            let variacao = rand::thread_rng().gen_range(
                self.vizinhanca_atual - self.passo_vizinhanca,
                self.vizinhanca_atual,
            );
            let neg_or_pos = rand::thread_rng().gen_range(-1.0, 1.0);

            if neg_or_pos <= 0.0 {
                variaveis[var] = -variacao;
            } else {
                variaveis[var] = variacao;
            }
        }

        let new_resultado = Solucao::new(self.funcao_objetivo, variaveis.clone());
        new_resultado
    }
}
