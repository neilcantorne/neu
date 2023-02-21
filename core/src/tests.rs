#[test]
fn query_device() {
    let iter = crate::Device::devices(crate::Backend::All).unwrap();

    for result in iter {
        let _device = result.unwrap();
    }
}

