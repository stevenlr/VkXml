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
                match p_allocator { Some(r) => r, None => core::ptr::null() },
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn get_physical_device_surface_present_modes_khr_count(&self,
        physical_device: VkPhysicalDevice,
        surface: VkSurfaceKHR) -> Result<(VkResult, usize), VkResult> {
        let mut p_present_mode_count = 0;
        let ret = unsafe {
            self.i.get_physical_device_surface_present_modes_khr(
                physical_device,
                surface,
                &mut p_present_mode_count,
                core::ptr::null_mut(),)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, p_present_mode_count as usize)),
            VkResult::INCOMPLETE => Ok((ret, p_present_mode_count as usize)),
            _ => Err(ret),
        };
    }

    pub fn get_physical_device_surface_present_modes_khr(&self,
        physical_device: VkPhysicalDevice,
        surface: VkSurfaceKHR,
        p_present_modes: &mut [VkPresentModeKHR]) -> Result<VkResult, VkResult> {
        let mut p_present_mode_count = p_present_modes.len() as _;
        let ret = unsafe {
            self.i.get_physical_device_surface_present_modes_khr(
                physical_device,
                surface,
                &mut p_present_mode_count,
                core::mem::transmute(p_present_modes.as_mut_ptr()),)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            VkResult::INCOMPLETE => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn get_physical_device_surface_formats_khr_count(&self,
        physical_device: VkPhysicalDevice,
        surface: VkSurfaceKHR) -> Result<(VkResult, usize), VkResult> {
        let mut p_surface_format_count = 0;
        let ret = unsafe {
            self.i.get_physical_device_surface_formats_khr(
                physical_device,
                surface,
                &mut p_surface_format_count,
                core::ptr::null_mut(),)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, p_surface_format_count as usize)),
            VkResult::INCOMPLETE => Ok((ret, p_surface_format_count as usize)),
            _ => Err(ret),
        };
    }

    pub fn get_physical_device_surface_formats_khr(&self,
        physical_device: VkPhysicalDevice,
        surface: VkSurfaceKHR,
        p_surface_formats: &mut [VkSurfaceFormatKHR]) -> Result<VkResult, VkResult> {
        let mut p_surface_format_count = p_surface_formats.len() as _;
        let ret = unsafe {
            self.i.get_physical_device_surface_formats_khr(
                physical_device,
                surface,
                &mut p_surface_format_count,
                core::mem::transmute(p_surface_formats.as_mut_ptr()),)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            VkResult::INCOMPLETE => Ok(ret),
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
                match p_allocator { Some(r) => r, None => core::ptr::null() },
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn get_physical_device_sparse_image_format_properties_count(&self,
        physical_device: VkPhysicalDevice,
        format: VkFormat,
        kind: VkImageType,
        samples: VkSampleCountFlagBits,
        usage: VkImageUsageFlags,
        tiling: VkImageTiling) -> usize {
        let mut p_property_count = 0;
        let ret = unsafe {
            self.i.get_physical_device_sparse_image_format_properties(
                physical_device,
                format,
                kind,
                samples,
                usage,
                tiling,
                &mut p_property_count,
                core::ptr::null_mut(),)
        };
        return p_property_count as usize;
    }

    pub fn get_physical_device_sparse_image_format_properties(&self,
        physical_device: VkPhysicalDevice,
        format: VkFormat,
        kind: VkImageType,
        samples: VkSampleCountFlagBits,
        usage: VkImageUsageFlags,
        tiling: VkImageTiling,
        p_properties: &mut [VkSparseImageFormatProperties]) {
        let mut p_property_count = p_properties.len() as _;
        let ret = unsafe {
            self.i.get_physical_device_sparse_image_format_properties(
                physical_device,
                format,
                kind,
                samples,
                usage,
                tiling,
                &mut p_property_count,
                core::mem::transmute(p_properties.as_mut_ptr()),)
        };
    }

    pub fn enumerate_device_layer_properties_count(&self,
        physical_device: VkPhysicalDevice) -> Result<(VkResult, usize), VkResult> {
        let mut p_property_count = 0;
        let ret = unsafe {
            self.i.enumerate_device_layer_properties(
                physical_device,
                &mut p_property_count,
                core::ptr::null_mut(),)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, p_property_count as usize)),
            VkResult::INCOMPLETE => Ok((ret, p_property_count as usize)),
            _ => Err(ret),
        };
    }

    pub fn enumerate_device_layer_properties(&self,
        physical_device: VkPhysicalDevice,
        p_properties: &mut [VkLayerProperties]) -> Result<VkResult, VkResult> {
        let mut p_property_count = p_properties.len() as _;
        let ret = unsafe {
            self.i.enumerate_device_layer_properties(
                physical_device,
                &mut p_property_count,
                core::mem::transmute(p_properties.as_mut_ptr()),)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            VkResult::INCOMPLETE => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn enumerate_device_extension_properties_count(&self,
        physical_device: VkPhysicalDevice,
        p_layer_name: Option<&[u8]>) -> Result<(VkResult, usize), VkResult> {
        let mut p_property_count = 0;
        let ret = unsafe {
            self.i.enumerate_device_extension_properties(
                physical_device,
                match p_layer_name { Some(r) => r.as_ptr(), None => core::ptr::null() },
                &mut p_property_count,
                core::ptr::null_mut(),)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, p_property_count as usize)),
            VkResult::INCOMPLETE => Ok((ret, p_property_count as usize)),
            _ => Err(ret),
        };
    }

    pub fn enumerate_device_extension_properties(&self,
        physical_device: VkPhysicalDevice,
        p_layer_name: Option<&[u8]>,
        p_properties: &mut [VkExtensionProperties]) -> Result<VkResult, VkResult> {
        let mut p_property_count = p_properties.len() as _;
        let ret = unsafe {
            self.i.enumerate_device_extension_properties(
                physical_device,
                match p_layer_name { Some(r) => r.as_ptr(), None => core::ptr::null() },
                &mut p_property_count,
                core::mem::transmute(p_properties.as_mut_ptr()),)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            VkResult::INCOMPLETE => Ok(ret),
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
                match p_allocator { Some(r) => r, None => core::ptr::null() },
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

    pub fn get_physical_device_queue_family_properties_count(&self,
        physical_device: VkPhysicalDevice) -> usize {
        let mut p_queue_family_property_count = 0;
        let ret = unsafe {
            self.i.get_physical_device_queue_family_properties(
                physical_device,
                &mut p_queue_family_property_count,
                core::ptr::null_mut(),)
        };
        return p_queue_family_property_count as usize;
    }

    pub fn get_physical_device_queue_family_properties(&self,
        physical_device: VkPhysicalDevice,
        p_queue_family_properties: &mut [VkQueueFamilyProperties]) {
        let mut p_queue_family_property_count = p_queue_family_properties.len() as _;
        let ret = unsafe {
            self.i.get_physical_device_queue_family_properties(
                physical_device,
                &mut p_queue_family_property_count,
                core::mem::transmute(p_queue_family_properties.as_mut_ptr()),)
        };
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

    pub fn enumerate_physical_devices_count(&self) -> Result<(VkResult, usize), VkResult> {
        let mut p_physical_device_count = 0;
        let ret = unsafe {
            self.i.enumerate_physical_devices(
                self.handle,
                &mut p_physical_device_count,
                core::ptr::null_mut(),)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, p_physical_device_count as usize)),
            VkResult::INCOMPLETE => Ok((ret, p_physical_device_count as usize)),
            _ => Err(ret),
        };
    }

    pub fn enumerate_physical_devices(&self,
        p_physical_devices: &mut [VkPhysicalDevice]) -> Result<VkResult, VkResult> {
        let mut p_physical_device_count = p_physical_devices.len() as _;
        let ret = unsafe {
            self.i.enumerate_physical_devices(
                self.handle,
                &mut p_physical_device_count,
                core::mem::transmute(p_physical_devices.as_mut_ptr()),)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            VkResult::INCOMPLETE => Ok(ret),
            _ => Err(ret),
        };
    }
}
