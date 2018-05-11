use std::fs::File;
use std::io::Error;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Lines;
use dna::Dna;

///从文件读入一段dna序列
pub struct DnaIo {
    lines: Lines<BufReader<File>>,
    info: String,
    dna: Dna,
    index: usize,
    line_num: (usize, usize),
}

impl DnaIo {
    ///打开一个fasta文件，并初始话数据
    pub fn open(path: &str) -> Result<DnaIo, Error> {
        match File::open(path) {
            Ok(file) => {
                let mut dna_io = DnaIo {
                    lines: BufReader::new(file).lines(),
                    info: String::new(),
                    dna: Dna::new(0usize),
                    index: 0usize,
                    line_num: (1, 1),
                };
                dna_io.info = dna_io.lines.next().unwrap().unwrap();
                dna_io.index = 1;
                Ok(dna_io)
            }
            Err(err) => Err(err),
        }
    }

    // pub fn to_read_buf(self) -> ReadBuf{
    //     ReadBuf::new(self)
    // }
}

impl DnaIo {
    pub fn get_info(&self) -> String {
        self.info.clone()
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_line_num(&self) -> (usize, usize) {
        self.line_num
    }
}

impl DnaIo {
    pub fn read_line(&mut self, line_num: usize) -> Dna {
        let mut string;
        let mut len = 0;
        self.dna.clean();
        for _i in 0..line_num {
            string = self.lines.next().unwrap().unwrap();
            len = len + string.len();
            self.dna.append(&read_to_dna(string.clone().into_bytes()));
            string.clear();
        }
        self.index = self.index + len;
        self.line_num = (self.line_num.1, self.line_num.1 + line_num);
        self.dna.clone()
    }
}

impl Iterator for DnaIo {
    type Item = Dna;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lines.next() {
            Some(read) => Some(read_to_dna(read.unwrap().into_bytes())),
            None => None,
        }
    }
}

pub fn read_to_dna(read: Vec<u8>) -> Dna {
    let mut array = vec![0usize; 0];
    for r in read {
        match r {
            65 => array.push(0),
            67 => array.push(1),
            71 => array.push(2),
            84 => array.push(3),
            _ => (),
        }
    }
    Dna::from_array(array)
}

//------------------------------------------------------------

pub struct ReadBuf{
    file: File,
    buf: [u8; 1024],
    start: usize,
    end: usize,
    index: usize,
}

impl ReadBuf{
    pub fn new(file: File) -> ReadBuf{
        ReadBuf{
            file,
            buf: [0u8; 1024],
            start: 0usize,
            end: 0usize,
            index: 0usize,
        }
    }
}

pub struct Mesg{
    mesg: String,
    start: usize,
    end: usize,
}

impl Mesg{
    pub fn new(s: String, start: usize, end: usize) -> Mesg{
        Mesg{
            mesg: s,
            start,
            end,
        }
    }
}

impl Mesg{
    pub fn get_mesg(file: &File) -> Mesg{
        let mut buf = [0u8; 1024];
        let mut mesg = Mesg::new(String::new(), 0, 0);
        for i in 0..buf.len(){

        }
        if mesg.end != 0{
            mesg
        }else{
            mesg
        }
    }

    pub fn get_string(&self) -> String{
        self.mesg
    }
}

impl Clone for Mesg{
    fn clone(&self) -> Self{
        Mesg{
            mesg: self.mesg,
            start: self.start,
            end: self.end,
        }
    }
}

pub struct Dnaio{
    buf: ReadBuf,
    mesgv: Vec<Mesg>,
}

impl Dnaio{
    pub fn open(file: &str) -> Dnaio{
        let mut f = File::open(file).unwrap();
        let mesgv: Vec<Mesg>;
        Dnaio{
            buf: ReadBuf::new(f),
            mesgv,
        }
    }
}

impl Dnaio{
    pub fn get_info(&self) -> Vec<String>{
        let mut v = vec![String::new(); 0];
        for i in self.mesgv{
            v.push(i.get_string());
        }
        v
    }
}
