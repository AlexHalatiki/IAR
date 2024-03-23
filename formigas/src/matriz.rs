use std::cell::RefCell;
use std::rc::Rc;

pub struct Matriz<T: Clone> {
    pub width: i32,
    pub height: i32,
    pub data: Vec<Vec<(bool, Option<Rc<RefCell<T>>>)>>,
}

impl<T: Clone> Matriz<T> {
    pub fn new(width: i32, height: i32) -> Matriz<T> {
        Matriz {
            width: width,
            height: height,
            data: vec![vec![(false, None); width as usize]; height as usize],
        }
    }

    pub fn print_matriz(&self) {
        for linha in &self.data {
            for (corpo, formiga_opt) in linha {
                if *corpo {
                    print!("C");
                    match formiga_opt {
                        Some(_) => print!("F "),
                        None => print!(" "),
                    }
                } else {
                    match formiga_opt {
                        Some(_) => print!("F "),
                        None => print!(". "),
                    }
                }
            }
            println!(); // Nova linha após cada linha da matriz
        }
    }
}
