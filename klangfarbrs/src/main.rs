use klangfarbrs::test::test;

fn main() {
    match test() {
        Ok(()) => (),
        Err(error) => panic!("oopsie: {:?}", error),
    };
}
