pub fn bytes2float(data: [u8; 8usize]) -> f32 {
    let mut a = [0 as u8; 4];
    a.copy_from_slice(&data[0..4]);
    f32::from_be_bytes(a)
}
