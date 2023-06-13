use sparse_by_default_cratesio::get_dummy_json;

fn main() {
    println!("hello, world");
    let result = get_dummy_json();
    println!("{:?}", result);
}
