pub mod aa;

use self::aa::Aa;

///储存蛋白质的结构体，具体方法参照Dna结构体
pub struct Protein {
    pub peptide: Vec<Aa>,
}

impl Protein {
    pub fn new(len: usize) -> Protein {
        Protein {
            peptide: vec![Aa::default(); len],
        }
    }

    pub fn new_rand(len: usize) -> Protein {
        let mut protein = Protein {
            peptide: vec![Aa::rand(); 0],
        };
        for _i in 0..len {
            protein.peptide.push(Aa::rand());
        }
        protein
    }
}

impl Protein {
    pub fn from_array(peptide: Vec<usize>) -> Protein {
        let mut p = Protein::new(0);
        for a in peptide {
            match a {
                0...21 => p.push_num(a).unwrap(),
                _ => (),
            }
        }
        p
    }
}

impl Protein {
    pub fn push_num<'a>(&mut self, aa: usize) -> Result<(), &'a str> {
        match Aa::from_num(aa) {
            Ok(a) => Ok(self.peptide.push(a)),
            Err(err) => Err(err),
        }
    }

    pub fn append(&mut self, other: Protein) {
        for i in 0..other.len() {
            self.push_num(other.index_num(i)).unwrap();
        }
    }
}

impl Protein {
    pub fn set_num<'a>(&mut self, index: usize, aa: usize) -> Result<(), &'a str> {
        match Aa::from_num(aa) {
            Ok(a) => Ok(self.peptide[index] = a),
            Err(err) => Err(err),
        }
    }
}

impl Protein {
    pub fn index(&self, index: usize) -> Aa {
        self.peptide[index].clone()
    }

    pub fn index_num(&self, index: usize) -> usize {
        self.peptide[index].get_num()
    }

    pub fn len(&self) -> usize {
        self.peptide.len()
    }
}

impl Protein {
    pub fn clean(&mut self) {
        self.peptide = vec![Aa::default(); 0];
    }
}
