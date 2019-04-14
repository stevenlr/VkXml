use crate::types::*;
use crate::entry_point::EntryPoint;
use crate::commands::InstanceCommands;

#[derive(Clone)]
pub struct Instance {
    pub(crate) handle: VkInstance,
    pub(crate) i: InstanceCommands,
}

impl Instance {
    pub fn new(instance: VkInstance, entry: &EntryPoint) -> Self {
        let commands = InstanceCommands::load(|fn_name| {
            unsafe { entry.s.get_instance_proc_addr(instance, fn_name.as_ptr()) }
        });
        Self {
            handle: instance,
            i: commands,
        }
    }

    pub fn create_win_32_surface_khr(&self,
        p_create_info: &VkWin32SurfaceCreateInfoKHR,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkSurfaceKHR), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.i.create_win_32_surface_khr(
                self.handle,
                p_create_info,
                match p_allocator { Some(r) => r, None => core::ptr::null(),},
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn get_physical_device_surface_capabilities_khr(&self,
        physical_device: VkPhysicalDevice,
        surface: VkSurfaceKHR) -> Result<(VkResult, VkSurfaceCapabilitiesKHR), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.i.get_physical_device_surface_capabilities_khr(
                physical_device,
                surface,
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn get_physical_device_surface_support_khr(&self,
        physical_device: VkPhysicalDevice,
        queue_family_index: u32,
        surface: VkSurfaceKHR) -> Result<(VkResult, bool), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.i.get_physical_device_surface_support_khr(
                physical_device,
                queue_family_index,
                surface,
                &mut ret_value,)
        };
        let ret_value = ret_value == VK_TRUE;
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn create_debug_utils_messenger_ext(&self,
        p_create_info: &VkDebugUtilsMessengerCreateInfoEXT,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkDebugUtilsMessengerEXT), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.i.create_debug_utils_messenger_ext(
                self.handle,
                p_create_info,
                match p_allocator { Some(r) => r, None => core::ptr::null(),},
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn create_device(&self,
        physical_device: VkPhysicalDevice,
        p_create_info: &VkDeviceCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkDevice), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.i.create_device(
                physical_device,
                p_create_info,
                match p_allocator { Some(r) => r, None => core::ptr::null(),},
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn get_physical_device_memory_properties(&self,
        physical_device: VkPhysicalDevice) -> VkPhysicalDeviceMemoryProperties {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.i.get_physical_device_memory_properties(
                physical_device,
                &mut ret_value,)
        };
        return ret_value;
    }

    pub fn get_physical_device_properties(&self,
        physical_device: VkPhysicalDevice) -> VkPhysicalDeviceProperties {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.i.get_physical_device_properties(
                physical_device,
                &mut ret_value,)
        };
        return ret_value;
    }

    pub fn get_physical_device_image_format_properties(&self,
        physical_device: VkPhysicalDevice,
        format: VkFormat,
        kind: VkImageType,
        tiling: VkImageTiling,
        usage: VkImageUsageFlags,
        flags: VkImageCreateFlags) -> Result<(VkResult, VkImageFormatProperties), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.i.get_physical_device_image_format_properties(
                physical_device,
                format,
                kind,
                tiling,
                usage,
                flags,
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn get_physical_device_format_properties(&self,
        physical_device: VkPhysicalDevice,
        format: VkFormat) -> VkFormatProperties {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.i.get_physical_device_format_properties(
                physical_device,
                format,
                &mut ret_value,)
        };
        return ret_value;
    }

    pub fn get_physical_device_features(&self,
        physical_device: VkPhysicalDevice) -> VkPhysicalDeviceFeatures {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.i.get_physical_device_features(
                physical_device,
                &mut ret_value,)
        };
        return ret_value;
    }
}
