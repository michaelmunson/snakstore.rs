mod argv;
mod command;
fn main() {
    let arg_map = argv::get_map(vec![
        "get"
    ]);
    for (k, v) in arg_map {
        println!("{k} : {v}")
    }
}
