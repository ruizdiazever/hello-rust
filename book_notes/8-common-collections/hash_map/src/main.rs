mod normal;
mod error;
mod get;
mod iteration;
mod rewrite;
mod entry;

fn main() {
    use normal::scores;
    use error::test;
    use get::get_score;
    use iteration::for_normal;
    use rewrite::banana;
    use entry::manzana;
    use entry::pera;

    scores();
    test();
    get_score();
    for_normal();
    banana();
    manzana();
    pera();
}
