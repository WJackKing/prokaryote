#![cfg(test)]

#[test]
fn base_work() {
    // use dna::base::Base;
    // let base_default = Base::default();
    // let base_rand = Base::rand();

    // let base_num = Base::from_num(1).unwrap();
    // let base_char = Base::from_char('A').unwrap();

    // println!("{}", base_default.get_num());
    // println!("{}", base_rand.get_char());

    // println!("{}", base_num.get_num());
    // println!("{}", base_char.get_char());
}

#[test]
#[should_panic]
fn base_panic() {
    // use dna::base::Base;
    // match Base::from_num(4){
    // 	Ok(b) => println!("{}", b.get_char()),
    // 	Err(err) => panic!("{}", err)
    // }
}

#[test]
fn dna_work() {
    // use dna::DNA;
    // let dna_default = DNA::new(100);
    // let dna_rand = DNA::new_rand(100);
    // dna_default.println_array();
    // dna_rand.println_array();
}
