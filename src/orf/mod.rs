use io::dna_io::DnaIo;
use dna::Dna;
use dna::codon::Codon;

pub struct Orf{
    get_dna: DnaIo,
    index: Vec<[usize; 2]>,
}

impl Orf{
   pub fn new(dna_io: DnaIo) -> Orf{
       Orf{
        get_dna: dna_io,
        index: vec![[0usize; 2]; 0],
       }
   }
}

impl Orf{
    pub fn get_orf(mut self) -> Option<Vec<[usize; 2]>>{
        let mut dna_buf = Dna::new(0);
        let atg = Codon::new_atg();
        let taa = Codon::new_taa();
        let tag = Codon::new_tag();
        let tga = Codon::new_tga();
        let mut start_buf = vec![0usize; 0];
        let mut end_buf = vec![0usize; 0];

        for (index, dna) in self.get_dna.enumerate(){
            if index % 2 == 0{
                dna_buf.append(&dna);
            }else{
                dna_buf.append(&dna);
                match dna_buf.find_codon_triple(&atg, 0){
                    Some(index) => start_buf.push(index),
                    None => (),
                }
                match dna_buf.find_codon_triple(&taa, 0){
                    Some(index) => end_buf.push(index),
                    None => (),
                }
                match dna_buf.find_codon_triple(&tag, 0){
                    Some(index) => end_buf.push(index),
                    None => (),
                }
                match dna_buf.find_codon_triple(&tga, 0){
                    Some(index) => end_buf.push(index),
                    None => (),
                }
                dna_buf = dna;
            }
        }
        if start_buf.len() != end_buf.len(){
            None
        }else{
            for i in 0..start_buf.len(){
                self.index.push([start_buf[i], end_buf[i]]);
            }
            Some(self.index)
        }
    }
}