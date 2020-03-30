pub fn pop_too_much() -> bool {
    let mut list = vec![3];

    let last = list.pop().unwrap();
    println!("The last item in the list is {:?}", last);

    let second_to_last = list.pop().unwrap_or(100);
    println!(
        "The second-to-last item in the list is {:?}",
        second_to_last
    );
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_not_panic() {
        assert!(pop_too_much());
    }
}
