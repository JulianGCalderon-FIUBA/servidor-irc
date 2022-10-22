pub fn index_of(element: String, array: &[String]) -> usize {
    array.iter().position(|client| *client == element).unwrap()
}
