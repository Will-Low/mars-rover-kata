use mars_rover_kata_rust::execute;

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

// #[test]
// fn rover_faces_west_after_rotating_left_once() {
//     assert_eq!(execute("L"), "0:0:W");
// }
