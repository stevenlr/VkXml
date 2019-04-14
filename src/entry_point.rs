use crate::types::*;
use crate::commands::{StaticCommands, EntryCommands};

#[derive(Clone)]
pub struct EntryPoint {
    pub(crate) s: StaticCommands,
    pub(crate) e: EntryCommands,
}

impl EntryPoint {
    pub fn new(load_fn: impl Fn(&[u8]) -> PfnVkVoidFunction) -> Self {
        let static_commands = StaticCommands::load(load_fn);
        let entry_commands = EntryCommands::load(|fn_name| {
            unsafe { static_commands.get_instance_proc_addr(VkInstance::null(), fn_name.as_ptr()) }
        });
        Self {
            s: static_commands,
            e: entry_commands,
        }
    }

    pub fn create_instance(&self,
        p_create_info: &VkInstanceCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkInstance), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.e.create_instance(
                p_create_info,
                match p_allocator { Some(r) => r, None => core::ptr::null(),},
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }
}
