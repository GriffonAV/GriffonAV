use std::thread::sleep;
use std::time::Instant;
use std::time::Duration;
use abi_stable::{
    export_root_module,
    prefix_type::PrefixTypeTrait,
    sabi_extern_fn,
    std_types::{RResult, RString},
};
use interface::{PluginI, PluginRoot, PluginRoot_Ref};


#[sabi_extern_fn]
fn start() -> RResult<(), RString> {
    let interval = Duration::from_secs(5);
    let mut next_time = Instant::now() + interval;

    loop {
        println!("Hi from plugin test 2!");
        if let Some(d) = next_time.checked_duration_since(Instant::now()) {
            sleep(d);
        }
        while next_time <= Instant::now() {
            next_time += interval;
        }
    }
}

#[export_root_module]
pub fn get_library() -> PluginRoot_Ref {
    PluginRoot {
        plugin: PluginI { start }.leak_into_prefix(),
    }
    .leak_into_prefix()
}
