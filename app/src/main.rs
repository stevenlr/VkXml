extern crate vk;

use core::ffi::c_void;
use vk::types::*;
use vk::builders::*;

type HModule = usize;

#[link(name = "kernel32")]
extern "system" {
    fn LoadLibraryA(lib_name: *const u8) -> HModule;
    fn GetProcAddress(module: HModule, fn_name: *const u8) -> *const c_void;
}

fn main() {
    let module = unsafe {
        LoadLibraryA(b"vulkan-1.dll\0".as_ptr())
    };

    let vk_entry = vk::EntryPoint::new(|fn_name: &[u8]| unsafe {
        core::mem::transmute(GetProcAddress(module, fn_name.as_ptr()))
    });

    let extensions = &[
        VK_EXT_DEBUG_UTILS_EXTENSION_NAME__C.as_ptr(),
        VK_KHR_SURFACE_EXTENSION_NAME__C.as_ptr(),
        VK_KHR_WIN32_SURFACE_EXTENSION_NAME__C.as_ptr(),
    ];

    let layers = &[
        b"VK_LAYER_LUNARG_standard_validation\0".as_ptr(),
    ];

    let create_info = VkInstanceCreateInfoBuilder::new()
        .pp_enabled_extension_names(extensions)
        .pp_enabled_layer_names(layers);

    let instance = vk_entry.create_instance(&create_info, None).unwrap().1;
    let instance = vk::Instance::new(instance, &vk_entry);

    let gpu_count = instance.enumerate_physical_devices_count().unwrap().1;
    println!("{} GPU(s)", gpu_count);

    let mut gpus = Vec::new();
    gpus.resize(gpu_count, VkPhysicalDevice::null());
    instance.enumerate_physical_devices(&mut gpus).unwrap();

    for (index, gpu) in gpus.iter().enumerate() {
        let prps = instance.get_physical_device_properties(*gpu);
        let name = unsafe { std::ffi::CStr::from_ptr(prps.device_name.as_ptr() as *const i8) };
        println!("    {}: {} {:?}", index, name.to_str().unwrap(), prps.device_type);
    }

    instance.destroy_instance(None);
}
