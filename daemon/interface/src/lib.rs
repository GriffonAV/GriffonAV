use abi_stable::StableAbi;
use abi_stable::library::RootModule;
use abi_stable::std_types::{RResult, RString};

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = PluginRef)))]
pub struct PluginI {
    pub start: extern "C" fn() -> RResult<(), RString>,
}

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix))]
#[sabi(missing_field(default))]
pub struct PluginRoot {
    #[sabi(last_prefix_field)]
    pub plugin: PluginRef,
}

impl RootModule for PluginRoot_Ref {
    abi_stable::declare_root_module_statics! {PluginRoot_Ref}
    const BASE_NAME: &'static str = "Griffon_Plugin";
    const NAME: &'static str = "Griffon_Plugin";
    const VERSION_STRINGS: abi_stable::sabi_types::VersionStrings =
        abi_stable::package_version_strings!();
}
