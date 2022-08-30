pub fn sort<T>(input: Vec<T>) -> Vec<T> 
where T : Clone + Ord {
    let mut internal = input.clone();
    internal.sort();
    internal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_sort() {
        let input = vec![3];
        assert_eq!(sort(input), [3]);
    }

    #[test]
    fn test_nil_array(){
        let input: Vec<i32> = [].to_vec();
        assert_eq!(sort(input), [])
    }

    #[test]
    fn test_size_two (){
        let input: Vec<i32> = [3, 2].to_vec();
        assert_eq!(sort(input), [2, 3]);
    }

    #[test]
    fn test_big_list (){
        let input: Vec<i32> = [10, 3, 5, 20, 12].to_vec();
        assert_eq!(sort(input), [3, 5, 10, 12, 20]);
    }
}
