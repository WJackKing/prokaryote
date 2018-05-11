use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use dna::Dna;

pub struct Dnaio {
    file: File,
    lines: Vec<usize>,
    records: Vec<usize>,
}

impl Dnaio {
    pub fn new(mut file: File) -> Dnaio {
        let mut index = 0;
        let mut lines = vec![0usize; 1];
        let mut records = vec![0usize; 0];
        let mut buf = vec![0u8; 200];
        file.seek(SeekFrom::Start(0)).unwrap();
        loop {
            match file.read(&mut buf) {
                Ok(a) => {
                    if a == 0 {
                        break;
                    } else {
                        //判断行尾和注释符
                        for i in 0..buf.len() {
                            if buf[i].eq(&"\n".as_bytes()[0]) {
                                lines.push(index + i + 1);
                            }
                            if buf[i].eq(&">".as_bytes()[0]) {
                                records.push(lines.len() - 1);
                            }
                        }
                        index = index + a;
                    }
                }
                Err(err) => panic!("{}", err),
            }
        }
        Dnaio { file, lines, records }
    }
}

impl Dnaio{
    pub fn lines(&self) -> usize{
        self.lines.len()
    }

    pub fn records(&self) -> usize{
        self.records.len()
    }

    pub fn get_info(&mut self, record_index: usize) -> Vec<u8>{
        self.read_line(record_index, record_index + 1)
    }
}

impl Dnaio {
    pub fn read_line(&mut self, start_line_num: usize, end_line_num: usize) -> Vec<u8> {
        let start_line_index = self.lines[start_line_num];
        let end_line_index = self.lines[end_line_num];
        let mut buf = vec![0u8; end_line_index - start_line_index];
        self.file
            .seek(SeekFrom::Start(start_line_index as u64))
            .unwrap();
        self.file.read(&mut buf).unwrap();
        buf
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
