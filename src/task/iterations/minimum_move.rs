#[must_use]
pub fn minimum_number_of_moves_to_seat_everyone(seats: &mut [i32], students: &mut [i32]) -> i32 {
    seats.sort_unstable();
    students.sort_unstable();

    seats
        .iter()
        .zip(students.iter())
        .map(|(se, st)| (st - se).abs())
        .sum()
}
