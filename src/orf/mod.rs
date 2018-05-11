use io::dna_io::Dnaio;
use dna::Dna;
use dna::codon::Codon;

pub struct Orf {
    buf: [u8; 1024],
    start: usize,
    end: usize,
    index: usize,
}
