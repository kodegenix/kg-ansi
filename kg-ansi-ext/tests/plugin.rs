#![feature(plugin)]
#![plugin(kg_ansi_ext)]

#[test]
fn plugin() {
    cprintln!("%red'aaa {liczba}'", liczba = 12);
    cprint!("%red[aaa {}]\n", 12);
}
