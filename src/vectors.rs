fn main() {
    let v1: Vec<f32> = vec![0.0, 1.0, 2.0];
    let v2: Vec<f32> = vec![3.0, 4.0, 5.0];

    let ans1 = sum_two_vecs(v1, v2);

    println!("Two vectos sum:");
    show_vec_elements(ans1);
    println!("");

    let k = 2.0;
    let v3: Vec<f32> = vec![3.0, 1.0, 4.0];

    let ans2 = scalar_multiplication(k, v3);

    println!("Scalar multiplication:");
    show_vec_elements(ans2);
    println!("");
}

fn sum_two_vecs(v: Vec<f32>, w: Vec<f32>) -> Vec<f32> {
    let mut vec: Vec<f32> = Vec::new();
    let n = v.len();
    for i in 0..n {
        vec.push(v[i] + w[i]);
    }
    return vec;
}

fn scalar_multiplication(k: f32, v: Vec<f32>) -> Vec<f32> {
    let mut vec: Vec<f32> = Vec::new();
    let n = v.len();
    for i in 0..n {
        vec.push(k * v[i]);
    }
    return vec;
}

fn show_vec_elements(v: Vec<f32>) -> () {
    let n = v.len();
    print!("[");
    for i in 0..n {
        if i < n - 1 {
            print!("{} ", v[i]);
        } else {
            print!("{}", v[i]);
        }
    }
    println!("]");
}
