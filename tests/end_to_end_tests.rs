use mars_rover::Grid;

fn ten_by_ten_grid() -> Grid {
    Grid::new(10, 10)
}

fn five_by_five_grid() -> Grid {
    Grid::new(5, 5)
}

#[test]
fn rover_starts_at_0_0_facing_north() {
    let grid = ten_by_ten_grid();
    assert_eq!(grid.execute(""), "0:0:N");
}

#[test]
fn rover_faces_east_after_rotating_right_once() {
    let grid = ten_by_ten_grid();
    assert_eq!(grid.execute("R"), "0:0:E");
}

#[test]
fn rover_faces_south_after_rotating_right_twice() {
    let grid = ten_by_ten_grid();
    assert_eq!(grid.execute("RR"), "0:0:S");
}

#[test]
fn rover_faces_west_after_rotating_right_thrice() {
    let grid = ten_by_ten_grid();
    assert_eq!(grid.execute("RRR"), "0:0:W");
}

#[test]
fn rover_faces_north_after_rotating_right_four_times() {
    let grid = ten_by_ten_grid();
    assert_eq!(grid.execute("RRRR"), "0:0:N");
}

#[test]
fn rover_faces_west_after_rotating_left_once() {
    let grid = ten_by_ten_grid();
    assert_eq!(grid.execute("L"), "0:0:W");
}

#[test]
fn rover_faces_south_after_rotating_left_twice() {
    let grid = ten_by_ten_grid();
    assert_eq!(grid.execute("LL"), "0:0:S");
}

#[test]
fn rover_faces_east_after_rotating_left_thrice() {
    let grid = ten_by_ten_grid();
    assert_eq!(grid.execute("LLL"), "0:0:E");
}

#[test]
fn rover_faces_north_after_rotating_left_four_times() {
    let grid = ten_by_ten_grid();
    assert_eq!(grid.execute("LLLL"), "0:0:N");
}

#[test]
fn rover_moves_north() {
    let grid = ten_by_ten_grid();
    assert_eq!(grid.execute("M"), "0:1:N");
}

#[test]
fn rover_moves_east() {
    let grid = ten_by_ten_grid();
    assert_eq!(grid.execute("RM"), "1:0:E");
}

#[test]
fn rover_moves_south_and_wraps_around() {
    let grid = ten_by_ten_grid();
    assert_eq!(grid.execute("RRM"), "0:9:S");

    let small_grid = five_by_five_grid();
    assert_eq!(small_grid.execute("RRM"), "0:4:S");
}

#[test]
fn rover_moves_north_and_wraps_around() {
    let grid = ten_by_ten_grid();
    assert_eq!(grid.execute("MMMMMMMMMMM"), "0:1:N");

    let small_grid = five_by_five_grid();
    assert_eq!(small_grid.execute("MMMMMM"), "0:1:N");
}

#[test]
fn rover_moves_east_and_wraps_around() {
    let grid = ten_by_ten_grid();
    assert_eq!(grid.execute("RMMMMMMMMMMM"), "1:0:E");

    let small_grid = five_by_five_grid();
    assert_eq!(small_grid.execute("RMMMMMM"), "1:0:E");
}

#[test]
fn rover_moves_west_and_wraps_around() {
    let grid = ten_by_ten_grid();
    assert_eq!(grid.execute("LM"), "9:0:W");

    let small_grid = five_by_five_grid();
    assert_eq!(small_grid.execute("LM"), "4:0:W");
}

#[test]
fn moving_on_a_single_square_grid() {
    let grid = Grid::new(1, 1);
    assert_eq!(grid.execute("M"), "0:0:N");
    assert_eq!(grid.execute("RM"), "0:0:E");
    assert_eq!(grid.execute("RRM"), "0:0:S");
    assert_eq!(grid.execute("RRRM"), "0:0:W");
}

#[test]
#[should_panic]
fn grid_of_height_zero_not_allowed() {
    Grid::new(1, 0);
}

#[test]
#[should_panic]
fn grid_of_width_zero_not_allowed() {
    Grid::new(0, 1);
}
