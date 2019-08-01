#![feature(plugin)]
#![plugin(kg_ansi_ext)]

#[test]
fn plugin() {
    cprintln!("{}-{}", 12, "aaa");
}
