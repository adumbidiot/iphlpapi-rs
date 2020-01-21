#[test]
fn get_adapters_info() {
    let adapters = iphlpapi::get_adapters_info().unwrap();
    dbg!(adapters);
}
