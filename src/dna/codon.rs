use Dna;
use io::BASE_TO_AA;
use protein::aa::Aa;

pub struct Codon {
    h: usize,
    m: usize,
    l: usize,
}

impl Codon {
    pub fn from_num<'a>(h: usize, m: usize, l: usize) -> Result<Codon, &'a str> {
        if h <= 3 && m <= 3 && l <= 3 {
            Ok(Codon { h, m, l })
        } else {
            Err("num err")
        }
    }

    pub fn new_atg() -> Codon {
        Codon { h: 0, m: 3, l: 2 }
    }

    pub fn new_taa() -> Codon {
        Codon { h: 3, m: 0, l: 0 }
    }

    pub fn new_tag() -> Codon {
        Codon { h: 3, m: 0, l: 2 }
    }

    pub fn new_tga() -> Codon {
        Codon { h: 3, m: 2, l: 0 }
    }
}

impl Codon {
    pub fn get_h(&self) -> usize {
        self.h
    }

    pub fn get_m(&self) -> usize {
        self.m
    }

    pub fn get_l(&self) -> usize {
        self.l
    }

    pub fn get_array(&self) -> Vec<usize> {
        vec![self.h, self.m, self.l]
    }
}

impl Codon {
    pub fn cmp(&self, other: &Codon) -> bool {
        let mut is_cmp = false;
        if self.get_h() == other.get_h() && self.get_m() == other.get_m()
            && self.get_l() == other.get_l()
        {
            is_cmp = true;
        }
        is_cmp
    }
}

impl Codon {
    pub fn to_aa(&self) -> Aa {
        Aa::from_num(BASE_TO_AA[self.h * 16 + self.m * 4 + self.l]).unwrap()
    }
}

impl Clone for Codon {
    fn clone(&self) -> Self {
        Codon {
            h: self.h,
            m: self.m,
            l: self.l,
        }
    }
}

pub struct GetCodon<'a> {
    dna: &'a Dna,
    codon: Codon,
    index: usize,
}

impl<'a> GetCodon<'a> {
    pub fn from_dna(dna: &Dna, offset: usize) -> GetCodon {
        GetCodon {
            dna,
            codon: Codon {
                h: dna.index_num(offset),
                m: dna.index_num(offset + 1),
                l: dna.index_num(offset + 2),
            },
            index: offset,
        }
    }
}

impl<'a> GetCodon<'a> {
    pub fn get_codon(&self) -> Codon {
        self.codon.clone()
    }
}

impl<'a> Iterator for GetCodon<'a> {
    type Item = Codon;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.index + 2) >= self.dna.len() {
            None
        } else {
            let codon = Codon {
                h: self.dna.index_num(self.index),
                m: self.dna.index_num(self.index + 1),
                l: self.dna.index_num(self.index + 2),
            };
            self.index = self.index + 3;
            Some(codon)
        }
    }
}
