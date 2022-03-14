extern crate domain;

#[cfg(test)]
mod domain_tests {
    #[test]
    fn test_add() {
        let en = domain::Entity{};
        assert_eq!(domain::Domain::add(&en, 1,2), 3);
    }
}