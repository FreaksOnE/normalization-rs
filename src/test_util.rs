use rust_decimal::Decimal;

pub fn decimals(values: &[i64]) -> Vec<Decimal> {
    values.iter().copied().map(Decimal::from).collect()
}

pub fn assert_all_close(actual: &[Decimal], expected: &[Decimal]) {
    assert_eq!(actual.len(), expected.len(), "length mismatch");
    for (index, (got, want)) in actual.iter().zip(expected).enumerate() {
        assert!(
            (got - want).abs() <= Decimal::new(1, 12),
            "element {index}: got {got}, want {want}"
        );
    }
}
