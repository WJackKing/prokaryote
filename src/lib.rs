///！一个模拟原核细胞的crate
extern crate rand;

pub mod dna;
pub mod protein;
pub mod io;
pub mod orf;
pub mod test;

pub use dna::Dna;
pub use dna::base::Base;
pub use dna::codon::Codon;
pub use dna::codon::GetCodon;
pub use protein::Protein;
pub use protein::aa::Aa;
pub use io::dna_io::DnaIo;
pub use io::protein_io;
pub use orf::Orf;

///功能测试函数
pub fn run() {
    // let mut di = DnaIo::open("GCF_000005845.2_ASM584v2_genomic.fna").unwrap();
    // println!("{}", di.get_info());

    // let d = di.read_line(1);
    // io::print_dna(&d);
    // println!();
    // println!("{}", di.get_index());
    // let (start, end) = di.get_line_num();
    // println!("{}-{}", start, end);

    // let e = di.read_line(1);
    // io::print_dna(&e);
    // println!();
    // let (start, end) = di.get_line_num();
    // println!("{}-{}", start, end);

    // let dna = Dna::new_rand(99);
    // io::print_dna(&dna);
    // println!();
    // let codon = dna.codon(0).unwrap();
    // for c in codon {
    //     // let d = Dna::from_array(c.get_array());
    //     // io::print_dna(&d);
    //     // print!("  ");
    //     // let a = c.to_aa();
    //     // io::print_aa(&a);
    //     // println!();

    //     // print!("{} ", dna.find_codon(c).unwrap());
    //     let v = dna.find_codon_all(c).unwrap();
    //     for i in v{
    //         print!("{} ", i);
    //     }
    //     println!();
    // }

    // let c = Codon::from_num(0, 3, 2).unwrap();
    // let v = dna.find_codon_all(&c, 0).unwrap();
    // for i in v {
    //     print!("{} ", i);
    // }
    // println!();
    // println!("{}", dna.find_codon(&c, 1).unwrap());
    // println!("{}", dna.find_codon_triple(&c, 1).unwrap());

    let di = DnaIo::open("GCF_000005845.2_ASM584v2_genomic.fna").unwrap();
    println!("{}", di.get_info());

    let orf = Orf::new(di);
    match orf.get_orf(){
        Some(v) => {
            for i in v{
                println!("{}:{}", i[0], i[1]);
            }
        }
        None => println!("none"),
    }
}
