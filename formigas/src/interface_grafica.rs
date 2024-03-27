use std::{cell::RefCell, rc::Rc};

use minifb::{Window, WindowOptions};

use crate::formiga::Formiga;

pub struct Interface {
    buffer: Vec<Vec<u32>>,
    window: Window,
    width: i32,
    height: i32,
}

impl Interface {
    pub fn new(width: i32, height: i32, escala: i32) -> Self {
        Self {
            buffer: vec![vec![0xFFFFFF; width as usize]; height as usize],
            window: Window::new(
                "Formigas!",
                (width * escala) as usize,
                (height * escala) as usize,
                WindowOptions::default(),
            )
            .unwrap(),
            width: width,
            height: height,
        }
    }

    pub fn atualizar_buffer(
        &mut self,
        ambiente: &Vec<Vec<(bool, Option<Rc<RefCell<Formiga>>>)>>,
        x: i32,
        y: i32,
    ) {
        for i in x - 1..=x + 1 {
            let mut indice_x = i;

            if i < 0 {
                indice_x = self.width + i;
            }

            if i >= self.width {
                indice_x = i - self.width;
            }
            for j in y - 1..=y + 1 {
                let mut indice_y = j;

                if j < 0 {
                    indice_y = self.width + j;
                }

                if j >= self.width {
                    indice_y = j - self.width;
                }

                let (corpo, formiga) = &ambiente[indice_x as usize][indice_y as usize];

                match formiga {
                    Some(valor) => {
                        if *corpo {
                            if valor.borrow().estado {
                                self.buffer[indice_x as usize][indice_y as usize] = 0xFF0000;
                            } else {
                                self.buffer[indice_x as usize][indice_y as usize] = 0x00FF00;
                            }
                        } else {
                            if valor.borrow().estado {
                                self.buffer[indice_x as usize][indice_y as usize] = 0xFFFF00;
                            } else {
                                self.buffer[indice_x as usize][indice_y as usize] = 0x8B4513;
                            }
                        }
                    }
                    None => {
                        if *corpo {
                            self.buffer[indice_x as usize][indice_y as usize] = 0x000000;
                        } else {
                            self.buffer[indice_x as usize][indice_y as usize] = 0xFFFFFF;
                        }
                    }
                }
            }
        }
    }

    pub fn display(&mut self) {
        let flat_buffer: Vec<u32> = self
            .buffer
            .iter()
            .flat_map(|row| row.iter())
            .cloned()
            .collect();
        self.window
            .update_with_buffer(&flat_buffer, self.width as usize, self.height as usize)
            .unwrap();
    }
}
