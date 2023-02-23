use crate::{BackendApi, Device};

#[test]
fn query_device() {
    for device in Device::devices(BackendApi::All).unwrap().flatten() {
        if device.backend() == BackendApi::Cuda {
            let cuda_name = device.name().unwrap();
            println!("CUDA: {cuda_name}");
        } else {
            let cl_name = device.name().unwrap();
            println!("OpenCL: {cl_name}");

            let _engine = device.create_engine().unwrap();
        }
    }
}

