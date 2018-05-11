use io::dna_io::Dnaio;

pub struct Orf<'a> {
    dna_io: &'a mut Dnaio,
    buf: [u8; 4096],
    start: usize,
    end: usize,
    index: usize,
}

impl<'a> Orf<'a> {
    pub fn new(dna_io: &mut Dnaio, record_index: usize) -> Result<Orf, &'static str> {
        let mut buf = [0u8; 4096];
        let n = dna_io.read_record(record_index, &mut buf);
        if record_index >= dna_io.records() {
            Ok(Orf {
                dna_io,
                buf,
                start: 0usize,
                end: n,
                index: 0usize,
            })
        } else {
            Err("record index out of bound")
        }
    }
}

impl<'a> Orf<'a> {
    pub fn forward(&mut self, b: u8) {
        self.buf[self.end] = b;
        if self.end == self.buf.len() - 1 {
            self.end = 0;
        } else {
            self.end = self.end + 1;
        }
        if self.start == self.buf.len() - 1 {
            self.start = 0;
        } else {
            self.start = self.start + 1;
        }
        self.index = self.index + 1;
    }

    pub fn back(&mut self, b: u8) {
        self.buf[self.end] = b;
        if self.end == 0 {
            self.end = self.buf.len() - 1;
        } else {
            self.end = self.end - 1;
        }
        if self.start == 0 {
            self.start = self.buf.len() - 1;
        } else {
            self.start = self.start - 1;
        }
        self.index = self.index - 1;
    }

    pub fn move_forward(&mut self, forward_buf: &[u8]) {
        for i in forward_buf {
            self.forward(*i);
        }
    }

    pub fn move_back(&mut self, back_buf: &[u8]){
        for i in back_buf{
            self.back(*i);
        }
    }
}
