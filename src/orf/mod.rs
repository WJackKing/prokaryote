use io::dna_io::DnaIo;
use dna::Dna;

pub struct Orf{
    get_dna: DnaIo,
    index: Vec<[usize; 2]>,
}

impl Orf{
    pub fn new(dna_io: DnaIo) -> Orf{
        let dna_1: Dna;
        let dna_2: Dna;
        for i in 0..dna_io.line_num(){
            if i % 2 == 0{
                dna_2 = dna_io.read_line(1);
            }else{
                dna_1 = dna_io.read_line(1);
            }
        }
    }
}