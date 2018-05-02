pub fn protein_read_to_array(read: Vec<u8>) -> Vec<usize> {
    let mut array = vec![0usize; 0];
    for r in read {
        match r {
            65 => array.push(0),
            67 => array.push(1),
            68 => array.push(2),
            69 => array.push(3),
            70 => array.push(4),
            71 => array.push(5),
            72 => array.push(6),
            73 => array.push(7),
            75 => array.push(8),
            76 => array.push(9),
            77 => array.push(10),
            78 => array.push(11),
            80 => array.push(12),
            81 => array.push(13),
            82 => array.push(14),
            83 => array.push(15),
            84 => array.push(16),
            86 => array.push(17),
            87 => array.push(18),
            89 => array.push(19),
            _ => (),
        }
    }
    array
}
