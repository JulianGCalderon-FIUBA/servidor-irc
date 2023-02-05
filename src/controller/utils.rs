pub fn is_not_empty(vector: &Vec<String>) -> bool {
    !vector.is_empty()
}

pub fn remove_element(vector: &mut Vec<String>, element: &String) {
    if vector.contains(element) {
        vector.remove(vector.iter().position(|x| x == element).unwrap());
    }
}

pub fn push_if_absent(
    original_vector: &Vec<String>,
    new_vector: &mut Vec<String>,
    element: String,
) {
    if !original_vector.contains(&element) {
        new_vector.push(element.clone());
    }
}
