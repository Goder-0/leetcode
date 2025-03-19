pub mod task;

#[cfg(test)]
mod tests {

    use task::*;

    use super::*;

    #[test]
    fn add() {
        assert_eq!(8, task::add::sum(3, 5));
        assert_eq!(200, task::add::sum(100, 100));
    }
}
