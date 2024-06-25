pub mod warsh;

fn main() {
    let result = warsh::get_ayas();
    match result {
        Ok(_) => (),
        Err(error) => println!("{}", error),
    }
}
