use mars_rover::Grid;

fn standard_grid() -> Grid {
    Grid::new(10, 10)
}

#[test]
fn rover_starts_at_0_0_facing_north() {
    let grid = standard_grid();
    assert_eq!(grid.execute(""), "0:0:N");
}

#[test]
fn rover_faces_east_after_rotating_right_once() {
    let grid = standard_grid();
    assert_eq!(grid.execute("R"), "0:0:E");
}

#[test]
fn rover_faces_south_after_rotating_right_twice() {
    let grid = standard_grid();
    assert_eq!(grid.execute("RR"), "0:0:S");
}

#[test]
fn rover_faces_west_after_rotating_right_thrice() {
    let grid = standard_grid();
    assert_eq!(grid.execute("RRR"), "0:0:W");
}

#[test]
fn rover_faces_north_after_rotating_right_four_times() {
    let grid = standard_grid();
    assert_eq!(grid.execute("RRRR"), "0:0:N");
}

#[test]
fn rover_faces_west_after_rotating_left_once() {
    let grid = standard_grid();
    assert_eq!(grid.execute("L"), "0:0:W");
}

#[test]
fn rover_faces_south_after_rotating_left_twice() {
    let grid = standard_grid();
    assert_eq!(grid.execute("LL"), "0:0:S");
}

#[test]
fn rover_faces_east_after_rotating_left_thrice() {
    let grid = standard_grid();
    assert_eq!(grid.execute("LLL"), "0:0:E");
}

#[test]
fn rover_faces_north_after_rotating_left_four_times() {
    let grid = standard_grid();
    assert_eq!(grid.execute("LLLL"), "0:0:N");
}

#[test]
fn rover_moves_north() {
    let grid = standard_grid();
    assert_eq!(grid.execute("M"), "0:1:N");
}

#[test]
fn rover_moves_east() {
    let grid = standard_grid();
    assert_eq!(grid.execute("RM"), "1:0:E");
}

#[test]
fn rover_moves_south_and_wraps_around() {
    let grid = standard_grid();
    assert_eq!(grid.execute("RRM"), "0:9:S");
}

#[test]
fn rover_moves_north_and_wraps_around() {
    let grid = standard_grid();
    assert_eq!(grid.execute("MMMMMMMMMMM"), "0:1:N");
}

#[test]
fn rover_moves_east_and_wraps_around() {
    let grid = standard_grid();
    assert_eq!(grid.execute("RMMMMMMMMMMM"), "1:0:E");
}

#[test]
fn rover_moves_west_and_wraps_around() {
    let grid = standard_grid();
    assert_eq!(grid.execute("LM"), "9:0:W");
}

/*
   TODO
   [ ] Top wraparound
   [ ] Bottom wraparound
   [ ] Right wraparound
   [ ] Left wraparound
   [ ] 0 sized grid
*/
