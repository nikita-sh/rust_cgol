mod grid;

fn main() {
    let mut test : grid::Grid = grid::Grid {
        width: 10,
        height: 10,
        values: vec![vec![false; 10]; 10]
    };

    println!("grid {:?}", test.values);

    test.change_nth(10, true);
    test.change_coord(3, 3, true);

    println!("\ngrid {:?}", test.values);
}