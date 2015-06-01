extern crate parsetime;

#[test]
fn noon() {
    assert_eq!(12, parsetime::parsetime("noon").tm_hour)
}

#[test]
fn midnight() {
    assert_eq!(0, parsetime::parsetime("midnight").tm_hour)
}

#[test]
fn teatime() {
    assert_eq!(16, parsetime::parsetime("teatime").tm_hour)
}
