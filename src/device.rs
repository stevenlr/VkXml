use crate::types::*;
use crate::instance::Instance;
use crate::commands::DeviceCommands;

#[derive(Clone)]
pub struct Device {
    handle: VkDevice,
    d: DeviceCommands,
}

impl Device {
    pub fn new(device: VkDevice, instance: &Instance) -> Self {
        let commands = DeviceCommands::load(|fn_name| {
            unsafe { instance.i.get_device_proc_addr(device, fn_name.as_ptr()) }
        });
        Self {
            handle: device,
            d: commands,
        }
    }

    pub fn acquire_next_image_khr(&self,
        swapchain: VkSwapchainKHR,
        timeout: u64,
        semaphore: VkSemaphore,
        fence: VkFence) -> Result<(VkResult, u32), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.acquire_next_image_khr(
                self.handle,
                swapchain,
                timeout,
                semaphore,
                fence,
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            VkResult::TIMEOUT => Ok((ret, ret_value)),
            VkResult::NOT_READY => Ok((ret, ret_value)),
            VkResult::SUBOPTIMAL_KHR => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn create_swapchain_khr(&self,
        p_create_info: &VkSwapchainCreateInfoKHR,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkSwapchainKHR), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_swapchain_khr(
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

    pub fn create_command_pool(&self,
        p_create_info: &VkCommandPoolCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkCommandPool), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_command_pool(
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

    pub fn get_render_area_granularity(&self,
        render_pass: VkRenderPass) -> VkExtent2D {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.get_render_area_granularity(
                self.handle,
                render_pass,
                &mut ret_value,)
        };
        return ret_value;
    }

    pub fn create_render_pass(&self,
        p_create_info: &VkRenderPassCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkRenderPass), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_render_pass(
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

    pub fn create_framebuffer(&self,
        p_create_info: &VkFramebufferCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkFramebuffer), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_framebuffer(
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

    pub fn create_descriptor_pool(&self,
        p_create_info: &VkDescriptorPoolCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkDescriptorPool), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_descriptor_pool(
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

    pub fn create_descriptor_set_layout(&self,
        p_create_info: &VkDescriptorSetLayoutCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkDescriptorSetLayout), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_descriptor_set_layout(
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

    pub fn create_sampler(&self,
        p_create_info: &VkSamplerCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkSampler), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_sampler(
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

    pub fn create_pipeline_layout(&self,
        p_create_info: &VkPipelineLayoutCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkPipelineLayout), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_pipeline_layout(
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

    pub fn create_pipeline_cache(&self,
        p_create_info: &VkPipelineCacheCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkPipelineCache), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_pipeline_cache(
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

    pub fn create_shader_module(&self,
        p_create_info: &VkShaderModuleCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkShaderModule), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_shader_module(
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

    pub fn create_image_view(&self,
        p_create_info: &VkImageViewCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkImageView), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_image_view(
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

    pub fn get_image_subresource_layout(&self,
        image: VkImage,
        p_subresource: &VkImageSubresource) -> VkSubresourceLayout {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.get_image_subresource_layout(
                self.handle,
                image,
                p_subresource,
                &mut ret_value,)
        };
        return ret_value;
    }

    pub fn create_image(&self,
        p_create_info: &VkImageCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkImage), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_image(
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

    pub fn create_buffer_view(&self,
        p_create_info: &VkBufferViewCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkBufferView), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_buffer_view(
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

    pub fn create_buffer(&self,
        p_create_info: &VkBufferCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkBuffer), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_buffer(
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

    pub fn create_query_pool(&self,
        p_create_info: &VkQueryPoolCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkQueryPool), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_query_pool(
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

    pub fn create_event(&self,
        p_create_info: &VkEventCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkEvent), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_event(
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

    pub fn create_semaphore(&self,
        p_create_info: &VkSemaphoreCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkSemaphore), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_semaphore(
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

    pub fn create_fence(&self,
        p_create_info: &VkFenceCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkFence), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_fence(
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

    pub fn get_image_memory_requirements(&self,
        image: VkImage) -> VkMemoryRequirements {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.get_image_memory_requirements(
                self.handle,
                image,
                &mut ret_value,)
        };
        return ret_value;
    }

    pub fn get_buffer_memory_requirements(&self,
        buffer: VkBuffer) -> VkMemoryRequirements {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.get_buffer_memory_requirements(
                self.handle,
                buffer,
                &mut ret_value,)
        };
        return ret_value;
    }

    pub fn get_device_memory_commitment(&self,
        memory: VkDeviceMemory) -> VkDeviceSize {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.get_device_memory_commitment(
                self.handle,
                memory,
                &mut ret_value,)
        };
        return ret_value;
    }

    pub fn map_memory(&self,
        memory: VkDeviceMemory,
        offset: VkDeviceSize,
        size: VkDeviceSize,
        flags: VkMemoryMapFlags) -> Result<(VkResult, *mut core::ffi::c_void), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.map_memory(
                self.handle,
                memory,
                offset,
                size,
                flags,
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn allocate_memory(&self,
        p_allocate_info: &VkMemoryAllocateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkDeviceMemory), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.allocate_memory(
                self.handle,
                p_allocate_info,
                match p_allocator { Some(r) => r, None => core::ptr::null(),},
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn get_device_queue(&self,
        queue_family_index: u32,
        queue_index: u32) -> VkQueue {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.get_device_queue(
                self.handle,
                queue_family_index,
                queue_index,
                &mut ret_value,)
        };
        return ret_value;
    }
}
