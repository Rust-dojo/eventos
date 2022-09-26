pub fn last_two_elements(vector: Vec<i32>) -> Vec<i32> {
    if vector.len() == 0 {
        return vec![];
    }
    let mut vector_clone = vector.clone();
    let mut result = vec![];
    // if let Some(last) = vector_clone.pop() {
    //     result.push(last);
    // }

    // if let Some(penultimate) = vector_clone.pop() {
    //     result.push(penultimate);
    // }
    let last = vector_clone.pop();
    let penultimate = vector_clone.pop();
    match penultimate{
        Some(penultimate) => result.push(penultimate);
        _
    }
    if Some(last) = last{
        result.push(last);
    }

    /*
    enum Option<T> {
        None,
        Some(T),
    }
    */ 

    result
    // result.into_iter().rev().collect::<Vec<i32>>()
}

/* Playground
https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=35dbe83c6b98f7ecba70b7eac7fd8a98

fn main() {
    let list = vec![1, 2, 3, 4, 5];
    
    let result: Vec<i32> = list.into_iter().rev().take(2).rev().collect();
    
    println!("{:?}", result);
}
*/
#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_two_elements_vector() {
        let input = vec![3, 4];
        assert_eq!(last_two_elements(input), [3, 4]);
    }

    #[test]
    fn test_empty_vector() {
        let input = vec![];
        assert_eq!(last_two_elements(input), []);
    }

    #[test]
    fn test_one_element_vector() {
        let input = vec![1];
        assert_eq!(last_two_elements(input), [1]);
    }

    #[test]
    fn test_four_elements_vector() {
        let input = vec![1, 2, 3, 4];
        assert_eq!(last_two_elements(input), [3, 4]);
    }

    // https://ocaml.org/problems
    // Last two elements of a list
    // Find the last but one (last and penultimate) elements of a list.
    
}
