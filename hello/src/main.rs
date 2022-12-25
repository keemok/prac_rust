fn main() {
    greet_world();
}

fn greet_world(){
    let america = "Hello, world!";
    let korea ="안녕하세요.";
    let regions = [america, korea];

    for region in regions.iter(){
        println!("{}",&region);
    }
}
