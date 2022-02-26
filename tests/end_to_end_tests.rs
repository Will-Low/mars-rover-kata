use mars_rover::execute;

#[test]
fn rover_starts_at_0_0_facing_north() {
    assert_eq!(execute(""), "0:0:N");
}

#[test]
fn rover_faces_east_after_rotating_right_once() {
    assert_eq!(execute("R"), "0:0:E");
}

#[test]
fn rover_faces_south_after_rotating_right_twice() {
    assert_eq!(execute("RR"), "0:0:S");
}

#[test]
fn rover_faces_west_after_rotating_right_thrice() {
    assert_eq!(execute("RRR"), "0:0:W");
}

#[test]
fn rover_faces_north_after_rotating_right_four_times() {
    assert_eq!(execute("RRRR"), "0:0:N");
}

#[test]
fn rover_faces_west_after_rotating_left_once() {
    assert_eq!(execute("L"), "0:0:W");
}

#[test]
fn rover_faces_south_after_rotating_left_twice() {
    assert_eq!(execute("LL"), "0:0:S");
}

#[test]
fn rover_faces_east_after_rotating_left_thrice() {
    assert_eq!(execute("LLL"), "0:0:E");
}

#[test]
fn rover_faces_north_after_rotating_left_four_times() {
    assert_eq!(execute("LLLL"), "0:0:N");
}

#[test]
fn rover_moves_north() {
    assert_eq!(execute("M"), "0:1:N");
}

#[test]
fn rover_moves_east() {
    assert_eq!(execute("RM"), "1:0:E");
}

#[test]
fn rover_moves_south_and_wraps_around() {
    assert_eq!(execute("RRM"), "0:9:S");
}

/*
   TODO
   [ ] Top wraparound
   [ ] Bottom wraparound
   [ ] Right wraparound
   [ ] Left wraparound
*/
