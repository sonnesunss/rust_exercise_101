use std::env;

fn main() {
    let env1 = env::args();

    for env in env1 {
        println!("env: {:?}", env);
    }
}
