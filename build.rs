#[cfg(feature = "lalrpop")]
extern crate lalrpop;

fn main() {
    #[cfg(feature = "lalrpop")]
    lalrpop::process_root().unwrap();
}
