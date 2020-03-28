fn main() {
    let vec0 = Vec::new();
    let mut x = vec0;

    let mut vec1 = fill_vec(x);

    // Do not change the following line!
    
    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
