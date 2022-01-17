extern crate warp_devices;

use enum_iterator::IntoEnumIterator;
use std::thread::sleep;
use std::time::Duration;
use warp_devices::{
    cms::{CardMgmtOps, CardMgmtSys, CmsReg},
    varium_c1100::VariumC1100,
};

fn main() {
    let mut varium = VariumC1100::new().expect("cannot construct device");
    varium.init_cms().expect("cannot initialise CMS");

    // Expect to wait up to at least 1ms.
    match varium.expect_ready_host_status(1000) {
        Ok(us) => println!("CMS became ready after {}µs", us),
        Err(e) => {
            println!("CMS is not ready: {:?}", e);
            std::process::exit(1);
        }
    }

    varium
        .enable_hbm_temp_monitoring()
        .expect("cannot enable HBM temp monitor");

    // Wait 1ms to allow readings to be populated.
    sleep(Duration::from_millis(1));

    for reg in CmsReg::into_enum_iter() {
        println!(
            "{:?} = {}",
            reg,
            varium.get_cms_reg(reg).expect("no reading")
        );
    }
}
