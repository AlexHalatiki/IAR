mod formiga;
mod matriz;

use formiga::Formiga;
use matriz::Matriz;
use minifb::{Window, WindowOptions};
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

const WIDTH: i32 = 15;
const HEIGHT: i32 = 15;
const ESCALA: i32 = 30;
const NUM_FORMIGAS: i32 = 15;
const NUM_CORPOS: i32 = 60;
const RAIO_DE_VISAO: u16 = 1;

fn atualizar_buffer(
    buffer: &mut Vec<Vec<u32>>,
    data: &Vec<Vec<(bool, Option<Rc<RefCell<Formiga>>>)>>,
    x: i32,
    y: i32,
) {
    for i in x - 1..=x + 1 {
        let mut indice_x = i;

        if i < 0 {
            indice_x = WIDTH + i;
        }

        if i >= WIDTH {
            indice_x = i - WIDTH;
        }
        for j in y - 1..=y + 1 {
            let mut indice_y = j;

            if j < 0 {
                indice_y = HEIGHT + j;
            }

            if j >= HEIGHT {
                indice_y = j - HEIGHT;
            }

            let (corpo, formiga) = &data[indice_x as usize][indice_y as usize];

            if *corpo {
                match formiga {
                    Some(valor) => {
                        if valor.borrow().estado {
                            buffer[indice_x as usize][indice_y as usize] = 0xFF0000;
                        } else {
                            buffer[indice_x as usize][indice_y as usize] = 0x00FF00;
                        }
                    }
                    None => {
                        buffer[indice_x as usize][indice_y as usize] = 0x000000;
                    }
                }
            } else {
                match formiga {
                    Some(valor) => {
                        if valor.borrow().estado {
                            buffer[indice_x as usize][indice_y as usize] = 0xFFFF00;
                        } else {
                            buffer[indice_x as usize][indice_y as usize] = 0x8B4513;
                        }
                    }
                    None => {
                        buffer[indice_x as usize][indice_y as usize] = 0xFFFFFF;
                    }
                }
            }
        }
    }
}

pub fn display(window: &mut Window, buffer: &Vec<Vec<u32>>) {
    let flat_buffer: Vec<u32> = buffer.iter().flat_map(|row| row.iter()).cloned().collect();
    window
        .update_with_buffer(&flat_buffer, WIDTH as usize, HEIGHT as usize)
        .unwrap();
}

fn main() {
    let matriz: Rc<RefCell<Matriz<Formiga>>> = Rc::new(RefCell::new(Matriz::new(WIDTH, HEIGHT)));
    let mut formigas: Vec<Rc<RefCell<Formiga>>> = Vec::with_capacity(NUM_FORMIGAS as usize);

    let mut buffer: Vec<Vec<u32>> = vec![vec![0xFFFFFF; WIDTH as usize]; HEIGHT as usize];

    let mut window = Window::new(
        "Formigas!",
        (WIDTH * ESCALA) as usize,
        (HEIGHT * ESCALA) as usize,
        WindowOptions::default(),
    )
    .unwrap();

    let mut rng = rand::thread_rng();

    let mut ambiente = matriz.borrow_mut();

    // Criar formigas
    for _ in 0..NUM_FORMIGAS {
        loop {
            let posicao_x = rng.gen_range(0..WIDTH);
            let posicao_y = rng.gen_range(0..HEIGHT);

            let (_, formiga) = &ambiente.data[posicao_x as usize][posicao_y as usize];

            if formiga.is_none() {
                let nova_formiga = Rc::new(RefCell::new(Formiga::new(
                    Rc::clone(&matriz),
                    RAIO_DE_VISAO,
                    posicao_x,
                    posicao_y,
                )));
                formigas.push(Rc::clone(&nova_formiga));
                ambiente.data[posicao_x as usize][posicao_y as usize].1 =
                    Some(Rc::clone(&nova_formiga));
                atualizar_buffer(&mut buffer, &ambiente.data, posicao_x, posicao_y);
                break;
            }
        }
    }

    // Criar corpos
    for _ in 0..NUM_CORPOS {
        loop {
            let posicao_x = rng.gen_range(0..WIDTH);
            let posicao_y = rng.gen_range(0..HEIGHT);

            let (corpo, _) = ambiente.data[posicao_x as usize][posicao_y as usize];

            if !corpo {
                ambiente.data[posicao_x as usize][posicao_y as usize].0 = true;
                atualizar_buffer(&mut buffer, &ambiente.data, posicao_x, posicao_y);
                break;
            }
        }
    }

    drop(ambiente);

    let mut rodar = true;

    while window.is_open() {
        thread::sleep(Duration::from_secs(1));
        display(&mut window, &buffer);
        if rodar {
            thread::sleep(Duration::from_secs(5));
            for _ in 0..10000 {
                for i in 0..NUM_FORMIGAS {
                    let mut formiga = formigas[i as usize].borrow_mut();
                    let x = formiga.posicao_x;
                    let y = formiga.posicao_y;
                    formiga.iteracao();
                    drop(formiga);
                    atualizar_buffer(&mut buffer, &matriz.borrow().data, x, y);
                }
                display(&mut window, &buffer);
            }
        }
        rodar = false;
        display(&mut window, &buffer);
    }
}
