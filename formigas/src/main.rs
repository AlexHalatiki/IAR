mod formiga;
mod interface_grafica;

use formiga::Formiga;
use interface_grafica::Interface;
use rand::Rng;
use std::{cell::RefCell, rc::Rc, thread, time::Duration};

const WIDTH: i32 = 40;
const HEIGHT: i32 = 40;
const ESCALA: i32 = 20;
const NUM_FORMIGAS: i32 = 40;
const NUM_CORPOS: i32 = 100;
const RAIO_DE_VISAO: i32 = 1;

fn main() {
    let mut ambiente: Vec<Vec<(bool, Option<Rc<RefCell<Formiga>>>)>> =
        vec![vec![(false, None); WIDTH as usize]; HEIGHT as usize];
    let mut formigas: Vec<Rc<RefCell<Formiga>>> = Vec::with_capacity(NUM_FORMIGAS as usize);
    let mut interface = Interface::new(WIDTH, HEIGHT, ESCALA);

    let mut rng = rand::thread_rng();

    for _ in 0..NUM_FORMIGAS {
        loop {
            let posicao_x = rng.gen_range(0..WIDTH);
            let posicao_y = rng.gen_range(0..HEIGHT);

            let formiga = &ambiente[posicao_x as usize][posicao_y as usize].1;

            if formiga.is_none() {
                let nova_formiga = Rc::new(RefCell::new(Formiga::new(
                    RAIO_DE_VISAO,
                    posicao_x,
                    posicao_y,
                )));
                formigas.push(nova_formiga.clone());
                ambiente[posicao_x as usize][posicao_y as usize].1 = Some(nova_formiga);
                interface.atualizar_buffer(&ambiente, posicao_x, posicao_y);
                break;
            }
        }
    }

    for _ in 0..NUM_CORPOS {
        loop {
            let posicao_x = rng.gen_range(0..WIDTH);
            let posicao_y = rng.gen_range(0..HEIGHT);

            let (corpo, _) = ambiente[posicao_x as usize][posicao_y as usize];

            if !corpo {
                ambiente[posicao_x as usize][posicao_y as usize].0 = true;
                interface.atualizar_buffer(&ambiente, posicao_x, posicao_y);
                break;
            }
        }
    }

    thread::sleep(Duration::from_secs(1));
    interface.display();
    thread::sleep(Duration::from_secs(5));
    for _ in 0..20000 {
        for i in 0..NUM_FORMIGAS {
            let mut formiga = formigas[i as usize].borrow_mut();
            formiga.iteracao(&mut ambiente, WIDTH, HEIGHT);
            let x = formiga.posicao_x;
            let y = formiga.posicao_y;
            drop(formiga);
            interface.atualizar_buffer(&ambiente, x, y);
        }
        interface.display();
    }
    for i in 0..NUM_FORMIGAS {
        loop {
            let mut formiga = formigas[i as usize].borrow_mut();
            if !formiga.estado {
                break;
            }
            formiga.iteracao(&mut ambiente, WIDTH, HEIGHT);
            let x = formiga.posicao_x;
            let y = formiga.posicao_y;
            drop(formiga);
            interface.atualizar_buffer(&ambiente, x, y);
        }
    }
    loop{    interface.display();}

    
}
