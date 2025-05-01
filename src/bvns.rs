use crate::Solucao;
use std::time::Instant;
use rand::Rng;
use crate::ObjetivoFn;
use std::f64::consts::PI;
use crate::hillclimb;

pub fn bvns(solucao_inicial: &Solucao, vizinhanca_max: u64, mut tempo_max: u128, 
            espaco_busca: (f64, f64), fo: ObjetivoFn) -> Solucao {

    let mut tempo: u128 = 0;
    // TODO: Conferir onde é o melhor lugar para se deixar essa variavel
    let tamanho_passo = 1.0; // é o raio que será variado entre cada vizinhanca
    print!("SOLUCAO INICIAL: {:?}\n", solucao_inicial);

    // transforma segundos em ns para fazer a contagem de tempo
    tempo_max = tempo_max * 36000000;

    // Cria estrutura de vizinhancas
    let mut v: Vizinhanca = Vizinhanca::new(fo, vizinhanca_max, tamanho_passo);

    let mut solucao_atual = solucao_inicial.clone();

    while tempo < tempo_max {
        v.reset_estrutura_vizinhanca(); // seta vizinhanca para maior raio

        let start = Instant::now();

        while !v.estruturas_totalmente_exploradas() {

            //print!("\n-----------------------\n");
            // gera solução candidata com variação Shake(sol_otima, vizinhanca)
            let sol_candidata: Solucao = varia_solucao(&solucao_atual, &espaco_busca, &v);
            //print!("Solucao variada: {:?}\n", sol_candidata);

            // faz uma busca local nessa solução candidata
            let sol_candidata_melhorada:Solucao = intensifica_solucao(sol_candidata, espaco_busca, fo);
            print!("Solucao intensificada: {:?}\n", sol_candidata_melhorada);

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
fn varia_solucao(sol_otima:&Solucao, espaco_busca: &(f64, f64),  vizinhanca: &Vizinhanca) -> Solucao {
    let solucao_variada = vizinhanca.gera_vizinho_aleatorio(sol_otima, &espaco_busca);
    solucao_variada
}

/// Local Search
fn intensifica_solucao(sol_candidata: Solucao, espaco_busca: (f64, f64), fo: ObjetivoFn) -> Solucao {
    hillclimb::hillclimb(sol_candidata, espaco_busca, fo)
}

fn muda_vizinhanca( sol_atual: &mut Solucao, sol_candidata: &Solucao, vizinhanca: &mut Vizinhanca,) {
    // TODO: muda vizinhanca
    let resultado_x_atual = Solucao::evaluate(vizinhanca.funcao_objetivo, &sol_atual.variaveis);
    let resultado_x_canditato = Solucao::evaluate(vizinhanca.funcao_objetivo, &sol_candidata.variaveis);

    if resultado_x_canditato.resultado < resultado_x_atual.resultado {
        // Atualiza solucao otima
        vizinhanca.reset_estrutura_vizinhanca();
        *sol_atual = sol_candidata.clone();
    }
    else {
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
        Vizinhanca { funcao_objetivo, tot_vizinhancas, passo_vizinhanca, vizinhanca_atual}
    }
    // pub fn create_var_circle(&self) -> Solucao {
    //     let v = Vec::new();
    //     v.resize(2);
    //     print!();
    //     //print!("Raio da vizinhanca atual: {}\n", self.vizinhanca_atual);
    // }
    pub fn gera_vizinho_aleatorio(&self, solucao:&Solucao, espaco_busca: &(f64, f64)) -> Solucao {
        let mut variaveis : Vec<f64>= Vec::new();
        let lim_min = espaco_busca.0;
        let lim_max = espaco_busca.1;

        let size = solucao.variaveis.len();

        variaveis.resize(size, 0.0);
        // print!("-----------------\n");

        let mut na_caixa = false; // Logica muito complexa pra explicar no código sem nenhuma imagem. Mas é muito foda (fiquei legitimamente orgulhoso)
        for elemen in variaveis.iter_mut(){
            loop {
                let variacao = rand::thread_rng().gen_range(0.0, self.vizinhanca_atual);
                let neg_or_pos = rand::thread_rng().gen_range(-1.0,1.0);

                if neg_or_pos <= 0.0 {
                    *elemen = *elemen + variacao;
                    if lim_min <= *elemen && *elemen <= lim_max {
                        if -self.vizinhanca_atual + self.passo_vizinhanca <= *elemen || *elemen >= self.vizinhanca_atual - self.passo_vizinhanca {na_caixa = true};
                        // print!("elemen neg{:?}\n", *elemen);
                        break;
                    }
                } else {
                    *elemen = *elemen + self.vizinhanca_atual - variacao;
                    if lim_min <= *elemen && *elemen <= lim_max {
                        if -self.vizinhanca_atual + self.passo_vizinhanca <= *elemen || *elemen >= self.vizinhanca_atual - self.passo_vizinhanca {na_caixa = true};
                        // print!("elemen pos{:?}\n", *elemen);
                        break;
                    }
                }
            }
        }
        if !na_caixa { // Pulo do gato.
            let var = rand::thread_rng().gen_range(0,size);
            let variacao = rand::thread_rng().gen_range(self.vizinhanca_atual - self.passo_vizinhanca, self.vizinhanca_atual);
            let neg_or_pos = rand::thread_rng().gen_range(-1.0,1.0);

            if neg_or_pos <= 0.0 {
                variaveis[var] = -variacao;
            } else {
                variaveis[var] = variacao;
            }
        }
        //print!("Gerando novos valores: {} {}\n", variaveis[0], variaveis[1]);
        
        let new_resultado= Solucao::evaluate(self.funcao_objetivo,&variaveis);
        new_resultado 
    }
}

// Teoria dos calculos
// A ideia para geração das vizinhanças é criar circulos cilindricos concentricos em torno da
// coordenada atual do mapa. O calculo para gerar uma nova casca dado um raio r é:
// - x = r cos(teta) , y = r sen(teta)
//
// A ideia é:
// - Cada vizinhança é um circulo. Logo, o valor da vizinhança é o raio do circulo
// - Um valor da vizinhança é um ponto sobre o círculo
