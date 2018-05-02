use dna::base::Base;
use dna::Dna;
use protein::aa::Aa;
use protein::Protein;

pub mod dna_io;
pub mod protein_io;

static BASE: [char; 4] = ['A', 'C', 'G', 'T'];

static AA: [char; 21] = [
    'A', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W',
    'Y', '*',
];

static TRIPLE_AA: [&str; 21] = [
    "Ala", "Cys", "Asp", "Glu", "Phe", "Gly", "His", "Ile", "Lys", "Leu", "Met", "Asn", "Pro",
    "Gln", "Arg", "Ser", "Thr", "Val", "Trp", "Tyr", " * ",
];

static BASE_TO_AA: [usize; 64] = [
    8, 11, 8, 11, 16, 16, 16, 16, 14, 15, 14, 15, 7, 7, 10, 7, 13, 6, 13, 6, 12, 12, 12, 12, 14,
    14, 14, 14, 9, 9, 9, 9, 3, 2, 3, 2, 0, 0, 0, 0, 5, 5, 5, 5, 17, 17, 17, 17, 20, 19, 20, 19, 15,
    15, 15, 15, 20, 1, 18, 1, 9, 4, 9, 4,
];

///输出碱基
pub fn print_base(base: &Base) {
    print!("{}", BASE[base.get_num()]);
}

///输出氨基酸
pub fn print_aa(aa: &Aa) {
    print!("{}", AA[aa.get_num()]);
}


///输出dna
pub fn print_dna(dna: &Dna) {
    for i in 0..dna.len() {
        print!("{}", BASE[dna.index_num(i)]);
    }
}

///以单字符输出蛋白质序列
pub fn print_protein(protein: &Protein) {
    for i in 0..protein.len() {
        print!("{}", AA[protein.index_num(i)]);
    }
}

///以三字符输出蛋白质序列
pub fn print_protein_triple(protein: &Protein) {
    for i in 0..protein.len() {
        print!("{}", TRIPLE_AA[protein.index_num(i)]);
    }
}

///dna序列转换为蛋白质序列
pub fn dna_to_protein(dna: &Dna, offset: usize) -> Protein {
    let mut array = vec![0usize; 0];

    for i in offset..dna.len() - 1 {
        if i % 3 == 0 {
            let h = dna.index_num(i);
            let m = dna.index_num(i + 1);
            let l = dna.index_num(i + 2);
            array.push(BASE_TO_AA[h * 16 + m * 4 + l]);
        }
    }

    Protein::from_array(array)
}
