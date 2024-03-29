mod formiga;
mod dado;
mod imagem;

use imagem::Imagem;
use dado::Dado;
use formiga::Formiga;
use rand::Rng;
use std::{
    cell::RefCell,
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

const WIDTH: i32 = 60;
const HEIGHT: i32 = 60;
const NUM_FORMIGAS: i32 = 10;
const RAIO_DE_VISAO: i32 = 1;
const ALFA: f64 = 0.97;
const K1: f64 = 0.35;
const K2: f64 = 0.65;
const ARQUIVO: &str = "600itens.txt";

fn main() {
    let mut ambiente: Vec<Vec<(Option<Dado>, Option<Rc<RefCell<Formiga>>>)>> =
        vec![vec![(None, None); WIDTH as usize]; HEIGHT as usize];
    let mut formigas: Vec<Rc<RefCell<Formiga>>> = Vec::with_capacity(NUM_FORMIGAS as usize);

    let mut rng = rand::thread_rng();

    let file = File::open(format!("../data/{}", ARQUIVO)).expect("Erro ao abrir o arquivo");
    let reader = BufReader::new(file);

    let mut indice_linha = 1;

    for linha in reader.lines() {
        let conteudo_linha = linha
            .expect(&format!("Erro ao ler linha {}", indice_linha))
            .replace(',', ".");

        let dados_aux: Vec<&str> = conteudo_linha.split("\t").collect();
        let mut props: Vec<f64> = Vec::with_capacity(dados_aux.len() - 1);
        let mut grupo: i32 = 0;

        for i in 0..dados_aux.len() {
            if i == dados_aux.len() - 1 {
                grupo = dados_aux[i].parse().expect(&format!(
                    "Erro ao converter {} para i32 (linha: {})",
                    dados_aux[i], indice_linha
                ));
                continue;
            }

            props.push(dados_aux[i].parse().expect(&format!(
                "Erro ao converter {} para f64 (linha: {})",
                dados_aux[i], indice_linha
            )));
        }

        let dado = Dado {
            grupo,
            props,
        };

        loop {
            let posicao_x = rng.gen_range(0..WIDTH);
            let posicao_y = rng.gen_range(0..HEIGHT);

            let dado_atual = &ambiente[posicao_x as usize][posicao_y as usize].0;

            if dado_atual.is_none() {
                ambiente[posicao_x as usize][posicao_y as usize].0 = Some(dado);
                break;
            }
        }

        indice_linha += 1;
    }

    for _ in 0..NUM_FORMIGAS {
        loop {
            let posicao_x = rng.gen_range(0..WIDTH);
            let posicao_y = rng.gen_range(0..HEIGHT);

            let formiga = &ambiente[posicao_x as usize][posicao_y as usize].1;

            if formiga.is_none() {
                let nova_formiga = Rc::new(RefCell::new(Formiga::new(
                    posicao_x,
                    posicao_y,
                )));
                formigas.push(nova_formiga.clone());
                ambiente[posicao_x as usize][posicao_y as usize].1 = Some(nova_formiga);
                break;
            }
        }
    }

    Imagem::salvar_matriz(&ambiente, "../data/inicio.png");

    for _ in 0..100000000 {
        for i in 0..NUM_FORMIGAS {
            let mut formiga = formigas[i as usize].borrow_mut();
            formiga.iteracao(&mut ambiente);
        }
    }

    for i in 0..NUM_FORMIGAS {
        let mut formiga = formigas[i as usize].borrow_mut();
        loop {
            if formiga.estado.is_none() {
                break;
            }
            formiga.iteracao(&mut ambiente);
        }
        ambiente[formiga.posicao_x as usize][formiga.posicao_y as usize].1 = None;
    }

    Imagem::salvar_matriz(&ambiente, "../data/fim.png");
}
