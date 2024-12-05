use std::fs;

fn main() {
    let path = "lista.txt";

    let list = fs::read_to_string(path).expect("File not found or could not be opened");
    let (list_a, list_b) = list_to_vectors(list);

    let acc = get_differences(list_a, list_b);

    println!("{acc}");
}

fn get_differences(mut list_a: Vec<isize>, mut list_b: Vec<isize>) -> isize{
    list_a.sort();
    list_b.sort();

    let mut acc = 0;

    for i in 0..list_a.len(){
        acc += (list_a[i] - list_b[i]).abs();
    }

    return acc;
}

fn list_to_vectors(list: String) -> (Vec<isize>, Vec<isize>){
    let mut list_a: Vec<isize> = Vec::new();
    let mut list_b: Vec<isize> = Vec::new();
    for line in list.lines(){
        let collection: Vec<&str> = line.split("   ").collect();
        list_a.push(collection[0].parse().expect("No es un número"));
        list_b.push(collection[1].parse().expect("No es un número"));
    }

    return (list_a, list_b);
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn list_to_vectors_1(){
        let list = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3".to_string();
        let (list1, list2) = list_to_vectors(list);
        assert_eq!(list1, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(list2, vec![4, 3, 5, 3, 9, 3]);
    }
    #[test]
    fn list_to_vectors_2(){
        let list = "33   44\n44   33\n23   53\n13   33\n33   93\n33   33".to_string();
        let (list1, list2) = list_to_vectors(list);
        assert_eq!(list1, vec![33, 44, 23, 13, 33, 33]);
        assert_eq!(list2, vec![44, 33, 53, 33, 93, 33]);
    }
    #[test]
    fn get_differences_1(){
        let list_a = vec![3, 4, 2, 1, 3, 3];
        let list_b = vec![4, 3, 5, 3, 9, 3];

        assert_eq!(get_differences(list_a, list_b), 11);
    }
}
