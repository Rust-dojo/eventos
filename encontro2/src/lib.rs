pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// Funcao que retorna o ultimo elemento de uma lista
// https://ocaml.org/problems#1
pub fn last_element(l: &[i32]) -> Option<i32> {
    let length = l.len();
    if length > 0 {
        Some(l[length - 1])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_last_element() {
        let input = [1, 2, 3, 4];
        assert_eq!(last_element(&input), Some(4));
    }

    #[test]
    fn test_last_element_if_empty() {
        let input = [];
        assert_eq!(last_element(&input), None);
    }

}
