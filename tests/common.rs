#[test]
fn common() {
    let adapters = iphlpapi::get_adapters_info().unwrap();
    println!("Num Adapters: {}", adapters.iter().count());
    for adapter in adapters.iter() {
        println!("Combo Index: {}", adapter.get_combo_index());
        println!("Adapter Name: {}", adapter.get_name().to_string_lossy());
        println!(
            "Description: {}",
            adapter.get_description().to_string_lossy()
        );
        print!("Hardware Address: ");
        let addresses = adapter.get_address();
        for (i, b) in addresses.iter().enumerate() {
            if i == addresses.len() - 1 {
                print!("{:02X}", b);
            } else {
                print!("{:02X}-", b);
            }
        }
        println!();

        println!("IP Address List: ");
        for ip in adapter.get_ip_address_list().iter() {
            println!("    Ip: {}", ip.get_address().to_string_lossy());
            println!("    Mask: {}", ip.get_mask().to_string_lossy());
            println!();
        }

        println!("Gateway List: ");
        for ip in adapter.get_gateway_list().iter() {
            println!("    Ip: {}", ip.get_address().to_string_lossy());
            println!("    Mask: {}", ip.get_mask().to_string_lossy());
            println!();
        }

        println!();
    }
}

#[test]
fn adapter_list_debug() {
    let adapters = iphlpapi::get_adapters_info().unwrap();
    dbg!(adapters);
}
