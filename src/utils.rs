pub fn unprocess_data(data: &[u8]) -> Vec<u8> {
    data.chunks_exact(2)
        .map(|chunk| chunk[0])
        .collect()
}