extern crate mnist_handwritten_digits as mnist;

fn main() {
    let labels = mnist::read_label_file("datasets/train-labels-idx1-ubyte");
    println!("{:?}", labels);
}
