use rand;
use rand::Rng;

///储存氨基酸的结构体，具体方法参照Base结构体
pub struct Aa {
    aa: usize,
}

impl Aa {
    pub fn default() -> Aa {
        Aa { aa: 0usize }
    }

    pub fn rand() -> Aa {
        Aa { aa: get_rand() }
    }
}

impl Aa {
    pub fn from_num<'a>(aa: usize) -> Result<Aa, &'a str> {
        match aa {
            0...21 => Ok(Aa { aa }),
            _ => Err("u8 err"),
        }
    }
}

impl Aa {
    pub fn get_num(&self) -> usize {
        self.aa
    }
}

impl Clone for Aa {
    fn clone(&self) -> Self {
        Aa { aa: self.aa }
    }
}

pub fn get_rand() -> usize {
    rand::thread_rng().gen_range(0, 20)
}
