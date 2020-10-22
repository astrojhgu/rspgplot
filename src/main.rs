extern crate rspgplot;
use rspgplot::*;
fn main() {
    init_dev("?").unwrap();
    set_paper_size(16.0, 13.0);
    init_env((-1,4), (-1,4), types::Just::NonEqual, types::AxesStyle::WithLabels);
    scatter(&vec![1,2,3], &vec![1,2,3], 3);
    plot(&vec![1,2,3], &vec![1,2,3]);
    pgclos();
}
