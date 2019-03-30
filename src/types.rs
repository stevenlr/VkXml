pub type HINSTANCE = usize;
pub type HWND = usize;

pub const VK_KHR_WIN32_SURFACE_SPEC_VERSION: u32 = 6;
pub const VK_KHR_SURFACE_SPEC_VERSION: u32 = 25;
pub const VK_KHR_SWAPCHAIN_SPEC_VERSION: u32 = 70;
pub const VK_EXT_DEBUG_UTILS_SPEC_VERSION: u32 = 1;
pub const VK_SUBPASS_EXTERNAL: u32 = 4294967295;
pub const VK_QUEUE_FAMILY_IGNORED: u32 = 4294967295;
pub const VK_FALSE: u32 = 0;
pub const VK_TRUE: u32 = 1;
pub const VK_ATTACHMENT_UNUSED: u32 = 4294967295;
pub const VK_WHOLE_SIZE: u64 = 18446744073709551615;
pub const VK_REMAINING_ARRAY_LAYERS: u32 = 4294967295;
pub const VK_REMAINING_MIP_LEVELS: u32 = 4294967295;

pub const VK_LOD_CLAMP_NONE: f32 = 1000.000000;

pub const VK_KHR_WIN32_SURFACE_EXTENSION_NAME: &str = "VK_KHR_win32_surface";
pub const VK_KHR_WIN32_SURFACE_EXTENSION_NAME__C: &[u8] = b"VK_KHR_win32_surface\0";
pub const VK_KHR_SURFACE_EXTENSION_NAME: &str = "VK_KHR_surface";
pub const VK_KHR_SURFACE_EXTENSION_NAME__C: &[u8] = b"VK_KHR_surface\0";
pub const VK_KHR_SWAPCHAIN_EXTENSION_NAME: &str = "VK_KHR_swapchain";
pub const VK_KHR_SWAPCHAIN_EXTENSION_NAME__C: &[u8] = b"VK_KHR_swapchain\0";
pub const VK_EXT_DEBUG_UTILS_EXTENSION_NAME: &str = "VK_EXT_debug_utils";
pub const VK_EXT_DEBUG_UTILS_EXTENSION_NAME__C: &[u8] = b"VK_EXT_debug_utils\0";

pub type VkBool32 = u32;
pub type VkFlags = u32;
pub type VkDeviceSize = u64;
pub type VkSampleMask = u32;

pub type VkImageUsageFlags = VkImageUsageFlagBits;
pub type VkCompositeAlphaFlagsKHR = VkCompositeAlphaFlagBitsKHR;
pub type VkSurfaceTransformFlagsKHR = VkSurfaceTransformFlagBitsKHR;
pub type VkSwapchainCreateFlagsKHR = VkSwapchainCreateFlagBitsKHR;
pub type VkDebugUtilsMessageTypeFlagsEXT = VkDebugUtilsMessageTypeFlagBitsEXT;
pub type VkDebugUtilsMessageSeverityFlagsEXT = VkDebugUtilsMessageSeverityFlagBitsEXT;
pub type VkAccessFlags = VkAccessFlagBits;
pub type VkImageAspectFlags = VkImageAspectFlagBits;
pub type VkShaderStageFlags = VkShaderStageFlagBits;
pub type VkQueryResultFlags = VkQueryResultFlagBits;
pub type VkQueryControlFlags = VkQueryControlFlagBits;
pub type VkDependencyFlags = VkDependencyFlagBits;
pub type VkPipelineStageFlags = VkPipelineStageFlagBits;
pub type VkStencilFaceFlags = VkStencilFaceFlagBits;
pub type VkCommandBufferResetFlags = VkCommandBufferResetFlagBits;
pub type VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlagBits;
pub type VkCommandBufferUsageFlags = VkCommandBufferUsageFlagBits;
pub type VkCommandPoolResetFlags = VkCommandPoolResetFlagBits;
pub type VkCommandPoolCreateFlags = VkCommandPoolCreateFlagBits;
pub type VkSubpassDescriptionFlags = VkSubpassDescriptionFlagBits;
pub type VkAttachmentDescriptionFlags = VkAttachmentDescriptionFlagBits;
pub type VkDescriptorPoolCreateFlags = VkDescriptorPoolCreateFlagBits;
pub type VkDescriptorSetLayoutCreateFlags = VkDescriptorSetLayoutCreateFlagBits;
pub type VkSamplerCreateFlags = VkSamplerCreateFlagBits;
pub type VkPipelineCreateFlags = VkPipelineCreateFlagBits;
pub type VkColorComponentFlags = VkColorComponentFlagBits;
pub type VkCullModeFlags = VkCullModeFlagBits;
pub type VkImageViewCreateFlags = VkImageViewCreateFlagBits;
pub type VkImageCreateFlags = VkImageCreateFlagBits;
pub type VkBufferUsageFlags = VkBufferUsageFlagBits;
pub type VkBufferCreateFlags = VkBufferCreateFlagBits;
pub type VkFenceCreateFlags = VkFenceCreateFlagBits;
pub type VkSparseMemoryBindFlags = VkSparseMemoryBindFlagBits;
pub type VkSparseImageFormatFlags = VkSparseImageFormatFlagBits;
pub type VkDeviceQueueCreateFlags = VkDeviceQueueCreateFlagBits;
pub type VkMemoryHeapFlags = VkMemoryHeapFlagBits;
pub type VkMemoryPropertyFlags = VkMemoryPropertyFlagBits;
pub type VkQueueFlags = VkQueueFlagBits;
pub type VkSampleCountFlags = VkSampleCountFlagBits;
pub type VkFormatFeatureFlags = VkFormatFeatureFlagBits;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct VkPhysicalDevice(usize);
impl VkPhysicalDevice {
    #[inline]
    pub fn from_raw(r: usize) -> Self {
        Self(r)
    }

    #[inline]
    pub fn as_raw(&self) -> usize {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct VkSurfaceKHR(u64);
impl VkSurfaceKHR {
    #[inline]
    pub fn from_raw(r: u64) -> Self {
        Self(r)
    }

    #[inline]
    pub fn as_raw(&self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct VkInstance(usize);
impl VkInstance {
    #[inline]
    pub fn from_raw(r: usize) -> Self {
        Self(r)
    }

    #[inline]
    pub fn as_raw(&self) -> usize {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct VkSwapchainKHR(u64);
impl VkSwapchainKHR {
    #[inline]
    pub fn from_raw(r: u64) -> Self {
        Self(r)
    }

    #[inline]
    pub fn as_raw(&self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct VkSemaphore(u64);
impl VkSemaphore {
    #[inline]
    pub fn from_raw(r: u64) -> Self {
        Self(r)
    }

    #[inline]
    pub fn as_raw(&self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct VkQueue(usize);
impl VkQueue {
    #[inline]
    pub fn from_raw(r: usize) -> Self {
        Self(r)
    }

    #[inline]
    pub fn as_raw(&self) -> usize {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct VkFence(u64);
impl VkFence {
    #[inline]
    pub fn from_raw(r: u64) -> Self {
        Self(r)
    }

    #[inline]
    pub fn as_raw(&self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct VkDevice(usize);
impl VkDevice {
    #[inline]
    pub fn from_raw(r: usize) -> Self {
        Self(r)
    }

    #[inline]
    pub fn as_raw(&self) -> usize {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct VkImage(u64);
impl VkImage {
    #[inline]
    pub fn from_raw(r: u64) -> Self {
        Self(r)
    }

    #[inline]
    pub fn as_raw(&self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct VkDebugUtilsMessengerEXT(u64);
impl VkDebugUtilsMessengerEXT {
    #[inline]
    pub fn from_raw(r: u64) -> Self {
        Self(r)
    }

    #[inline]
    pub fn as_raw(&self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct VkCommandBuffer(usize);
impl VkCommandBuffer {
    #[inline]
    pub fn from_raw(r: usize) -> Self {
        Self(r)
    }

    #[inline]
    pub fn as_raw(&self) -> usize {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct VkBuffer(u64);
impl VkBuffer {
    #[inline]
    pub fn from_raw(r: u64) -> Self {
        Self(r)
    }

    #[inline]
    pub fn as_raw(&self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct VkFramebuffer(u64);
impl VkFramebuffer {
    #[inline]
    pub fn from_raw(r: u64) -> Self {
        Self(r)
    }

    #[inline]
    pub fn as_raw(&self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct VkRenderPass(u64);
impl VkRenderPass {
    #[inline]
    pub fn from_raw(r: u64) -> Self {
        Self(r)
    }

    #[inline]
    pub fn as_raw(&self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct VkPipelineLayout(u64);
impl VkPipelineLayout {
    #[inline]
    pub fn from_raw(r: u64) -> Self {
        Self(r)
    }

    #[inline]
    pub fn as_raw(&self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct VkQueryPool(u64);
impl VkQueryPool {
    #[inline]
    pub fn from_raw(r: u64) -> Self {
        Self(r)
    }

    #[inline]
    pub fn as_raw(&self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct VkEvent(u64);
impl VkEvent {
    #[inline]
    pub fn from_raw(r: u64) -> Self {
        Self(r)
    }

    #[inline]
    pub fn as_raw(&self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct VkDescriptorSet(u64);
impl VkDescriptorSet {
    #[inline]
    pub fn from_raw(r: u64) -> Self {
        Self(r)
    }

    #[inline]
    pub fn as_raw(&self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct VkPipeline(u64);
impl VkPipeline {
    #[inline]
    pub fn from_raw(r: u64) -> Self {
        Self(r)
    }

    #[inline]
    pub fn as_raw(&self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct VkCommandPool(u64);
impl VkCommandPool {
    #[inline]
    pub fn from_raw(r: u64) -> Self {
        Self(r)
    }

    #[inline]
    pub fn as_raw(&self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct VkImageView(u64);
impl VkImageView {
    #[inline]
    pub fn from_raw(r: u64) -> Self {
        Self(r)
    }

    #[inline]
    pub fn as_raw(&self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct VkBufferView(u64);
impl VkBufferView {
    #[inline]
    pub fn from_raw(r: u64) -> Self {
        Self(r)
    }

    #[inline]
    pub fn as_raw(&self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct VkSampler(u64);
impl VkSampler {
    #[inline]
    pub fn from_raw(r: u64) -> Self {
        Self(r)
    }

    #[inline]
    pub fn as_raw(&self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct VkDescriptorPool(u64);
impl VkDescriptorPool {
    #[inline]
    pub fn from_raw(r: u64) -> Self {
        Self(r)
    }

    #[inline]
    pub fn as_raw(&self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct VkDescriptorSetLayout(u64);
impl VkDescriptorSetLayout {
    #[inline]
    pub fn from_raw(r: u64) -> Self {
        Self(r)
    }

    #[inline]
    pub fn as_raw(&self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct VkShaderModule(u64);
impl VkShaderModule {
    #[inline]
    pub fn from_raw(r: u64) -> Self {
        Self(r)
    }

    #[inline]
    pub fn as_raw(&self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct VkPipelineCache(u64);
impl VkPipelineCache {
    #[inline]
    pub fn from_raw(r: u64) -> Self {
        Self(r)
    }

    #[inline]
    pub fn as_raw(&self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct VkDeviceMemory(u64);
impl VkDeviceMemory {
    #[inline]
    pub fn from_raw(r: u64) -> Self {
        Self(r)
    }

    #[inline]
    pub fn as_raw(&self) -> u64 {
        self.0
    }
}

pub type VkWin32SurfaceCreateFlagsKHR = VkFlags;
pub type VkDebugUtilsMessengerCallbackDataFlagsEXT = VkFlags;
pub type VkDebugUtilsMessengerCreateFlagsEXT = VkFlags;
pub type VkRenderPassCreateFlags = VkFlags;
pub type VkFramebufferCreateFlags = VkFlags;
pub type VkDescriptorPoolResetFlags = VkFlags;
pub type VkPipelineLayoutCreateFlags = VkFlags;
pub type VkPipelineShaderStageCreateFlags = VkFlags;
pub type VkPipelineDynamicStateCreateFlags = VkFlags;
pub type VkPipelineColorBlendStateCreateFlags = VkFlags;
pub type VkPipelineDepthStencilStateCreateFlags = VkFlags;
pub type VkPipelineMultisampleStateCreateFlags = VkFlags;
pub type VkPipelineRasterizationStateCreateFlags = VkFlags;
pub type VkPipelineViewportStateCreateFlags = VkFlags;
pub type VkPipelineTessellationStateCreateFlags = VkFlags;
pub type VkPipelineInputAssemblyStateCreateFlags = VkFlags;
pub type VkPipelineVertexInputStateCreateFlags = VkFlags;
pub type VkPipelineCacheCreateFlags = VkFlags;
pub type VkShaderModuleCreateFlags = VkFlags;
pub type VkBufferViewCreateFlags = VkFlags;
pub type VkQueryPoolCreateFlags = VkFlags;
pub type VkEventCreateFlags = VkFlags;
pub type VkSemaphoreCreateFlags = VkFlags;
pub type VkMemoryMapFlags = VkFlags;
pub type VkDeviceCreateFlags = VkFlags;
pub type VkInstanceCreateFlags = VkFlags;

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkSystemAllocationScope(u32);
impl VkSystemAllocationScope {
    pub const COMMAND: VkSystemAllocationScope = VkSystemAllocationScope(0);
    pub const OBJECT: VkSystemAllocationScope = VkSystemAllocationScope(1);
    pub const CACHE: VkSystemAllocationScope = VkSystemAllocationScope(2);
    pub const DEVICE: VkSystemAllocationScope = VkSystemAllocationScope(3);
    pub const INSTANCE: VkSystemAllocationScope = VkSystemAllocationScope(4);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkInternalAllocationType(u32);
impl VkInternalAllocationType {
    pub const EXECUTABLE: VkInternalAllocationType = VkInternalAllocationType(0);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkStructureType(u32);
impl VkStructureType {
    pub const APPLICATION_INFO: VkStructureType = VkStructureType(0);
    pub const INSTANCE_CREATE_INFO: VkStructureType = VkStructureType(1);
    pub const DEVICE_QUEUE_CREATE_INFO: VkStructureType = VkStructureType(2);
    pub const DEVICE_CREATE_INFO: VkStructureType = VkStructureType(3);
    pub const SUBMIT_INFO: VkStructureType = VkStructureType(4);
    pub const MEMORY_ALLOCATE_INFO: VkStructureType = VkStructureType(5);
    pub const MAPPED_MEMORY_RANGE: VkStructureType = VkStructureType(6);
    pub const BIND_SPARSE_INFO: VkStructureType = VkStructureType(7);
    pub const FENCE_CREATE_INFO: VkStructureType = VkStructureType(8);
    pub const SEMAPHORE_CREATE_INFO: VkStructureType = VkStructureType(9);
    pub const EVENT_CREATE_INFO: VkStructureType = VkStructureType(10);
    pub const QUERY_POOL_CREATE_INFO: VkStructureType = VkStructureType(11);
    pub const BUFFER_CREATE_INFO: VkStructureType = VkStructureType(12);
    pub const BUFFER_VIEW_CREATE_INFO: VkStructureType = VkStructureType(13);
    pub const IMAGE_CREATE_INFO: VkStructureType = VkStructureType(14);
    pub const IMAGE_VIEW_CREATE_INFO: VkStructureType = VkStructureType(15);
    pub const SHADER_MODULE_CREATE_INFO: VkStructureType = VkStructureType(16);
    pub const PIPELINE_CACHE_CREATE_INFO: VkStructureType = VkStructureType(17);
    pub const PIPELINE_SHADER_STAGE_CREATE_INFO: VkStructureType = VkStructureType(18);
    pub const PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO: VkStructureType = VkStructureType(19);
    pub const PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO: VkStructureType = VkStructureType(20);
    pub const PIPELINE_TESSELLATION_STATE_CREATE_INFO: VkStructureType = VkStructureType(21);
    pub const PIPELINE_VIEWPORT_STATE_CREATE_INFO: VkStructureType = VkStructureType(22);
    pub const PIPELINE_RASTERIZATION_STATE_CREATE_INFO: VkStructureType = VkStructureType(23);
    pub const PIPELINE_MULTISAMPLE_STATE_CREATE_INFO: VkStructureType = VkStructureType(24);
    pub const PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO: VkStructureType = VkStructureType(25);
    pub const PIPELINE_COLOR_BLEND_STATE_CREATE_INFO: VkStructureType = VkStructureType(26);
    pub const PIPELINE_DYNAMIC_STATE_CREATE_INFO: VkStructureType = VkStructureType(27);
    pub const GRAPHICS_PIPELINE_CREATE_INFO: VkStructureType = VkStructureType(28);
    pub const COMPUTE_PIPELINE_CREATE_INFO: VkStructureType = VkStructureType(29);
    pub const PIPELINE_LAYOUT_CREATE_INFO: VkStructureType = VkStructureType(30);
    pub const SAMPLER_CREATE_INFO: VkStructureType = VkStructureType(31);
    pub const DESCRIPTOR_SET_LAYOUT_CREATE_INFO: VkStructureType = VkStructureType(32);
    pub const DESCRIPTOR_POOL_CREATE_INFO: VkStructureType = VkStructureType(33);
    pub const DESCRIPTOR_SET_ALLOCATE_INFO: VkStructureType = VkStructureType(34);
    pub const WRITE_DESCRIPTOR_SET: VkStructureType = VkStructureType(35);
    pub const COPY_DESCRIPTOR_SET: VkStructureType = VkStructureType(36);
    pub const FRAMEBUFFER_CREATE_INFO: VkStructureType = VkStructureType(37);
    pub const RENDER_PASS_CREATE_INFO: VkStructureType = VkStructureType(38);
    pub const COMMAND_POOL_CREATE_INFO: VkStructureType = VkStructureType(39);
    pub const COMMAND_BUFFER_ALLOCATE_INFO: VkStructureType = VkStructureType(40);
    pub const COMMAND_BUFFER_INHERITANCE_INFO: VkStructureType = VkStructureType(41);
    pub const COMMAND_BUFFER_BEGIN_INFO: VkStructureType = VkStructureType(42);
    pub const RENDER_PASS_BEGIN_INFO: VkStructureType = VkStructureType(43);
    pub const BUFFER_MEMORY_BARRIER: VkStructureType = VkStructureType(44);
    pub const IMAGE_MEMORY_BARRIER: VkStructureType = VkStructureType(45);
    pub const MEMORY_BARRIER: VkStructureType = VkStructureType(46);
    pub const LOADER_INSTANCE_CREATE_INFO: VkStructureType = VkStructureType(47);
    pub const LOADER_DEVICE_CREATE_INFO: VkStructureType = VkStructureType(48);
    pub const SWAPCHAIN_CREATE_INFO_KHR: VkStructureType = VkStructureType(1000001000);
    pub const PRESENT_INFO_KHR: VkStructureType = VkStructureType(1000001001);
    pub const WIN32_SURFACE_CREATE_INFO_KHR: VkStructureType = VkStructureType(1000009000);
    pub const DEBUG_UTILS_OBJECT_NAME_INFO_EXT: VkStructureType = VkStructureType(1000128000);
    pub const DEBUG_UTILS_OBJECT_TAG_INFO_EXT: VkStructureType = VkStructureType(1000128001);
    pub const DEBUG_UTILS_LABEL_EXT: VkStructureType = VkStructureType(1000128002);
    pub const DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT: VkStructureType = VkStructureType(1000128003);
    pub const DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT: VkStructureType = VkStructureType(1000128004);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkResult(u32);
impl VkResult {
    pub const ERROR_OUT_OF_DATE_KHR: VkResult = VkResult(-1000001004i32 as u32);
    pub const ERROR_NATIVE_WINDOW_IN_USE_KHR: VkResult = VkResult(-1000000001i32 as u32);
    pub const ERROR_SURFACE_LOST_KHR: VkResult = VkResult(-1000000000i32 as u32);
    pub const ERROR_FRAGMENTED_POOL: VkResult = VkResult(-12i32 as u32);
    pub const ERROR_FORMAT_NOT_SUPPORTED: VkResult = VkResult(-11i32 as u32);
    pub const ERROR_TOO_MANY_OBJECTS: VkResult = VkResult(-10i32 as u32);
    pub const ERROR_INCOMPATIBLE_DRIVER: VkResult = VkResult(-9i32 as u32);
    pub const ERROR_FEATURE_NOT_PRESENT: VkResult = VkResult(-8i32 as u32);
    pub const ERROR_EXTENSION_NOT_PRESENT: VkResult = VkResult(-7i32 as u32);
    pub const ERROR_LAYER_NOT_PRESENT: VkResult = VkResult(-6i32 as u32);
    pub const ERROR_MEMORY_MAP_FAILED: VkResult = VkResult(-5i32 as u32);
    pub const ERROR_DEVICE_LOST: VkResult = VkResult(-4i32 as u32);
    pub const ERROR_INITIALIZATION_FAILED: VkResult = VkResult(-3i32 as u32);
    pub const ERROR_OUT_OF_DEVICE_MEMORY: VkResult = VkResult(-2i32 as u32);
    pub const ERROR_OUT_OF_HOST_MEMORY: VkResult = VkResult(-1i32 as u32);
    pub const SUCCESS: VkResult = VkResult(0);
    pub const NOT_READY: VkResult = VkResult(1);
    pub const TIMEOUT: VkResult = VkResult(2);
    pub const EVENT_SET: VkResult = VkResult(3);
    pub const EVENT_RESET: VkResult = VkResult(4);
    pub const INCOMPLETE: VkResult = VkResult(5);
    pub const SUBOPTIMAL_KHR: VkResult = VkResult(1000001003);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkPresentModeKHR(u32);
impl VkPresentModeKHR {
    pub const IMMEDIATE_KHR: VkPresentModeKHR = VkPresentModeKHR(0);
    pub const MAILBOX_KHR: VkPresentModeKHR = VkPresentModeKHR(1);
    pub const FIFO_KHR: VkPresentModeKHR = VkPresentModeKHR(2);
    pub const FIFO_RELAXED_KHR: VkPresentModeKHR = VkPresentModeKHR(3);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkColorSpaceKHR(u32);
impl VkColorSpaceKHR {
    pub const SRGB_NONLINEAR_KHR: VkColorSpaceKHR = VkColorSpaceKHR(0);
    pub const VK_COLORSPACE_SRGB_NONLINEAR_KHR: VkColorSpaceKHR = VkColorSpaceKHR(0);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkFormat(u32);
impl VkFormat {
    pub const UNDEFINED: VkFormat = VkFormat(0);
    pub const R4G4_UNORM_PACK8: VkFormat = VkFormat(1);
    pub const R4G4B4A4_UNORM_PACK16: VkFormat = VkFormat(2);
    pub const B4G4R4A4_UNORM_PACK16: VkFormat = VkFormat(3);
    pub const R5G6B5_UNORM_PACK16: VkFormat = VkFormat(4);
    pub const B5G6R5_UNORM_PACK16: VkFormat = VkFormat(5);
    pub const R5G5B5A1_UNORM_PACK16: VkFormat = VkFormat(6);
    pub const B5G5R5A1_UNORM_PACK16: VkFormat = VkFormat(7);
    pub const A1R5G5B5_UNORM_PACK16: VkFormat = VkFormat(8);
    pub const R8_UNORM: VkFormat = VkFormat(9);
    pub const R8_SNORM: VkFormat = VkFormat(10);
    pub const R8_USCALED: VkFormat = VkFormat(11);
    pub const R8_SSCALED: VkFormat = VkFormat(12);
    pub const R8_UINT: VkFormat = VkFormat(13);
    pub const R8_SINT: VkFormat = VkFormat(14);
    pub const R8_SRGB: VkFormat = VkFormat(15);
    pub const R8G8_UNORM: VkFormat = VkFormat(16);
    pub const R8G8_SNORM: VkFormat = VkFormat(17);
    pub const R8G8_USCALED: VkFormat = VkFormat(18);
    pub const R8G8_SSCALED: VkFormat = VkFormat(19);
    pub const R8G8_UINT: VkFormat = VkFormat(20);
    pub const R8G8_SINT: VkFormat = VkFormat(21);
    pub const R8G8_SRGB: VkFormat = VkFormat(22);
    pub const R8G8B8_UNORM: VkFormat = VkFormat(23);
    pub const R8G8B8_SNORM: VkFormat = VkFormat(24);
    pub const R8G8B8_USCALED: VkFormat = VkFormat(25);
    pub const R8G8B8_SSCALED: VkFormat = VkFormat(26);
    pub const R8G8B8_UINT: VkFormat = VkFormat(27);
    pub const R8G8B8_SINT: VkFormat = VkFormat(28);
    pub const R8G8B8_SRGB: VkFormat = VkFormat(29);
    pub const B8G8R8_UNORM: VkFormat = VkFormat(30);
    pub const B8G8R8_SNORM: VkFormat = VkFormat(31);
    pub const B8G8R8_USCALED: VkFormat = VkFormat(32);
    pub const B8G8R8_SSCALED: VkFormat = VkFormat(33);
    pub const B8G8R8_UINT: VkFormat = VkFormat(34);
    pub const B8G8R8_SINT: VkFormat = VkFormat(35);
    pub const B8G8R8_SRGB: VkFormat = VkFormat(36);
    pub const R8G8B8A8_UNORM: VkFormat = VkFormat(37);
    pub const R8G8B8A8_SNORM: VkFormat = VkFormat(38);
    pub const R8G8B8A8_USCALED: VkFormat = VkFormat(39);
    pub const R8G8B8A8_SSCALED: VkFormat = VkFormat(40);
    pub const R8G8B8A8_UINT: VkFormat = VkFormat(41);
    pub const R8G8B8A8_SINT: VkFormat = VkFormat(42);
    pub const R8G8B8A8_SRGB: VkFormat = VkFormat(43);
    pub const B8G8R8A8_UNORM: VkFormat = VkFormat(44);
    pub const B8G8R8A8_SNORM: VkFormat = VkFormat(45);
    pub const B8G8R8A8_USCALED: VkFormat = VkFormat(46);
    pub const B8G8R8A8_SSCALED: VkFormat = VkFormat(47);
    pub const B8G8R8A8_UINT: VkFormat = VkFormat(48);
    pub const B8G8R8A8_SINT: VkFormat = VkFormat(49);
    pub const B8G8R8A8_SRGB: VkFormat = VkFormat(50);
    pub const A8B8G8R8_UNORM_PACK32: VkFormat = VkFormat(51);
    pub const A8B8G8R8_SNORM_PACK32: VkFormat = VkFormat(52);
    pub const A8B8G8R8_USCALED_PACK32: VkFormat = VkFormat(53);
    pub const A8B8G8R8_SSCALED_PACK32: VkFormat = VkFormat(54);
    pub const A8B8G8R8_UINT_PACK32: VkFormat = VkFormat(55);
    pub const A8B8G8R8_SINT_PACK32: VkFormat = VkFormat(56);
    pub const A8B8G8R8_SRGB_PACK32: VkFormat = VkFormat(57);
    pub const A2R10G10B10_UNORM_PACK32: VkFormat = VkFormat(58);
    pub const A2R10G10B10_SNORM_PACK32: VkFormat = VkFormat(59);
    pub const A2R10G10B10_USCALED_PACK32: VkFormat = VkFormat(60);
    pub const A2R10G10B10_SSCALED_PACK32: VkFormat = VkFormat(61);
    pub const A2R10G10B10_UINT_PACK32: VkFormat = VkFormat(62);
    pub const A2R10G10B10_SINT_PACK32: VkFormat = VkFormat(63);
    pub const A2B10G10R10_UNORM_PACK32: VkFormat = VkFormat(64);
    pub const A2B10G10R10_SNORM_PACK32: VkFormat = VkFormat(65);
    pub const A2B10G10R10_USCALED_PACK32: VkFormat = VkFormat(66);
    pub const A2B10G10R10_SSCALED_PACK32: VkFormat = VkFormat(67);
    pub const A2B10G10R10_UINT_PACK32: VkFormat = VkFormat(68);
    pub const A2B10G10R10_SINT_PACK32: VkFormat = VkFormat(69);
    pub const R16_UNORM: VkFormat = VkFormat(70);
    pub const R16_SNORM: VkFormat = VkFormat(71);
    pub const R16_USCALED: VkFormat = VkFormat(72);
    pub const R16_SSCALED: VkFormat = VkFormat(73);
    pub const R16_UINT: VkFormat = VkFormat(74);
    pub const R16_SINT: VkFormat = VkFormat(75);
    pub const R16_SFLOAT: VkFormat = VkFormat(76);
    pub const R16G16_UNORM: VkFormat = VkFormat(77);
    pub const R16G16_SNORM: VkFormat = VkFormat(78);
    pub const R16G16_USCALED: VkFormat = VkFormat(79);
    pub const R16G16_SSCALED: VkFormat = VkFormat(80);
    pub const R16G16_UINT: VkFormat = VkFormat(81);
    pub const R16G16_SINT: VkFormat = VkFormat(82);
    pub const R16G16_SFLOAT: VkFormat = VkFormat(83);
    pub const R16G16B16_UNORM: VkFormat = VkFormat(84);
    pub const R16G16B16_SNORM: VkFormat = VkFormat(85);
    pub const R16G16B16_USCALED: VkFormat = VkFormat(86);
    pub const R16G16B16_SSCALED: VkFormat = VkFormat(87);
    pub const R16G16B16_UINT: VkFormat = VkFormat(88);
    pub const R16G16B16_SINT: VkFormat = VkFormat(89);
    pub const R16G16B16_SFLOAT: VkFormat = VkFormat(90);
    pub const R16G16B16A16_UNORM: VkFormat = VkFormat(91);
    pub const R16G16B16A16_SNORM: VkFormat = VkFormat(92);
    pub const R16G16B16A16_USCALED: VkFormat = VkFormat(93);
    pub const R16G16B16A16_SSCALED: VkFormat = VkFormat(94);
    pub const R16G16B16A16_UINT: VkFormat = VkFormat(95);
    pub const R16G16B16A16_SINT: VkFormat = VkFormat(96);
    pub const R16G16B16A16_SFLOAT: VkFormat = VkFormat(97);
    pub const R32_UINT: VkFormat = VkFormat(98);
    pub const R32_SINT: VkFormat = VkFormat(99);
    pub const R32_SFLOAT: VkFormat = VkFormat(100);
    pub const R32G32_UINT: VkFormat = VkFormat(101);
    pub const R32G32_SINT: VkFormat = VkFormat(102);
    pub const R32G32_SFLOAT: VkFormat = VkFormat(103);
    pub const R32G32B32_UINT: VkFormat = VkFormat(104);
    pub const R32G32B32_SINT: VkFormat = VkFormat(105);
    pub const R32G32B32_SFLOAT: VkFormat = VkFormat(106);
    pub const R32G32B32A32_UINT: VkFormat = VkFormat(107);
    pub const R32G32B32A32_SINT: VkFormat = VkFormat(108);
    pub const R32G32B32A32_SFLOAT: VkFormat = VkFormat(109);
    pub const R64_UINT: VkFormat = VkFormat(110);
    pub const R64_SINT: VkFormat = VkFormat(111);
    pub const R64_SFLOAT: VkFormat = VkFormat(112);
    pub const R64G64_UINT: VkFormat = VkFormat(113);
    pub const R64G64_SINT: VkFormat = VkFormat(114);
    pub const R64G64_SFLOAT: VkFormat = VkFormat(115);
    pub const R64G64B64_UINT: VkFormat = VkFormat(116);
    pub const R64G64B64_SINT: VkFormat = VkFormat(117);
    pub const R64G64B64_SFLOAT: VkFormat = VkFormat(118);
    pub const R64G64B64A64_UINT: VkFormat = VkFormat(119);
    pub const R64G64B64A64_SINT: VkFormat = VkFormat(120);
    pub const R64G64B64A64_SFLOAT: VkFormat = VkFormat(121);
    pub const B10G11R11_UFLOAT_PACK32: VkFormat = VkFormat(122);
    pub const E5B9G9R9_UFLOAT_PACK32: VkFormat = VkFormat(123);
    pub const D16_UNORM: VkFormat = VkFormat(124);
    pub const X8_D24_UNORM_PACK32: VkFormat = VkFormat(125);
    pub const D32_SFLOAT: VkFormat = VkFormat(126);
    pub const S8_UINT: VkFormat = VkFormat(127);
    pub const D16_UNORM_S8_UINT: VkFormat = VkFormat(128);
    pub const D24_UNORM_S8_UINT: VkFormat = VkFormat(129);
    pub const D32_SFLOAT_S8_UINT: VkFormat = VkFormat(130);
    pub const BC1_RGB_UNORM_BLOCK: VkFormat = VkFormat(131);
    pub const BC1_RGB_SRGB_BLOCK: VkFormat = VkFormat(132);
    pub const BC1_RGBA_UNORM_BLOCK: VkFormat = VkFormat(133);
    pub const BC1_RGBA_SRGB_BLOCK: VkFormat = VkFormat(134);
    pub const BC2_UNORM_BLOCK: VkFormat = VkFormat(135);
    pub const BC2_SRGB_BLOCK: VkFormat = VkFormat(136);
    pub const BC3_UNORM_BLOCK: VkFormat = VkFormat(137);
    pub const BC3_SRGB_BLOCK: VkFormat = VkFormat(138);
    pub const BC4_UNORM_BLOCK: VkFormat = VkFormat(139);
    pub const BC4_SNORM_BLOCK: VkFormat = VkFormat(140);
    pub const BC5_UNORM_BLOCK: VkFormat = VkFormat(141);
    pub const BC5_SNORM_BLOCK: VkFormat = VkFormat(142);
    pub const BC6H_UFLOAT_BLOCK: VkFormat = VkFormat(143);
    pub const BC6H_SFLOAT_BLOCK: VkFormat = VkFormat(144);
    pub const BC7_UNORM_BLOCK: VkFormat = VkFormat(145);
    pub const BC7_SRGB_BLOCK: VkFormat = VkFormat(146);
    pub const ETC2_R8G8B8_UNORM_BLOCK: VkFormat = VkFormat(147);
    pub const ETC2_R8G8B8_SRGB_BLOCK: VkFormat = VkFormat(148);
    pub const ETC2_R8G8B8A1_UNORM_BLOCK: VkFormat = VkFormat(149);
    pub const ETC2_R8G8B8A1_SRGB_BLOCK: VkFormat = VkFormat(150);
    pub const ETC2_R8G8B8A8_UNORM_BLOCK: VkFormat = VkFormat(151);
    pub const ETC2_R8G8B8A8_SRGB_BLOCK: VkFormat = VkFormat(152);
    pub const EAC_R11_UNORM_BLOCK: VkFormat = VkFormat(153);
    pub const EAC_R11_SNORM_BLOCK: VkFormat = VkFormat(154);
    pub const EAC_R11G11_UNORM_BLOCK: VkFormat = VkFormat(155);
    pub const EAC_R11G11_SNORM_BLOCK: VkFormat = VkFormat(156);
    pub const ASTC_4X4_UNORM_BLOCK: VkFormat = VkFormat(157);
    pub const ASTC_4X4_SRGB_BLOCK: VkFormat = VkFormat(158);
    pub const ASTC_5X4_UNORM_BLOCK: VkFormat = VkFormat(159);
    pub const ASTC_5X4_SRGB_BLOCK: VkFormat = VkFormat(160);
    pub const ASTC_5X5_UNORM_BLOCK: VkFormat = VkFormat(161);
    pub const ASTC_5X5_SRGB_BLOCK: VkFormat = VkFormat(162);
    pub const ASTC_6X5_UNORM_BLOCK: VkFormat = VkFormat(163);
    pub const ASTC_6X5_SRGB_BLOCK: VkFormat = VkFormat(164);
    pub const ASTC_6X6_UNORM_BLOCK: VkFormat = VkFormat(165);
    pub const ASTC_6X6_SRGB_BLOCK: VkFormat = VkFormat(166);
    pub const ASTC_8X5_UNORM_BLOCK: VkFormat = VkFormat(167);
    pub const ASTC_8X5_SRGB_BLOCK: VkFormat = VkFormat(168);
    pub const ASTC_8X6_UNORM_BLOCK: VkFormat = VkFormat(169);
    pub const ASTC_8X6_SRGB_BLOCK: VkFormat = VkFormat(170);
    pub const ASTC_8X8_UNORM_BLOCK: VkFormat = VkFormat(171);
    pub const ASTC_8X8_SRGB_BLOCK: VkFormat = VkFormat(172);
    pub const ASTC_10X5_UNORM_BLOCK: VkFormat = VkFormat(173);
    pub const ASTC_10X5_SRGB_BLOCK: VkFormat = VkFormat(174);
    pub const ASTC_10X6_UNORM_BLOCK: VkFormat = VkFormat(175);
    pub const ASTC_10X6_SRGB_BLOCK: VkFormat = VkFormat(176);
    pub const ASTC_10X8_UNORM_BLOCK: VkFormat = VkFormat(177);
    pub const ASTC_10X8_SRGB_BLOCK: VkFormat = VkFormat(178);
    pub const ASTC_10X10_UNORM_BLOCK: VkFormat = VkFormat(179);
    pub const ASTC_10X10_SRGB_BLOCK: VkFormat = VkFormat(180);
    pub const ASTC_12X10_UNORM_BLOCK: VkFormat = VkFormat(181);
    pub const ASTC_12X10_SRGB_BLOCK: VkFormat = VkFormat(182);
    pub const ASTC_12X12_UNORM_BLOCK: VkFormat = VkFormat(183);
    pub const ASTC_12X12_SRGB_BLOCK: VkFormat = VkFormat(184);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkObjectType(u32);
impl VkObjectType {
    pub const UNKNOWN: VkObjectType = VkObjectType(0);
    pub const INSTANCE: VkObjectType = VkObjectType(1);
    pub const PHYSICAL_DEVICE: VkObjectType = VkObjectType(2);
    pub const DEVICE: VkObjectType = VkObjectType(3);
    pub const QUEUE: VkObjectType = VkObjectType(4);
    pub const SEMAPHORE: VkObjectType = VkObjectType(5);
    pub const COMMAND_BUFFER: VkObjectType = VkObjectType(6);
    pub const FENCE: VkObjectType = VkObjectType(7);
    pub const DEVICE_MEMORY: VkObjectType = VkObjectType(8);
    pub const BUFFER: VkObjectType = VkObjectType(9);
    pub const IMAGE: VkObjectType = VkObjectType(10);
    pub const EVENT: VkObjectType = VkObjectType(11);
    pub const QUERY_POOL: VkObjectType = VkObjectType(12);
    pub const BUFFER_VIEW: VkObjectType = VkObjectType(13);
    pub const IMAGE_VIEW: VkObjectType = VkObjectType(14);
    pub const SHADER_MODULE: VkObjectType = VkObjectType(15);
    pub const PIPELINE_CACHE: VkObjectType = VkObjectType(16);
    pub const PIPELINE_LAYOUT: VkObjectType = VkObjectType(17);
    pub const RENDER_PASS: VkObjectType = VkObjectType(18);
    pub const PIPELINE: VkObjectType = VkObjectType(19);
    pub const DESCRIPTOR_SET_LAYOUT: VkObjectType = VkObjectType(20);
    pub const SAMPLER: VkObjectType = VkObjectType(21);
    pub const DESCRIPTOR_POOL: VkObjectType = VkObjectType(22);
    pub const DESCRIPTOR_SET: VkObjectType = VkObjectType(23);
    pub const FRAMEBUFFER: VkObjectType = VkObjectType(24);
    pub const COMMAND_POOL: VkObjectType = VkObjectType(25);
    pub const SURFACE_KHR: VkObjectType = VkObjectType(1000000000);
    pub const SWAPCHAIN_KHR: VkObjectType = VkObjectType(1000001000);
    pub const DEBUG_UTILS_MESSENGER_EXT: VkObjectType = VkObjectType(1000128000);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkSharingMode(u32);
impl VkSharingMode {
    pub const EXCLUSIVE: VkSharingMode = VkSharingMode(0);
    pub const CONCURRENT: VkSharingMode = VkSharingMode(1);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkImageLayout(u32);
impl VkImageLayout {
    pub const UNDEFINED: VkImageLayout = VkImageLayout(0);
    pub const GENERAL: VkImageLayout = VkImageLayout(1);
    pub const COLOR_ATTACHMENT_OPTIMAL: VkImageLayout = VkImageLayout(2);
    pub const DEPTH_STENCIL_ATTACHMENT_OPTIMAL: VkImageLayout = VkImageLayout(3);
    pub const DEPTH_STENCIL_READ_ONLY_OPTIMAL: VkImageLayout = VkImageLayout(4);
    pub const SHADER_READ_ONLY_OPTIMAL: VkImageLayout = VkImageLayout(5);
    pub const TRANSFER_SRC_OPTIMAL: VkImageLayout = VkImageLayout(6);
    pub const TRANSFER_DST_OPTIMAL: VkImageLayout = VkImageLayout(7);
    pub const PREINITIALIZED: VkImageLayout = VkImageLayout(8);
    pub const PRESENT_SRC_KHR: VkImageLayout = VkImageLayout(1000001002);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkVendorId(u32);
impl VkVendorId {
    pub const VIV: VkVendorId = VkVendorId(65537);
    pub const VSI: VkVendorId = VkVendorId(65538);
    pub const KAZAN: VkVendorId = VkVendorId(65539);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkSubpassContents(u32);
impl VkSubpassContents {
    pub const INLINE: VkSubpassContents = VkSubpassContents(0);
    pub const SECONDARY_COMMAND_BUFFERS: VkSubpassContents = VkSubpassContents(1);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkFilter(u32);
impl VkFilter {
    pub const NEAREST: VkFilter = VkFilter(0);
    pub const LINEAR: VkFilter = VkFilter(1);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkIndexType(u32);
impl VkIndexType {
    pub const UINT16: VkIndexType = VkIndexType(0);
    pub const UINT32: VkIndexType = VkIndexType(1);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkPipelineBindPoint(u32);
impl VkPipelineBindPoint {
    pub const GRAPHICS: VkPipelineBindPoint = VkPipelineBindPoint(0);
    pub const COMPUTE: VkPipelineBindPoint = VkPipelineBindPoint(1);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkCommandBufferLevel(u32);
impl VkCommandBufferLevel {
    pub const PRIMARY: VkCommandBufferLevel = VkCommandBufferLevel(0);
    pub const SECONDARY: VkCommandBufferLevel = VkCommandBufferLevel(1);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkAttachmentStoreOp(u32);
impl VkAttachmentStoreOp {
    pub const STORE: VkAttachmentStoreOp = VkAttachmentStoreOp(0);
    pub const DONT_CARE: VkAttachmentStoreOp = VkAttachmentStoreOp(1);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkAttachmentLoadOp(u32);
impl VkAttachmentLoadOp {
    pub const LOAD: VkAttachmentLoadOp = VkAttachmentLoadOp(0);
    pub const CLEAR: VkAttachmentLoadOp = VkAttachmentLoadOp(1);
    pub const DONT_CARE: VkAttachmentLoadOp = VkAttachmentLoadOp(2);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkDescriptorType(u32);
impl VkDescriptorType {
    pub const SAMPLER: VkDescriptorType = VkDescriptorType(0);
    pub const COMBINED_IMAGE_SAMPLER: VkDescriptorType = VkDescriptorType(1);
    pub const SAMPLED_IMAGE: VkDescriptorType = VkDescriptorType(2);
    pub const STORAGE_IMAGE: VkDescriptorType = VkDescriptorType(3);
    pub const UNIFORM_TEXEL_BUFFER: VkDescriptorType = VkDescriptorType(4);
    pub const STORAGE_TEXEL_BUFFER: VkDescriptorType = VkDescriptorType(5);
    pub const UNIFORM_BUFFER: VkDescriptorType = VkDescriptorType(6);
    pub const STORAGE_BUFFER: VkDescriptorType = VkDescriptorType(7);
    pub const UNIFORM_BUFFER_DYNAMIC: VkDescriptorType = VkDescriptorType(8);
    pub const STORAGE_BUFFER_DYNAMIC: VkDescriptorType = VkDescriptorType(9);
    pub const INPUT_ATTACHMENT: VkDescriptorType = VkDescriptorType(10);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkBorderColor(u32);
impl VkBorderColor {
    pub const FLOAT_TRANSPARENT_BLACK: VkBorderColor = VkBorderColor(0);
    pub const INT_TRANSPARENT_BLACK: VkBorderColor = VkBorderColor(1);
    pub const FLOAT_OPAQUE_BLACK: VkBorderColor = VkBorderColor(2);
    pub const INT_OPAQUE_BLACK: VkBorderColor = VkBorderColor(3);
    pub const FLOAT_OPAQUE_WHITE: VkBorderColor = VkBorderColor(4);
    pub const INT_OPAQUE_WHITE: VkBorderColor = VkBorderColor(5);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkCompareOp(u32);
impl VkCompareOp {
    pub const NEVER: VkCompareOp = VkCompareOp(0);
    pub const LESS: VkCompareOp = VkCompareOp(1);
    pub const EQUAL: VkCompareOp = VkCompareOp(2);
    pub const LESS_OR_EQUAL: VkCompareOp = VkCompareOp(3);
    pub const GREATER: VkCompareOp = VkCompareOp(4);
    pub const NOT_EQUAL: VkCompareOp = VkCompareOp(5);
    pub const GREATER_OR_EQUAL: VkCompareOp = VkCompareOp(6);
    pub const ALWAYS: VkCompareOp = VkCompareOp(7);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkSamplerAddressMode(u32);
impl VkSamplerAddressMode {
    pub const REPEAT: VkSamplerAddressMode = VkSamplerAddressMode(0);
    pub const MIRRORED_REPEAT: VkSamplerAddressMode = VkSamplerAddressMode(1);
    pub const CLAMP_TO_EDGE: VkSamplerAddressMode = VkSamplerAddressMode(2);
    pub const CLAMP_TO_BORDER: VkSamplerAddressMode = VkSamplerAddressMode(3);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkSamplerMipmapMode(u32);
impl VkSamplerMipmapMode {
    pub const NEAREST: VkSamplerMipmapMode = VkSamplerMipmapMode(0);
    pub const LINEAR: VkSamplerMipmapMode = VkSamplerMipmapMode(1);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkDynamicState(u32);
impl VkDynamicState {
    pub const VIEWPORT: VkDynamicState = VkDynamicState(0);
    pub const SCISSOR: VkDynamicState = VkDynamicState(1);
    pub const LINE_WIDTH: VkDynamicState = VkDynamicState(2);
    pub const DEPTH_BIAS: VkDynamicState = VkDynamicState(3);
    pub const BLEND_CONSTANTS: VkDynamicState = VkDynamicState(4);
    pub const DEPTH_BOUNDS: VkDynamicState = VkDynamicState(5);
    pub const STENCIL_COMPARE_MASK: VkDynamicState = VkDynamicState(6);
    pub const STENCIL_WRITE_MASK: VkDynamicState = VkDynamicState(7);
    pub const STENCIL_REFERENCE: VkDynamicState = VkDynamicState(8);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkBlendOp(u32);
impl VkBlendOp {
    pub const ADD: VkBlendOp = VkBlendOp(0);
    pub const SUBTRACT: VkBlendOp = VkBlendOp(1);
    pub const REVERSE_SUBTRACT: VkBlendOp = VkBlendOp(2);
    pub const MIN: VkBlendOp = VkBlendOp(3);
    pub const MAX: VkBlendOp = VkBlendOp(4);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkBlendFactor(u32);
impl VkBlendFactor {
    pub const ZERO: VkBlendFactor = VkBlendFactor(0);
    pub const ONE: VkBlendFactor = VkBlendFactor(1);
    pub const SRC_COLOR: VkBlendFactor = VkBlendFactor(2);
    pub const ONE_MINUS_SRC_COLOR: VkBlendFactor = VkBlendFactor(3);
    pub const DST_COLOR: VkBlendFactor = VkBlendFactor(4);
    pub const ONE_MINUS_DST_COLOR: VkBlendFactor = VkBlendFactor(5);
    pub const SRC_ALPHA: VkBlendFactor = VkBlendFactor(6);
    pub const ONE_MINUS_SRC_ALPHA: VkBlendFactor = VkBlendFactor(7);
    pub const DST_ALPHA: VkBlendFactor = VkBlendFactor(8);
    pub const ONE_MINUS_DST_ALPHA: VkBlendFactor = VkBlendFactor(9);
    pub const CONSTANT_COLOR: VkBlendFactor = VkBlendFactor(10);
    pub const ONE_MINUS_CONSTANT_COLOR: VkBlendFactor = VkBlendFactor(11);
    pub const CONSTANT_ALPHA: VkBlendFactor = VkBlendFactor(12);
    pub const ONE_MINUS_CONSTANT_ALPHA: VkBlendFactor = VkBlendFactor(13);
    pub const SRC_ALPHA_SATURATE: VkBlendFactor = VkBlendFactor(14);
    pub const SRC1_COLOR: VkBlendFactor = VkBlendFactor(15);
    pub const ONE_MINUS_SRC1_COLOR: VkBlendFactor = VkBlendFactor(16);
    pub const SRC1_ALPHA: VkBlendFactor = VkBlendFactor(17);
    pub const ONE_MINUS_SRC1_ALPHA: VkBlendFactor = VkBlendFactor(18);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkLogicOp(u32);
impl VkLogicOp {
    pub const CLEAR: VkLogicOp = VkLogicOp(0);
    pub const AND: VkLogicOp = VkLogicOp(1);
    pub const AND_REVERSE: VkLogicOp = VkLogicOp(2);
    pub const COPY: VkLogicOp = VkLogicOp(3);
    pub const AND_INVERTED: VkLogicOp = VkLogicOp(4);
    pub const NO_OP: VkLogicOp = VkLogicOp(5);
    pub const XOR: VkLogicOp = VkLogicOp(6);
    pub const OR: VkLogicOp = VkLogicOp(7);
    pub const NOR: VkLogicOp = VkLogicOp(8);
    pub const EQUIVALENT: VkLogicOp = VkLogicOp(9);
    pub const INVERT: VkLogicOp = VkLogicOp(10);
    pub const OR_REVERSE: VkLogicOp = VkLogicOp(11);
    pub const COPY_INVERTED: VkLogicOp = VkLogicOp(12);
    pub const OR_INVERTED: VkLogicOp = VkLogicOp(13);
    pub const NAND: VkLogicOp = VkLogicOp(14);
    pub const SET: VkLogicOp = VkLogicOp(15);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkStencilOp(u32);
impl VkStencilOp {
    pub const KEEP: VkStencilOp = VkStencilOp(0);
    pub const ZERO: VkStencilOp = VkStencilOp(1);
    pub const REPLACE: VkStencilOp = VkStencilOp(2);
    pub const INCREMENT_AND_CLAMP: VkStencilOp = VkStencilOp(3);
    pub const DECREMENT_AND_CLAMP: VkStencilOp = VkStencilOp(4);
    pub const INVERT: VkStencilOp = VkStencilOp(5);
    pub const INCREMENT_AND_WRAP: VkStencilOp = VkStencilOp(6);
    pub const DECREMENT_AND_WRAP: VkStencilOp = VkStencilOp(7);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkFrontFace(u32);
impl VkFrontFace {
    pub const COUNTER_CLOCKWISE: VkFrontFace = VkFrontFace(0);
    pub const CLOCKWISE: VkFrontFace = VkFrontFace(1);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkPolygonMode(u32);
impl VkPolygonMode {
    pub const FILL: VkPolygonMode = VkPolygonMode(0);
    pub const LINE: VkPolygonMode = VkPolygonMode(1);
    pub const POINT: VkPolygonMode = VkPolygonMode(2);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkPrimitiveTopology(u32);
impl VkPrimitiveTopology {
    pub const POINT_LIST: VkPrimitiveTopology = VkPrimitiveTopology(0);
    pub const LINE_LIST: VkPrimitiveTopology = VkPrimitiveTopology(1);
    pub const LINE_STRIP: VkPrimitiveTopology = VkPrimitiveTopology(2);
    pub const TRIANGLE_LIST: VkPrimitiveTopology = VkPrimitiveTopology(3);
    pub const TRIANGLE_STRIP: VkPrimitiveTopology = VkPrimitiveTopology(4);
    pub const TRIANGLE_FAN: VkPrimitiveTopology = VkPrimitiveTopology(5);
    pub const LINE_LIST_WITH_ADJACENCY: VkPrimitiveTopology = VkPrimitiveTopology(6);
    pub const LINE_STRIP_WITH_ADJACENCY: VkPrimitiveTopology = VkPrimitiveTopology(7);
    pub const TRIANGLE_LIST_WITH_ADJACENCY: VkPrimitiveTopology = VkPrimitiveTopology(8);
    pub const TRIANGLE_STRIP_WITH_ADJACENCY: VkPrimitiveTopology = VkPrimitiveTopology(9);
    pub const PATCH_LIST: VkPrimitiveTopology = VkPrimitiveTopology(10);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkVertexInputRate(u32);
impl VkVertexInputRate {
    pub const VERTEX: VkVertexInputRate = VkVertexInputRate(0);
    pub const INSTANCE: VkVertexInputRate = VkVertexInputRate(1);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkComponentSwizzle(u32);
impl VkComponentSwizzle {
    pub const IDENTITY: VkComponentSwizzle = VkComponentSwizzle(0);
    pub const ZERO: VkComponentSwizzle = VkComponentSwizzle(1);
    pub const ONE: VkComponentSwizzle = VkComponentSwizzle(2);
    pub const R: VkComponentSwizzle = VkComponentSwizzle(3);
    pub const G: VkComponentSwizzle = VkComponentSwizzle(4);
    pub const B: VkComponentSwizzle = VkComponentSwizzle(5);
    pub const A: VkComponentSwizzle = VkComponentSwizzle(6);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkImageViewType(u32);
impl VkImageViewType {
    pub const K_1D: VkImageViewType = VkImageViewType(0);
    pub const K_2D: VkImageViewType = VkImageViewType(1);
    pub const K_3D: VkImageViewType = VkImageViewType(2);
    pub const CUBE: VkImageViewType = VkImageViewType(3);
    pub const K_1D_ARRAY: VkImageViewType = VkImageViewType(4);
    pub const K_2D_ARRAY: VkImageViewType = VkImageViewType(5);
    pub const CUBE_ARRAY: VkImageViewType = VkImageViewType(6);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkImageTiling(u32);
impl VkImageTiling {
    pub const OPTIMAL: VkImageTiling = VkImageTiling(0);
    pub const LINEAR: VkImageTiling = VkImageTiling(1);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkImageType(u32);
impl VkImageType {
    pub const K_1D: VkImageType = VkImageType(0);
    pub const K_2D: VkImageType = VkImageType(1);
    pub const K_3D: VkImageType = VkImageType(2);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkQueryType(u32);
impl VkQueryType {
    pub const OCCLUSION: VkQueryType = VkQueryType(0);
    pub const PIPELINE_STATISTICS: VkQueryType = VkQueryType(1);
    pub const TIMESTAMP: VkQueryType = VkQueryType(2);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkPhysicalDeviceType(u32);
impl VkPhysicalDeviceType {
    pub const OTHER: VkPhysicalDeviceType = VkPhysicalDeviceType(0);
    pub const INTEGRATED_GPU: VkPhysicalDeviceType = VkPhysicalDeviceType(1);
    pub const DISCRETE_GPU: VkPhysicalDeviceType = VkPhysicalDeviceType(2);
    pub const VIRTUAL_GPU: VkPhysicalDeviceType = VkPhysicalDeviceType(3);
    pub const CPU: VkPhysicalDeviceType = VkPhysicalDeviceType(4);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VkPipelineCacheHeaderVersion(u32);
impl VkPipelineCacheHeaderVersion {
    pub const ONE: VkPipelineCacheHeaderVersion = VkPipelineCacheHeaderVersion(1);
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkImageUsageFlagBits(VkFlags);
impl VkImageUsageFlagBits {
    pub const TRANSFER_SRC_BIT: VkImageUsageFlagBits = VkImageUsageFlagBits(1);
    pub const TRANSFER_DST_BIT: VkImageUsageFlagBits = VkImageUsageFlagBits(2);
    pub const SAMPLED_BIT: VkImageUsageFlagBits = VkImageUsageFlagBits(4);
    pub const STORAGE_BIT: VkImageUsageFlagBits = VkImageUsageFlagBits(8);
    pub const COLOR_ATTACHMENT_BIT: VkImageUsageFlagBits = VkImageUsageFlagBits(16);
    pub const DEPTH_STENCIL_ATTACHMENT_BIT: VkImageUsageFlagBits = VkImageUsageFlagBits(32);
    pub const TRANSIENT_ATTACHMENT_BIT: VkImageUsageFlagBits = VkImageUsageFlagBits(64);
    pub const INPUT_ATTACHMENT_BIT: VkImageUsageFlagBits = VkImageUsageFlagBits(128);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkImageUsageFlagBits {
    type Output = VkImageUsageFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkImageUsageFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkImageUsageFlagBits {
    type Output = VkImageUsageFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkImageUsageFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkImageUsageFlagBits {
    type Output = VkImageUsageFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkImageUsageFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkCompositeAlphaFlagBitsKHR(VkFlags);
impl VkCompositeAlphaFlagBitsKHR {
    pub const OPAQUE_BIT_KHR: VkCompositeAlphaFlagBitsKHR = VkCompositeAlphaFlagBitsKHR(1);
    pub const PRE_MULTIPLIED_BIT_KHR: VkCompositeAlphaFlagBitsKHR = VkCompositeAlphaFlagBitsKHR(2);
    pub const POST_MULTIPLIED_BIT_KHR: VkCompositeAlphaFlagBitsKHR = VkCompositeAlphaFlagBitsKHR(4);
    pub const INHERIT_BIT_KHR: VkCompositeAlphaFlagBitsKHR = VkCompositeAlphaFlagBitsKHR(8);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkCompositeAlphaFlagBitsKHR {
    type Output = VkCompositeAlphaFlagBitsKHR;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkCompositeAlphaFlagBitsKHR {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkCompositeAlphaFlagBitsKHR {
    type Output = VkCompositeAlphaFlagBitsKHR;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkCompositeAlphaFlagBitsKHR {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkCompositeAlphaFlagBitsKHR {
    type Output = VkCompositeAlphaFlagBitsKHR;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkCompositeAlphaFlagBitsKHR {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkSurfaceTransformFlagBitsKHR(VkFlags);
impl VkSurfaceTransformFlagBitsKHR {
    pub const IDENTITY_BIT_KHR: VkSurfaceTransformFlagBitsKHR = VkSurfaceTransformFlagBitsKHR(1);
    pub const ROTATE_90_BIT_KHR: VkSurfaceTransformFlagBitsKHR = VkSurfaceTransformFlagBitsKHR(2);
    pub const ROTATE_180_BIT_KHR: VkSurfaceTransformFlagBitsKHR = VkSurfaceTransformFlagBitsKHR(4);
    pub const ROTATE_270_BIT_KHR: VkSurfaceTransformFlagBitsKHR = VkSurfaceTransformFlagBitsKHR(8);
    pub const HORIZONTAL_MIRROR_BIT_KHR: VkSurfaceTransformFlagBitsKHR = VkSurfaceTransformFlagBitsKHR(16);
    pub const HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR: VkSurfaceTransformFlagBitsKHR = VkSurfaceTransformFlagBitsKHR(32);
    pub const HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR: VkSurfaceTransformFlagBitsKHR = VkSurfaceTransformFlagBitsKHR(64);
    pub const HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR: VkSurfaceTransformFlagBitsKHR = VkSurfaceTransformFlagBitsKHR(128);
    pub const INHERIT_BIT_KHR: VkSurfaceTransformFlagBitsKHR = VkSurfaceTransformFlagBitsKHR(256);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkSurfaceTransformFlagBitsKHR {
    type Output = VkSurfaceTransformFlagBitsKHR;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkSurfaceTransformFlagBitsKHR {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkSurfaceTransformFlagBitsKHR {
    type Output = VkSurfaceTransformFlagBitsKHR;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkSurfaceTransformFlagBitsKHR {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkSurfaceTransformFlagBitsKHR {
    type Output = VkSurfaceTransformFlagBitsKHR;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkSurfaceTransformFlagBitsKHR {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkSwapchainCreateFlagBitsKHR(VkFlags);
impl VkSwapchainCreateFlagBitsKHR {

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkSwapchainCreateFlagBitsKHR {
    type Output = VkSwapchainCreateFlagBitsKHR;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkSwapchainCreateFlagBitsKHR {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkSwapchainCreateFlagBitsKHR {
    type Output = VkSwapchainCreateFlagBitsKHR;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkSwapchainCreateFlagBitsKHR {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkSwapchainCreateFlagBitsKHR {
    type Output = VkSwapchainCreateFlagBitsKHR;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkSwapchainCreateFlagBitsKHR {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkDebugUtilsMessageTypeFlagBitsEXT(VkFlags);
impl VkDebugUtilsMessageTypeFlagBitsEXT {
    pub const GENERAL_BIT_EXT: VkDebugUtilsMessageTypeFlagBitsEXT = VkDebugUtilsMessageTypeFlagBitsEXT(1);
    pub const VALIDATION_BIT_EXT: VkDebugUtilsMessageTypeFlagBitsEXT = VkDebugUtilsMessageTypeFlagBitsEXT(2);
    pub const PERFORMANCE_BIT_EXT: VkDebugUtilsMessageTypeFlagBitsEXT = VkDebugUtilsMessageTypeFlagBitsEXT(4);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkDebugUtilsMessageTypeFlagBitsEXT {
    type Output = VkDebugUtilsMessageTypeFlagBitsEXT;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkDebugUtilsMessageTypeFlagBitsEXT {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkDebugUtilsMessageTypeFlagBitsEXT {
    type Output = VkDebugUtilsMessageTypeFlagBitsEXT;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkDebugUtilsMessageTypeFlagBitsEXT {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkDebugUtilsMessageTypeFlagBitsEXT {
    type Output = VkDebugUtilsMessageTypeFlagBitsEXT;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkDebugUtilsMessageTypeFlagBitsEXT {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkDebugUtilsMessageSeverityFlagBitsEXT(VkFlags);
impl VkDebugUtilsMessageSeverityFlagBitsEXT {
    pub const VERBOSE_BIT_EXT: VkDebugUtilsMessageSeverityFlagBitsEXT = VkDebugUtilsMessageSeverityFlagBitsEXT(1);
    pub const INFO_BIT_EXT: VkDebugUtilsMessageSeverityFlagBitsEXT = VkDebugUtilsMessageSeverityFlagBitsEXT(16);
    pub const WARNING_BIT_EXT: VkDebugUtilsMessageSeverityFlagBitsEXT = VkDebugUtilsMessageSeverityFlagBitsEXT(256);
    pub const ERROR_BIT_EXT: VkDebugUtilsMessageSeverityFlagBitsEXT = VkDebugUtilsMessageSeverityFlagBitsEXT(4096);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkDebugUtilsMessageSeverityFlagBitsEXT {
    type Output = VkDebugUtilsMessageSeverityFlagBitsEXT;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkDebugUtilsMessageSeverityFlagBitsEXT {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkDebugUtilsMessageSeverityFlagBitsEXT {
    type Output = VkDebugUtilsMessageSeverityFlagBitsEXT;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkDebugUtilsMessageSeverityFlagBitsEXT {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkDebugUtilsMessageSeverityFlagBitsEXT {
    type Output = VkDebugUtilsMessageSeverityFlagBitsEXT;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkDebugUtilsMessageSeverityFlagBitsEXT {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkAccessFlagBits(VkFlags);
impl VkAccessFlagBits {
    pub const INDIRECT_COMMAND_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(1);
    pub const INDEX_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(2);
    pub const VERTEX_ATTRIBUTE_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(4);
    pub const UNIFORM_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(8);
    pub const INPUT_ATTACHMENT_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(16);
    pub const SHADER_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(32);
    pub const SHADER_WRITE_BIT: VkAccessFlagBits = VkAccessFlagBits(64);
    pub const COLOR_ATTACHMENT_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(128);
    pub const COLOR_ATTACHMENT_WRITE_BIT: VkAccessFlagBits = VkAccessFlagBits(256);
    pub const DEPTH_STENCIL_ATTACHMENT_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(512);
    pub const DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: VkAccessFlagBits = VkAccessFlagBits(1024);
    pub const TRANSFER_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(2048);
    pub const TRANSFER_WRITE_BIT: VkAccessFlagBits = VkAccessFlagBits(4096);
    pub const HOST_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(8192);
    pub const HOST_WRITE_BIT: VkAccessFlagBits = VkAccessFlagBits(16384);
    pub const MEMORY_READ_BIT: VkAccessFlagBits = VkAccessFlagBits(32768);
    pub const MEMORY_WRITE_BIT: VkAccessFlagBits = VkAccessFlagBits(65536);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkAccessFlagBits {
    type Output = VkAccessFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkAccessFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkAccessFlagBits {
    type Output = VkAccessFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkAccessFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkAccessFlagBits {
    type Output = VkAccessFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkAccessFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkImageAspectFlagBits(VkFlags);
impl VkImageAspectFlagBits {
    pub const COLOR_BIT: VkImageAspectFlagBits = VkImageAspectFlagBits(1);
    pub const DEPTH_BIT: VkImageAspectFlagBits = VkImageAspectFlagBits(2);
    pub const STENCIL_BIT: VkImageAspectFlagBits = VkImageAspectFlagBits(4);
    pub const METADATA_BIT: VkImageAspectFlagBits = VkImageAspectFlagBits(8);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkImageAspectFlagBits {
    type Output = VkImageAspectFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkImageAspectFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkImageAspectFlagBits {
    type Output = VkImageAspectFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkImageAspectFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkImageAspectFlagBits {
    type Output = VkImageAspectFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkImageAspectFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkShaderStageFlagBits(VkFlags);
impl VkShaderStageFlagBits {
    pub const VERTEX_BIT: VkShaderStageFlagBits = VkShaderStageFlagBits(1);
    pub const TESSELLATION_CONTROL_BIT: VkShaderStageFlagBits = VkShaderStageFlagBits(2);
    pub const TESSELLATION_EVALUATION_BIT: VkShaderStageFlagBits = VkShaderStageFlagBits(4);
    pub const GEOMETRY_BIT: VkShaderStageFlagBits = VkShaderStageFlagBits(8);
    pub const FRAGMENT_BIT: VkShaderStageFlagBits = VkShaderStageFlagBits(16);
    pub const ALL_GRAPHICS: VkShaderStageFlagBits = VkShaderStageFlagBits(31);
    pub const COMPUTE_BIT: VkShaderStageFlagBits = VkShaderStageFlagBits(32);
    pub const ALL: VkShaderStageFlagBits = VkShaderStageFlagBits(2147483647);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkShaderStageFlagBits {
    type Output = VkShaderStageFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkShaderStageFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkShaderStageFlagBits {
    type Output = VkShaderStageFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkShaderStageFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkShaderStageFlagBits {
    type Output = VkShaderStageFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkShaderStageFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkQueryResultFlagBits(VkFlags);
impl VkQueryResultFlagBits {
    pub const K_64_BIT: VkQueryResultFlagBits = VkQueryResultFlagBits(1);
    pub const WAIT_BIT: VkQueryResultFlagBits = VkQueryResultFlagBits(2);
    pub const WITH_AVAILABILITY_BIT: VkQueryResultFlagBits = VkQueryResultFlagBits(4);
    pub const PARTIAL_BIT: VkQueryResultFlagBits = VkQueryResultFlagBits(8);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkQueryResultFlagBits {
    type Output = VkQueryResultFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkQueryResultFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkQueryResultFlagBits {
    type Output = VkQueryResultFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkQueryResultFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkQueryResultFlagBits {
    type Output = VkQueryResultFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkQueryResultFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkPipelineStageFlagBits(VkFlags);
impl VkPipelineStageFlagBits {
    pub const TOP_OF_PIPE_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1);
    pub const DRAW_INDIRECT_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(2);
    pub const VERTEX_INPUT_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(4);
    pub const VERTEX_SHADER_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(8);
    pub const TESSELLATION_CONTROL_SHADER_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(16);
    pub const TESSELLATION_EVALUATION_SHADER_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(32);
    pub const GEOMETRY_SHADER_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(64);
    pub const FRAGMENT_SHADER_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(128);
    pub const EARLY_FRAGMENT_TESTS_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(256);
    pub const LATE_FRAGMENT_TESTS_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(512);
    pub const COLOR_ATTACHMENT_OUTPUT_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(1024);
    pub const COMPUTE_SHADER_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(2048);
    pub const TRANSFER_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(4096);
    pub const BOTTOM_OF_PIPE_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(8192);
    pub const HOST_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(16384);
    pub const ALL_GRAPHICS_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(32768);
    pub const ALL_COMMANDS_BIT: VkPipelineStageFlagBits = VkPipelineStageFlagBits(65536);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkPipelineStageFlagBits {
    type Output = VkPipelineStageFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkPipelineStageFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkPipelineStageFlagBits {
    type Output = VkPipelineStageFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkPipelineStageFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkPipelineStageFlagBits {
    type Output = VkPipelineStageFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkPipelineStageFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkQueryControlFlagBits(VkFlags);
impl VkQueryControlFlagBits {
    pub const PRECISE_BIT: VkQueryControlFlagBits = VkQueryControlFlagBits(1);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkQueryControlFlagBits {
    type Output = VkQueryControlFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkQueryControlFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkQueryControlFlagBits {
    type Output = VkQueryControlFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkQueryControlFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkQueryControlFlagBits {
    type Output = VkQueryControlFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkQueryControlFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkDependencyFlagBits(VkFlags);
impl VkDependencyFlagBits {
    pub const BY_REGION_BIT: VkDependencyFlagBits = VkDependencyFlagBits(1);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkDependencyFlagBits {
    type Output = VkDependencyFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkDependencyFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkDependencyFlagBits {
    type Output = VkDependencyFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkDependencyFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkDependencyFlagBits {
    type Output = VkDependencyFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkDependencyFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkStencilFaceFlagBits(VkFlags);
impl VkStencilFaceFlagBits {
    pub const FRONT_BIT: VkStencilFaceFlagBits = VkStencilFaceFlagBits(1);
    pub const BACK_BIT: VkStencilFaceFlagBits = VkStencilFaceFlagBits(2);
    pub const FRONT_AND_BACK: VkStencilFaceFlagBits = VkStencilFaceFlagBits(3);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkStencilFaceFlagBits {
    type Output = VkStencilFaceFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkStencilFaceFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkStencilFaceFlagBits {
    type Output = VkStencilFaceFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkStencilFaceFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkStencilFaceFlagBits {
    type Output = VkStencilFaceFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkStencilFaceFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkCommandBufferResetFlagBits(VkFlags);
impl VkCommandBufferResetFlagBits {
    pub const RELEASE_RESOURCES_BIT: VkCommandBufferResetFlagBits = VkCommandBufferResetFlagBits(1);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkCommandBufferResetFlagBits {
    type Output = VkCommandBufferResetFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkCommandBufferResetFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkCommandBufferResetFlagBits {
    type Output = VkCommandBufferResetFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkCommandBufferResetFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkCommandBufferResetFlagBits {
    type Output = VkCommandBufferResetFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkCommandBufferResetFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkQueryPipelineStatisticFlagBits(VkFlags);
impl VkQueryPipelineStatisticFlagBits {
    pub const INPUT_ASSEMBLY_VERTICES_BIT: VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1);
    pub const INPUT_ASSEMBLY_PRIMITIVES_BIT: VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(2);
    pub const VERTEX_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(4);
    pub const GEOMETRY_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(8);
    pub const GEOMETRY_SHADER_PRIMITIVES_BIT: VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(16);
    pub const CLIPPING_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(32);
    pub const CLIPPING_PRIMITIVES_BIT: VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(64);
    pub const FRAGMENT_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(128);
    pub const TESSELLATION_CONTROL_SHADER_PATCHES_BIT: VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(256);
    pub const TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(512);
    pub const COMPUTE_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlagBits(1024);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkQueryPipelineStatisticFlagBits {
    type Output = VkQueryPipelineStatisticFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkQueryPipelineStatisticFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkQueryPipelineStatisticFlagBits {
    type Output = VkQueryPipelineStatisticFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkQueryPipelineStatisticFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkQueryPipelineStatisticFlagBits {
    type Output = VkQueryPipelineStatisticFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkQueryPipelineStatisticFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkCommandBufferUsageFlagBits(VkFlags);
impl VkCommandBufferUsageFlagBits {
    pub const ONE_TIME_SUBMIT_BIT: VkCommandBufferUsageFlagBits = VkCommandBufferUsageFlagBits(1);
    pub const RENDER_PASS_CONTINUE_BIT: VkCommandBufferUsageFlagBits = VkCommandBufferUsageFlagBits(2);
    pub const SIMULTANEOUS_USE_BIT: VkCommandBufferUsageFlagBits = VkCommandBufferUsageFlagBits(4);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkCommandBufferUsageFlagBits {
    type Output = VkCommandBufferUsageFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkCommandBufferUsageFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkCommandBufferUsageFlagBits {
    type Output = VkCommandBufferUsageFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkCommandBufferUsageFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkCommandBufferUsageFlagBits {
    type Output = VkCommandBufferUsageFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkCommandBufferUsageFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkCommandPoolResetFlagBits(VkFlags);
impl VkCommandPoolResetFlagBits {
    pub const RELEASE_RESOURCES_BIT: VkCommandPoolResetFlagBits = VkCommandPoolResetFlagBits(1);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkCommandPoolResetFlagBits {
    type Output = VkCommandPoolResetFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkCommandPoolResetFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkCommandPoolResetFlagBits {
    type Output = VkCommandPoolResetFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkCommandPoolResetFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkCommandPoolResetFlagBits {
    type Output = VkCommandPoolResetFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkCommandPoolResetFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkCommandPoolCreateFlagBits(VkFlags);
impl VkCommandPoolCreateFlagBits {
    pub const TRANSIENT_BIT: VkCommandPoolCreateFlagBits = VkCommandPoolCreateFlagBits(1);
    pub const RESET_COMMAND_BUFFER_BIT: VkCommandPoolCreateFlagBits = VkCommandPoolCreateFlagBits(2);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkCommandPoolCreateFlagBits {
    type Output = VkCommandPoolCreateFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkCommandPoolCreateFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkCommandPoolCreateFlagBits {
    type Output = VkCommandPoolCreateFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkCommandPoolCreateFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkCommandPoolCreateFlagBits {
    type Output = VkCommandPoolCreateFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkCommandPoolCreateFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkSubpassDescriptionFlagBits(VkFlags);
impl VkSubpassDescriptionFlagBits {

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkSubpassDescriptionFlagBits {
    type Output = VkSubpassDescriptionFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkSubpassDescriptionFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkSubpassDescriptionFlagBits {
    type Output = VkSubpassDescriptionFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkSubpassDescriptionFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkSubpassDescriptionFlagBits {
    type Output = VkSubpassDescriptionFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkSubpassDescriptionFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkSampleCountFlagBits(VkFlags);
impl VkSampleCountFlagBits {
    pub const K_1_BIT: VkSampleCountFlagBits = VkSampleCountFlagBits(1);
    pub const K_2_BIT: VkSampleCountFlagBits = VkSampleCountFlagBits(2);
    pub const K_4_BIT: VkSampleCountFlagBits = VkSampleCountFlagBits(4);
    pub const K_8_BIT: VkSampleCountFlagBits = VkSampleCountFlagBits(8);
    pub const K_16_BIT: VkSampleCountFlagBits = VkSampleCountFlagBits(16);
    pub const K_32_BIT: VkSampleCountFlagBits = VkSampleCountFlagBits(32);
    pub const K_64_BIT: VkSampleCountFlagBits = VkSampleCountFlagBits(64);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkSampleCountFlagBits {
    type Output = VkSampleCountFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkSampleCountFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkSampleCountFlagBits {
    type Output = VkSampleCountFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkSampleCountFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkSampleCountFlagBits {
    type Output = VkSampleCountFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkSampleCountFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkAttachmentDescriptionFlagBits(VkFlags);
impl VkAttachmentDescriptionFlagBits {
    pub const MAY_ALIAS_BIT: VkAttachmentDescriptionFlagBits = VkAttachmentDescriptionFlagBits(1);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkAttachmentDescriptionFlagBits {
    type Output = VkAttachmentDescriptionFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkAttachmentDescriptionFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkAttachmentDescriptionFlagBits {
    type Output = VkAttachmentDescriptionFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkAttachmentDescriptionFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkAttachmentDescriptionFlagBits {
    type Output = VkAttachmentDescriptionFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkAttachmentDescriptionFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkDescriptorPoolCreateFlagBits(VkFlags);
impl VkDescriptorPoolCreateFlagBits {
    pub const FREE_DESCRIPTOR_SET_BIT: VkDescriptorPoolCreateFlagBits = VkDescriptorPoolCreateFlagBits(1);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkDescriptorPoolCreateFlagBits {
    type Output = VkDescriptorPoolCreateFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkDescriptorPoolCreateFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkDescriptorPoolCreateFlagBits {
    type Output = VkDescriptorPoolCreateFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkDescriptorPoolCreateFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkDescriptorPoolCreateFlagBits {
    type Output = VkDescriptorPoolCreateFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkDescriptorPoolCreateFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkDescriptorSetLayoutCreateFlagBits(VkFlags);
impl VkDescriptorSetLayoutCreateFlagBits {

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkDescriptorSetLayoutCreateFlagBits {
    type Output = VkDescriptorSetLayoutCreateFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkDescriptorSetLayoutCreateFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkDescriptorSetLayoutCreateFlagBits {
    type Output = VkDescriptorSetLayoutCreateFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkDescriptorSetLayoutCreateFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkDescriptorSetLayoutCreateFlagBits {
    type Output = VkDescriptorSetLayoutCreateFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkDescriptorSetLayoutCreateFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkSamplerCreateFlagBits(VkFlags);
impl VkSamplerCreateFlagBits {

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkSamplerCreateFlagBits {
    type Output = VkSamplerCreateFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkSamplerCreateFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkSamplerCreateFlagBits {
    type Output = VkSamplerCreateFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkSamplerCreateFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkSamplerCreateFlagBits {
    type Output = VkSamplerCreateFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkSamplerCreateFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkPipelineCreateFlagBits(VkFlags);
impl VkPipelineCreateFlagBits {
    pub const DISABLE_OPTIMIZATION_BIT: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(1);
    pub const ALLOW_DERIVATIVES_BIT: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(2);
    pub const DERIVATIVE_BIT: VkPipelineCreateFlagBits = VkPipelineCreateFlagBits(4);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkPipelineCreateFlagBits {
    type Output = VkPipelineCreateFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkPipelineCreateFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkPipelineCreateFlagBits {
    type Output = VkPipelineCreateFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkPipelineCreateFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkPipelineCreateFlagBits {
    type Output = VkPipelineCreateFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkPipelineCreateFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkColorComponentFlagBits(VkFlags);
impl VkColorComponentFlagBits {
    pub const R_BIT: VkColorComponentFlagBits = VkColorComponentFlagBits(1);
    pub const G_BIT: VkColorComponentFlagBits = VkColorComponentFlagBits(2);
    pub const B_BIT: VkColorComponentFlagBits = VkColorComponentFlagBits(4);
    pub const A_BIT: VkColorComponentFlagBits = VkColorComponentFlagBits(8);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkColorComponentFlagBits {
    type Output = VkColorComponentFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkColorComponentFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkColorComponentFlagBits {
    type Output = VkColorComponentFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkColorComponentFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkColorComponentFlagBits {
    type Output = VkColorComponentFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkColorComponentFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkCullModeFlagBits(VkFlags);
impl VkCullModeFlagBits {
    pub const NONE: VkCullModeFlagBits = VkCullModeFlagBits(0);
    pub const FRONT_BIT: VkCullModeFlagBits = VkCullModeFlagBits(1);
    pub const BACK_BIT: VkCullModeFlagBits = VkCullModeFlagBits(2);
    pub const FRONT_AND_BACK: VkCullModeFlagBits = VkCullModeFlagBits(3);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkCullModeFlagBits {
    type Output = VkCullModeFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkCullModeFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkCullModeFlagBits {
    type Output = VkCullModeFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkCullModeFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkCullModeFlagBits {
    type Output = VkCullModeFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkCullModeFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkImageViewCreateFlagBits(VkFlags);
impl VkImageViewCreateFlagBits {

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkImageViewCreateFlagBits {
    type Output = VkImageViewCreateFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkImageViewCreateFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkImageViewCreateFlagBits {
    type Output = VkImageViewCreateFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkImageViewCreateFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkImageViewCreateFlagBits {
    type Output = VkImageViewCreateFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkImageViewCreateFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkImageCreateFlagBits(VkFlags);
impl VkImageCreateFlagBits {
    pub const SPARSE_BINDING_BIT: VkImageCreateFlagBits = VkImageCreateFlagBits(1);
    pub const SPARSE_RESIDENCY_BIT: VkImageCreateFlagBits = VkImageCreateFlagBits(2);
    pub const SPARSE_ALIASED_BIT: VkImageCreateFlagBits = VkImageCreateFlagBits(4);
    pub const MUTABLE_FORMAT_BIT: VkImageCreateFlagBits = VkImageCreateFlagBits(8);
    pub const CUBE_COMPATIBLE_BIT: VkImageCreateFlagBits = VkImageCreateFlagBits(16);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkImageCreateFlagBits {
    type Output = VkImageCreateFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkImageCreateFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkImageCreateFlagBits {
    type Output = VkImageCreateFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkImageCreateFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkImageCreateFlagBits {
    type Output = VkImageCreateFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkImageCreateFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkBufferUsageFlagBits(VkFlags);
impl VkBufferUsageFlagBits {
    pub const TRANSFER_SRC_BIT: VkBufferUsageFlagBits = VkBufferUsageFlagBits(1);
    pub const TRANSFER_DST_BIT: VkBufferUsageFlagBits = VkBufferUsageFlagBits(2);
    pub const UNIFORM_TEXEL_BUFFER_BIT: VkBufferUsageFlagBits = VkBufferUsageFlagBits(4);
    pub const STORAGE_TEXEL_BUFFER_BIT: VkBufferUsageFlagBits = VkBufferUsageFlagBits(8);
    pub const UNIFORM_BUFFER_BIT: VkBufferUsageFlagBits = VkBufferUsageFlagBits(16);
    pub const STORAGE_BUFFER_BIT: VkBufferUsageFlagBits = VkBufferUsageFlagBits(32);
    pub const INDEX_BUFFER_BIT: VkBufferUsageFlagBits = VkBufferUsageFlagBits(64);
    pub const VERTEX_BUFFER_BIT: VkBufferUsageFlagBits = VkBufferUsageFlagBits(128);
    pub const INDIRECT_BUFFER_BIT: VkBufferUsageFlagBits = VkBufferUsageFlagBits(256);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkBufferUsageFlagBits {
    type Output = VkBufferUsageFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkBufferUsageFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkBufferUsageFlagBits {
    type Output = VkBufferUsageFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkBufferUsageFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkBufferUsageFlagBits {
    type Output = VkBufferUsageFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkBufferUsageFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkBufferCreateFlagBits(VkFlags);
impl VkBufferCreateFlagBits {
    pub const SPARSE_BINDING_BIT: VkBufferCreateFlagBits = VkBufferCreateFlagBits(1);
    pub const SPARSE_RESIDENCY_BIT: VkBufferCreateFlagBits = VkBufferCreateFlagBits(2);
    pub const SPARSE_ALIASED_BIT: VkBufferCreateFlagBits = VkBufferCreateFlagBits(4);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkBufferCreateFlagBits {
    type Output = VkBufferCreateFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkBufferCreateFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkBufferCreateFlagBits {
    type Output = VkBufferCreateFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkBufferCreateFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkBufferCreateFlagBits {
    type Output = VkBufferCreateFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkBufferCreateFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkFenceCreateFlagBits(VkFlags);
impl VkFenceCreateFlagBits {
    pub const SIGNALED_BIT: VkFenceCreateFlagBits = VkFenceCreateFlagBits(1);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkFenceCreateFlagBits {
    type Output = VkFenceCreateFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkFenceCreateFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkFenceCreateFlagBits {
    type Output = VkFenceCreateFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkFenceCreateFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkFenceCreateFlagBits {
    type Output = VkFenceCreateFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkFenceCreateFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkSparseMemoryBindFlagBits(VkFlags);
impl VkSparseMemoryBindFlagBits {
    pub const METADATA_BIT: VkSparseMemoryBindFlagBits = VkSparseMemoryBindFlagBits(1);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkSparseMemoryBindFlagBits {
    type Output = VkSparseMemoryBindFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkSparseMemoryBindFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkSparseMemoryBindFlagBits {
    type Output = VkSparseMemoryBindFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkSparseMemoryBindFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkSparseMemoryBindFlagBits {
    type Output = VkSparseMemoryBindFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkSparseMemoryBindFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkSparseImageFormatFlagBits(VkFlags);
impl VkSparseImageFormatFlagBits {
    pub const SINGLE_MIPTAIL_BIT: VkSparseImageFormatFlagBits = VkSparseImageFormatFlagBits(1);
    pub const ALIGNED_MIP_SIZE_BIT: VkSparseImageFormatFlagBits = VkSparseImageFormatFlagBits(2);
    pub const NONSTANDARD_BLOCK_SIZE_BIT: VkSparseImageFormatFlagBits = VkSparseImageFormatFlagBits(4);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkSparseImageFormatFlagBits {
    type Output = VkSparseImageFormatFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkSparseImageFormatFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkSparseImageFormatFlagBits {
    type Output = VkSparseImageFormatFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkSparseImageFormatFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkSparseImageFormatFlagBits {
    type Output = VkSparseImageFormatFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkSparseImageFormatFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkDeviceQueueCreateFlagBits(VkFlags);
impl VkDeviceQueueCreateFlagBits {

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkDeviceQueueCreateFlagBits {
    type Output = VkDeviceQueueCreateFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkDeviceQueueCreateFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkDeviceQueueCreateFlagBits {
    type Output = VkDeviceQueueCreateFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkDeviceQueueCreateFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkDeviceQueueCreateFlagBits {
    type Output = VkDeviceQueueCreateFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkDeviceQueueCreateFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkMemoryHeapFlagBits(VkFlags);
impl VkMemoryHeapFlagBits {
    pub const DEVICE_LOCAL_BIT: VkMemoryHeapFlagBits = VkMemoryHeapFlagBits(1);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkMemoryHeapFlagBits {
    type Output = VkMemoryHeapFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkMemoryHeapFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkMemoryHeapFlagBits {
    type Output = VkMemoryHeapFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkMemoryHeapFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkMemoryHeapFlagBits {
    type Output = VkMemoryHeapFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkMemoryHeapFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkMemoryPropertyFlagBits(VkFlags);
impl VkMemoryPropertyFlagBits {
    pub const DEVICE_LOCAL_BIT: VkMemoryPropertyFlagBits = VkMemoryPropertyFlagBits(1);
    pub const HOST_VISIBLE_BIT: VkMemoryPropertyFlagBits = VkMemoryPropertyFlagBits(2);
    pub const HOST_COHERENT_BIT: VkMemoryPropertyFlagBits = VkMemoryPropertyFlagBits(4);
    pub const HOST_CACHED_BIT: VkMemoryPropertyFlagBits = VkMemoryPropertyFlagBits(8);
    pub const LAZILY_ALLOCATED_BIT: VkMemoryPropertyFlagBits = VkMemoryPropertyFlagBits(16);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkMemoryPropertyFlagBits {
    type Output = VkMemoryPropertyFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkMemoryPropertyFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkMemoryPropertyFlagBits {
    type Output = VkMemoryPropertyFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkMemoryPropertyFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkMemoryPropertyFlagBits {
    type Output = VkMemoryPropertyFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkMemoryPropertyFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkQueueFlagBits(VkFlags);
impl VkQueueFlagBits {
    pub const GRAPHICS_BIT: VkQueueFlagBits = VkQueueFlagBits(1);
    pub const COMPUTE_BIT: VkQueueFlagBits = VkQueueFlagBits(2);
    pub const TRANSFER_BIT: VkQueueFlagBits = VkQueueFlagBits(4);
    pub const SPARSE_BINDING_BIT: VkQueueFlagBits = VkQueueFlagBits(8);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkQueueFlagBits {
    type Output = VkQueueFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkQueueFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkQueueFlagBits {
    type Output = VkQueueFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkQueueFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkQueueFlagBits {
    type Output = VkQueueFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkQueueFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VkFormatFeatureFlagBits(VkFlags);
impl VkFormatFeatureFlagBits {
    pub const SAMPLED_IMAGE_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1);
    pub const STORAGE_IMAGE_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(2);
    pub const STORAGE_IMAGE_ATOMIC_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(4);
    pub const UNIFORM_TEXEL_BUFFER_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(8);
    pub const STORAGE_TEXEL_BUFFER_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(16);
    pub const STORAGE_TEXEL_BUFFER_ATOMIC_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(32);
    pub const VERTEX_BUFFER_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(64);
    pub const COLOR_ATTACHMENT_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(128);
    pub const COLOR_ATTACHMENT_BLEND_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(256);
    pub const DEPTH_STENCIL_ATTACHMENT_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(512);
    pub const BLIT_SRC_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(1024);
    pub const BLIT_DST_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(2048);
    pub const SAMPLED_IMAGE_FILTER_LINEAR_BIT: VkFormatFeatureFlagBits = VkFormatFeatureFlagBits(4096);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for VkFormatFeatureFlagBits {
    type Output = VkFormatFeatureFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for VkFormatFeatureFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for VkFormatFeatureFlagBits {
    type Output = VkFormatFeatureFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for VkFormatFeatureFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for VkFormatFeatureFlagBits {
    type Output = VkFormatFeatureFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for VkFormatFeatureFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAllocationCallbacks {
    pub p_user_data: *mut core::ffi::c_void,
    pub pfn_allocation: PfnVkAllocationFunction,
    pub pfn_reallocation: PfnVkReallocationFunction,
    pub pfn_free: PfnVkFreeFunction,
    pub pfn_internal_allocation: PfnVkInternalAllocationNotification,
    pub pfn_internal_free: PfnVkInternalFreeNotification,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkWin32SurfaceCreateInfoKHR {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkWin32SurfaceCreateFlagsKHR,
    pub hinstance: HINSTANCE,
    pub hwnd: HWND,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSurfaceFormatKHR {
    pub format: VkFormat,
    pub color_space: VkColorSpaceKHR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSurfaceCapabilitiesKHR {
    pub min_image_count: u32,
    pub max_image_count: u32,
    pub current_extent: VkExtent2D,
    pub min_image_extent: VkExtent2D,
    pub max_image_extent: VkExtent2D,
    pub max_image_array_layers: u32,
    pub supported_transforms: VkSurfaceTransformFlagsKHR,
    pub current_transform: VkSurfaceTransformFlagBitsKHR,
    pub supported_composite_alpha: VkCompositeAlphaFlagsKHR,
    pub supported_usage_flags: VkImageUsageFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExtent2D {
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPresentInfoKHR {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphores: *const VkSemaphore,
    pub swapchain_count: u32,
    pub p_swapchains: *const VkSwapchainKHR,
    pub p_image_indices: *const u32,
    pub p_results: *mut VkResult,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSwapchainCreateInfoKHR {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkSwapchainCreateFlagsKHR,
    pub surface: VkSurfaceKHR,
    pub min_image_count: u32,
    pub image_format: VkFormat,
    pub image_color_space: VkColorSpaceKHR,
    pub image_extent: VkExtent2D,
    pub image_array_layers: u32,
    pub image_usage: VkImageUsageFlags,
    pub image_sharing_mode: VkSharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
    pub pre_transform: VkSurfaceTransformFlagBitsKHR,
    pub composite_alpha: VkCompositeAlphaFlagBitsKHR,
    pub present_mode: VkPresentModeKHR,
    pub clipped: VkBool32,
    pub old_swapchain: VkSwapchainKHR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDebugUtilsMessengerCallbackDataEXT {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkDebugUtilsMessengerCallbackDataFlagsEXT,
    pub p_message_id_name: *const u8,
    pub message_id_number: i32,
    pub p_message: *const u8,
    pub queue_label_count: u32,
    pub p_queue_labels: *const VkDebugUtilsLabelEXT,
    pub cmd_buf_label_count: u32,
    pub p_cmd_buf_labels: *const VkDebugUtilsLabelEXT,
    pub object_count: u32,
    pub p_objects: *const VkDebugUtilsObjectNameInfoEXT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDebugUtilsObjectNameInfoEXT {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub object_type: VkObjectType,
    pub object_handle: u64,
    pub p_object_name: *const u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDebugUtilsLabelEXT {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub p_label_name: *const u8,
    pub color: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDebugUtilsMessengerCreateInfoEXT {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkDebugUtilsMessengerCreateFlagsEXT,
    pub message_severity: VkDebugUtilsMessageSeverityFlagsEXT,
    pub message_type: VkDebugUtilsMessageTypeFlagsEXT,
    pub pfn_user_callback: PfnVkDebugUtilsMessengerCallbackEXT,
    pub p_user_data: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDebugUtilsObjectTagInfoEXT {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub object_type: VkObjectType,
    pub object_handle: u64,
    pub tag_name: u64,
    pub tag_size: usize,
    pub p_tag: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBaseInStructure {
    pub s_type: VkStructureType,
    pub p_next: *const VkBaseInStructure,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBaseOutStructure {
    pub s_type: VkStructureType,
    pub p_next: *mut VkBaseOutStructure,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryBarrier {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub src_access_mask: VkAccessFlags,
    pub dst_access_mask: VkAccessFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageMemoryBarrier {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub src_access_mask: VkAccessFlags,
    pub dst_access_mask: VkAccessFlags,
    pub old_layout: VkImageLayout,
    pub new_layout: VkImageLayout,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub image: VkImage,
    pub subresource_range: VkImageSubresourceRange,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageSubresourceRange {
    pub aspect_mask: VkImageAspectFlags,
    pub base_mip_level: u32,
    pub level_count: u32,
    pub base_array_layer: u32,
    pub layer_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDrawIndirectCommand {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub first_vertex: u32,
    pub first_instance: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDrawIndexedIndirectCommand {
    pub index_count: u32,
    pub instance_count: u32,
    pub first_index: u32,
    pub vertex_offset: i32,
    pub first_instance: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDispatchIndirectCommand {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferMemoryBarrier {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub src_access_mask: VkAccessFlags,
    pub dst_access_mask: VkAccessFlags,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRenderPassBeginInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub render_pass: VkRenderPass,
    pub framebuffer: VkFramebuffer,
    pub render_area: VkRect2D,
    pub clear_value_count: u32,
    pub p_clear_values: *const VkClearValue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkClearDepthStencilValue {
    pub depth: f32,
    pub stencil: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRect2D {
    pub offset: VkOffset2D,
    pub extent: VkExtent2D,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkOffset2D {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageResolve {
    pub src_subresource: VkImageSubresourceLayers,
    pub src_offset: VkOffset3D,
    pub dst_subresource: VkImageSubresourceLayers,
    pub dst_offset: VkOffset3D,
    pub extent: VkExtent3D,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExtent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkOffset3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageSubresourceLayers {
    pub aspect_mask: VkImageAspectFlags,
    pub mip_level: u32,
    pub base_array_layer: u32,
    pub layer_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkClearRect {
    pub rect: VkRect2D,
    pub base_array_layer: u32,
    pub layer_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkClearAttachment {
    pub aspect_mask: VkImageAspectFlags,
    pub color_attachment: u32,
    pub clear_value: VkClearValue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferImageCopy {
    pub buffer_offset: VkDeviceSize,
    pub buffer_row_length: u32,
    pub buffer_image_height: u32,
    pub image_subresource: VkImageSubresourceLayers,
    pub image_offset: VkOffset3D,
    pub image_extent: VkExtent3D,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageBlit {
    pub src_subresource: VkImageSubresourceLayers,
    pub src_offsets: [VkOffset3D; 2],
    pub dst_subresource: VkImageSubresourceLayers,
    pub dst_offsets: [VkOffset3D; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageCopy {
    pub src_subresource: VkImageSubresourceLayers,
    pub src_offset: VkOffset3D,
    pub dst_subresource: VkImageSubresourceLayers,
    pub dst_offset: VkOffset3D,
    pub extent: VkExtent3D,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferCopy {
    pub src_offset: VkDeviceSize,
    pub dst_offset: VkDeviceSize,
    pub size: VkDeviceSize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkViewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCommandBufferBeginInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkCommandBufferUsageFlags,
    pub p_inheritance_info: *const VkCommandBufferInheritanceInfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCommandBufferInheritanceInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub render_pass: VkRenderPass,
    pub subpass: u32,
    pub framebuffer: VkFramebuffer,
    pub occlusion_query_enable: VkBool32,
    pub query_flags: VkQueryControlFlags,
    pub pipeline_statistics: VkQueryPipelineStatisticFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCommandBufferAllocateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub command_pool: VkCommandPool,
    pub level: VkCommandBufferLevel,
    pub command_buffer_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCommandPoolCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkCommandPoolCreateFlags,
    pub queue_family_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRenderPassCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkRenderPassCreateFlags,
    pub attachment_count: u32,
    pub p_attachments: *const VkAttachmentDescription,
    pub subpass_count: u32,
    pub p_subpasses: *const VkSubpassDescription,
    pub dependency_count: u32,
    pub p_dependencies: *const VkSubpassDependency,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubpassDependency {
    pub src_subpass: u32,
    pub dst_subpass: u32,
    pub src_stage_mask: VkPipelineStageFlags,
    pub dst_stage_mask: VkPipelineStageFlags,
    pub src_access_mask: VkAccessFlags,
    pub dst_access_mask: VkAccessFlags,
    pub dependency_flags: VkDependencyFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubpassDescription {
    pub flags: VkSubpassDescriptionFlags,
    pub pipeline_bind_point: VkPipelineBindPoint,
    pub input_attachment_count: u32,
    pub p_input_attachments: *const VkAttachmentReference,
    pub color_attachment_count: u32,
    pub p_color_attachments: *const VkAttachmentReference,
    pub p_resolve_attachments: *const VkAttachmentReference,
    pub p_depth_stencil_attachment: *const VkAttachmentReference,
    pub preserve_attachment_count: u32,
    pub p_preserve_attachments: *const u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAttachmentReference {
    pub attachment: u32,
    pub layout: VkImageLayout,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAttachmentDescription {
    pub flags: VkAttachmentDescriptionFlags,
    pub format: VkFormat,
    pub samples: VkSampleCountFlagBits,
    pub load_op: VkAttachmentLoadOp,
    pub store_op: VkAttachmentStoreOp,
    pub stencil_load_op: VkAttachmentLoadOp,
    pub stencil_store_op: VkAttachmentStoreOp,
    pub initial_layout: VkImageLayout,
    pub final_layout: VkImageLayout,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkFramebufferCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkFramebufferCreateFlags,
    pub render_pass: VkRenderPass,
    pub attachment_count: u32,
    pub p_attachments: *const VkImageView,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCopyDescriptorSet {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub src_set: VkDescriptorSet,
    pub src_binding: u32,
    pub src_array_element: u32,
    pub dst_set: VkDescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkWriteDescriptorSet {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub dst_set: VkDescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
    pub descriptor_type: VkDescriptorType,
    pub p_image_info: *const VkDescriptorImageInfo,
    pub p_buffer_info: *const VkDescriptorBufferInfo,
    pub p_texel_buffer_view: *const VkBufferView,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorBufferInfo {
    pub buffer: VkBuffer,
    pub offset: VkDeviceSize,
    pub range: VkDeviceSize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorImageInfo {
    pub sampler: VkSampler,
    pub image_view: VkImageView,
    pub image_layout: VkImageLayout,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorSetAllocateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub descriptor_pool: VkDescriptorPool,
    pub descriptor_set_count: u32,
    pub p_set_layouts: *const VkDescriptorSetLayout,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorPoolCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkDescriptorPoolCreateFlags,
    pub max_sets: u32,
    pub pool_size_count: u32,
    pub p_pool_sizes: *const VkDescriptorPoolSize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorPoolSize {
    pub kind: VkDescriptorType,
    pub descriptor_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorSetLayoutCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkDescriptorSetLayoutCreateFlags,
    pub binding_count: u32,
    pub p_bindings: *const VkDescriptorSetLayoutBinding,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDescriptorSetLayoutBinding {
    pub binding: u32,
    pub descriptor_type: VkDescriptorType,
    pub descriptor_count: u32,
    pub stage_flags: VkShaderStageFlags,
    pub p_immutable_samplers: *const VkSampler,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSamplerCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkSamplerCreateFlags,
    pub mag_filter: VkFilter,
    pub min_filter: VkFilter,
    pub mipmap_mode: VkSamplerMipmapMode,
    pub address_mode_u: VkSamplerAddressMode,
    pub address_mode_v: VkSamplerAddressMode,
    pub address_mode_w: VkSamplerAddressMode,
    pub mip_lod_bias: f32,
    pub anisotropy_enable: VkBool32,
    pub max_anisotropy: f32,
    pub compare_enable: VkBool32,
    pub compare_op: VkCompareOp,
    pub min_lod: f32,
    pub max_lod: f32,
    pub border_color: VkBorderColor,
    pub unnormalized_coordinates: VkBool32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineLayoutCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkPipelineLayoutCreateFlags,
    pub set_layout_count: u32,
    pub p_set_layouts: *const VkDescriptorSetLayout,
    pub push_constant_range_count: u32,
    pub p_push_constant_ranges: *const VkPushConstantRange,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPushConstantRange {
    pub stage_flags: VkShaderStageFlags,
    pub offset: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkComputePipelineCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkPipelineCreateFlags,
    pub stage: VkPipelineShaderStageCreateInfo,
    pub layout: VkPipelineLayout,
    pub base_pipeline_handle: VkPipeline,
    pub base_pipeline_index: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineShaderStageCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkPipelineShaderStageCreateFlags,
    pub stage: VkShaderStageFlagBits,
    pub module: VkShaderModule,
    pub p_name: *const u8,
    pub p_specialization_info: *const VkSpecializationInfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSpecializationInfo {
    pub map_entry_count: u32,
    pub p_map_entries: *const VkSpecializationMapEntry,
    pub data_size: usize,
    pub p_data: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSpecializationMapEntry {
    pub ant_id: u32,
    pub offset: u32,
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkGraphicsPipelineCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkPipelineCreateFlags,
    pub stage_count: u32,
    pub p_stages: *const VkPipelineShaderStageCreateInfo,
    pub p_vertex_input_state: *const VkPipelineVertexInputStateCreateInfo,
    pub p_input_assembly_state: *const VkPipelineInputAssemblyStateCreateInfo,
    pub p_tessellation_state: *const VkPipelineTessellationStateCreateInfo,
    pub p_viewport_state: *const VkPipelineViewportStateCreateInfo,
    pub p_rasterization_state: *const VkPipelineRasterizationStateCreateInfo,
    pub p_multisample_state: *const VkPipelineMultisampleStateCreateInfo,
    pub p_depth_stencil_state: *const VkPipelineDepthStencilStateCreateInfo,
    pub p_color_blend_state: *const VkPipelineColorBlendStateCreateInfo,
    pub p_dynamic_state: *const VkPipelineDynamicStateCreateInfo,
    pub layout: VkPipelineLayout,
    pub render_pass: VkRenderPass,
    pub subpass: u32,
    pub base_pipeline_handle: VkPipeline,
    pub base_pipeline_index: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineDynamicStateCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkPipelineDynamicStateCreateFlags,
    pub dynamic_state_count: u32,
    pub p_dynamic_states: *const VkDynamicState,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineColorBlendStateCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkPipelineColorBlendStateCreateFlags,
    pub logic_op_enable: VkBool32,
    pub logic_op: VkLogicOp,
    pub attachment_count: u32,
    pub p_attachments: *const VkPipelineColorBlendAttachmentState,
    pub blend_constants: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineColorBlendAttachmentState {
    pub blend_enable: VkBool32,
    pub src_color_blend_factor: VkBlendFactor,
    pub dst_color_blend_factor: VkBlendFactor,
    pub color_blend_op: VkBlendOp,
    pub src_alpha_blend_factor: VkBlendFactor,
    pub dst_alpha_blend_factor: VkBlendFactor,
    pub alpha_blend_op: VkBlendOp,
    pub color_write_mask: VkColorComponentFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineDepthStencilStateCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkPipelineDepthStencilStateCreateFlags,
    pub depth_test_enable: VkBool32,
    pub depth_write_enable: VkBool32,
    pub depth_compare_op: VkCompareOp,
    pub depth_bounds_test_enable: VkBool32,
    pub stencil_test_enable: VkBool32,
    pub front: VkStencilOpState,
    pub back: VkStencilOpState,
    pub min_depth_bounds: f32,
    pub max_depth_bounds: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkStencilOpState {
    pub fail_op: VkStencilOp,
    pub pass_op: VkStencilOp,
    pub depth_fail_op: VkStencilOp,
    pub compare_op: VkCompareOp,
    pub compare_mask: u32,
    pub write_mask: u32,
    pub reference: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineMultisampleStateCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkPipelineMultisampleStateCreateFlags,
    pub rasterization_samples: VkSampleCountFlagBits,
    pub sample_shading_enable: VkBool32,
    pub min_sample_shading: f32,
    pub p_sample_mask: *const VkSampleMask,
    pub alpha_to_coverage_enable: VkBool32,
    pub alpha_to_one_enable: VkBool32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineRasterizationStateCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkPipelineRasterizationStateCreateFlags,
    pub depth_clamp_enable: VkBool32,
    pub rasterizer_discard_enable: VkBool32,
    pub polygon_mode: VkPolygonMode,
    pub cull_mode: VkCullModeFlags,
    pub front_face: VkFrontFace,
    pub depth_bias_enable: VkBool32,
    pub depth_bias_constant_factor: f32,
    pub depth_bias_clamp: f32,
    pub depth_bias_slope_factor: f32,
    pub line_width: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineViewportStateCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkPipelineViewportStateCreateFlags,
    pub viewport_count: u32,
    pub p_viewports: *const VkViewport,
    pub scissor_count: u32,
    pub p_scissors: *const VkRect2D,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineTessellationStateCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkPipelineTessellationStateCreateFlags,
    pub patch_control_points: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineInputAssemblyStateCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkPipelineInputAssemblyStateCreateFlags,
    pub topology: VkPrimitiveTopology,
    pub primitive_restart_enable: VkBool32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineVertexInputStateCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkPipelineVertexInputStateCreateFlags,
    pub vertex_binding_description_count: u32,
    pub p_vertex_binding_descriptions: *const VkVertexInputBindingDescription,
    pub vertex_attribute_description_count: u32,
    pub p_vertex_attribute_descriptions: *const VkVertexInputAttributeDescription,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkVertexInputAttributeDescription {
    pub location: u32,
    pub binding: u32,
    pub format: VkFormat,
    pub offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkVertexInputBindingDescription {
    pub binding: u32,
    pub stride: u32,
    pub input_rate: VkVertexInputRate,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineCacheCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkPipelineCacheCreateFlags,
    pub initial_data_size: usize,
    pub p_initial_data: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkShaderModuleCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkShaderModuleCreateFlags,
    pub code_size: usize,
    pub p_code: *const u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageViewCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkImageViewCreateFlags,
    pub image: VkImage,
    pub view_type: VkImageViewType,
    pub format: VkFormat,
    pub components: VkComponentMapping,
    pub subresource_range: VkImageSubresourceRange,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkComponentMapping {
    pub r: VkComponentSwizzle,
    pub g: VkComponentSwizzle,
    pub b: VkComponentSwizzle,
    pub a: VkComponentSwizzle,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubresourceLayout {
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
    pub row_pitch: VkDeviceSize,
    pub array_pitch: VkDeviceSize,
    pub depth_pitch: VkDeviceSize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageSubresource {
    pub aspect_mask: VkImageAspectFlags,
    pub mip_level: u32,
    pub array_layer: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkImageCreateFlags,
    pub image_type: VkImageType,
    pub format: VkFormat,
    pub extent: VkExtent3D,
    pub mip_levels: u32,
    pub array_layers: u32,
    pub samples: VkSampleCountFlagBits,
    pub tiling: VkImageTiling,
    pub usage: VkImageUsageFlags,
    pub sharing_mode: VkSharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
    pub initial_layout: VkImageLayout,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferViewCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkBufferViewCreateFlags,
    pub buffer: VkBuffer,
    pub format: VkFormat,
    pub offset: VkDeviceSize,
    pub range: VkDeviceSize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBufferCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkBufferCreateFlags,
    pub size: VkDeviceSize,
    pub usage: VkBufferUsageFlags,
    pub sharing_mode: VkSharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkQueryPoolCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkQueryPoolCreateFlags,
    pub query_type: VkQueryType,
    pub query_count: u32,
    pub pipeline_statistics: VkQueryPipelineStatisticFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkEventCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkEventCreateFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSemaphoreCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkSemaphoreCreateFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkFenceCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkFenceCreateFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkBindSparseInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphores: *const VkSemaphore,
    pub buffer_bind_count: u32,
    pub p_buffer_binds: *const VkSparseBufferMemoryBindInfo,
    pub image_opaque_bind_count: u32,
    pub p_image_opaque_binds: *const VkSparseImageOpaqueMemoryBindInfo,
    pub image_bind_count: u32,
    pub p_image_binds: *const VkSparseImageMemoryBindInfo,
    pub signal_semaphore_count: u32,
    pub p_signal_semaphores: *const VkSemaphore,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseImageMemoryBindInfo {
    pub image: VkImage,
    pub bind_count: u32,
    pub p_binds: *const VkSparseImageMemoryBind,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseImageMemoryBind {
    pub subresource: VkImageSubresource,
    pub offset: VkOffset3D,
    pub extent: VkExtent3D,
    pub memory: VkDeviceMemory,
    pub memory_offset: VkDeviceSize,
    pub flags: VkSparseMemoryBindFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseImageOpaqueMemoryBindInfo {
    pub image: VkImage,
    pub bind_count: u32,
    pub p_binds: *const VkSparseMemoryBind,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseMemoryBind {
    pub resource_offset: VkDeviceSize,
    pub size: VkDeviceSize,
    pub memory: VkDeviceMemory,
    pub memory_offset: VkDeviceSize,
    pub flags: VkSparseMemoryBindFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseBufferMemoryBindInfo {
    pub buffer: VkBuffer,
    pub bind_count: u32,
    pub p_binds: *const VkSparseMemoryBind,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseImageFormatProperties {
    pub aspect_mask: VkImageAspectFlags,
    pub image_granularity: VkExtent3D,
    pub flags: VkSparseImageFormatFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSparseImageMemoryRequirements {
    pub format_properties: VkSparseImageFormatProperties,
    pub image_mip_tail_first_lod: u32,
    pub image_mip_tail_size: VkDeviceSize,
    pub image_mip_tail_offset: VkDeviceSize,
    pub image_mip_tail_stride: VkDeviceSize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryRequirements {
    pub size: VkDeviceSize,
    pub alignment: VkDeviceSize,
    pub memory_type_bits: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMappedMemoryRange {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub memory: VkDeviceMemory,
    pub offset: VkDeviceSize,
    pub size: VkDeviceSize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryAllocateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub allocation_size: VkDeviceSize,
    pub memory_type_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSubmitInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphores: *const VkSemaphore,
    pub p_wait_dst_stage_mask: *const VkPipelineStageFlags,
    pub command_buffer_count: u32,
    pub p_command_buffers: *const VkCommandBuffer,
    pub signal_semaphore_count: u32,
    pub p_signal_semaphores: *const VkSemaphore,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkLayerProperties {
    pub layer_name: [u8; 256],
    pub spec_version: u32,
    pub implementation_version: u32,
    pub description: [u8; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkExtensionProperties {
    pub extension_name: [u8; 256],
    pub spec_version: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDeviceCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkDeviceCreateFlags,
    pub queue_create_info_count: u32,
    pub p_queue_create_infos: *const VkDeviceQueueCreateInfo,
    pub enabled_layer_count: u32,
    pub pp_enabled_layer_names: *const *const u8,
    pub enabled_extension_count: u32,
    pub pp_enabled_extension_names: *const *const u8,
    pub p_enabled_features: *const VkPhysicalDeviceFeatures,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceFeatures {
    pub robust_buffer_access: VkBool32,
    pub full_draw_index_uint_32: VkBool32,
    pub image_cube_array: VkBool32,
    pub independent_blend: VkBool32,
    pub geometry_shader: VkBool32,
    pub tessellation_shader: VkBool32,
    pub sample_rate_shading: VkBool32,
    pub dual_src_blend: VkBool32,
    pub logic_op: VkBool32,
    pub multi_draw_indirect: VkBool32,
    pub draw_indirect_first_instance: VkBool32,
    pub depth_clamp: VkBool32,
    pub depth_bias_clamp: VkBool32,
    pub fill_mode_non_solid: VkBool32,
    pub depth_bounds: VkBool32,
    pub wide_lines: VkBool32,
    pub large_points: VkBool32,
    pub alpha_to_one: VkBool32,
    pub multi_viewport: VkBool32,
    pub sampler_anisotropy: VkBool32,
    pub texture_compression_etc_2: VkBool32,
    pub texture_compression_astc_ldr: VkBool32,
    pub texture_compression_bc: VkBool32,
    pub occlusion_query_precise: VkBool32,
    pub pipeline_statistics_query: VkBool32,
    pub vertex_pipeline_stores_and_atomics: VkBool32,
    pub fragment_stores_and_atomics: VkBool32,
    pub shader_tessellation_and_geometry_point_size: VkBool32,
    pub shader_image_gather_extended: VkBool32,
    pub shader_storage_image_extended_formats: VkBool32,
    pub shader_storage_image_multisample: VkBool32,
    pub shader_storage_image_read_without_format: VkBool32,
    pub shader_storage_image_write_without_format: VkBool32,
    pub shader_uniform_buffer_array_dynamic_indexing: VkBool32,
    pub shader_sampled_image_array_dynamic_indexing: VkBool32,
    pub shader_storage_buffer_array_dynamic_indexing: VkBool32,
    pub shader_storage_image_array_dynamic_indexing: VkBool32,
    pub shader_clip_distance: VkBool32,
    pub shader_cull_distance: VkBool32,
    pub shader_float_64: VkBool32,
    pub shader_int_64: VkBool32,
    pub shader_int_16: VkBool32,
    pub shader_resource_residency: VkBool32,
    pub shader_resource_min_lod: VkBool32,
    pub sparse_binding: VkBool32,
    pub sparse_residency_buffer: VkBool32,
    pub sparse_residency_image_2_d: VkBool32,
    pub sparse_residency_image_3_d: VkBool32,
    pub sparse_residency_2_samples: VkBool32,
    pub sparse_residency_4_samples: VkBool32,
    pub sparse_residency_8_samples: VkBool32,
    pub sparse_residency_16_samples: VkBool32,
    pub sparse_residency_aliased: VkBool32,
    pub variable_multisample_rate: VkBool32,
    pub inherited_queries: VkBool32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkDeviceQueueCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkDeviceQueueCreateFlags,
    pub queue_family_index: u32,
    pub queue_count: u32,
    pub p_queue_priorities: *const f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceMemoryProperties {
    pub memory_type_count: u32,
    pub memory_types: [VkMemoryType; 32],
    pub memory_heap_count: u32,
    pub memory_heaps: [VkMemoryHeap; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryHeap {
    pub size: VkDeviceSize,
    pub flags: VkMemoryHeapFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkMemoryType {
    pub property_flags: VkMemoryPropertyFlags,
    pub heap_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkQueueFamilyProperties {
    pub queue_flags: VkQueueFlags,
    pub queue_count: u32,
    pub timestamp_valid_bits: u32,
    pub min_image_transfer_granularity: VkExtent3D,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceProperties {
    pub api_version: u32,
    pub driver_version: u32,
    pub vendor_id: u32,
    pub device_id: u32,
    pub device_type: VkPhysicalDeviceType,
    pub device_name: [u8; 256],
    pub pipeline_cache_uuid: [u8; 16],
    pub limits: VkPhysicalDeviceLimits,
    pub sparse_properties: VkPhysicalDeviceSparseProperties,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceSparseProperties {
    pub residency_standard_2_d_block_shape: VkBool32,
    pub residency_standard_2_d_multisample_block_shape: VkBool32,
    pub residency_standard_3_d_block_shape: VkBool32,
    pub residency_aligned_mip_size: VkBool32,
    pub residency_non_resident_strict: VkBool32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPhysicalDeviceLimits {
    pub max_image_dimension_1_d: u32,
    pub max_image_dimension_2_d: u32,
    pub max_image_dimension_3_d: u32,
    pub max_image_dimension_cube: u32,
    pub max_image_array_layers: u32,
    pub max_texel_buffer_elements: u32,
    pub max_uniform_buffer_range: u32,
    pub max_storage_buffer_range: u32,
    pub max_push_constants_size: u32,
    pub max_memory_allocation_count: u32,
    pub max_sampler_allocation_count: u32,
    pub buffer_image_granularity: VkDeviceSize,
    pub sparse_address_space_size: VkDeviceSize,
    pub max_bound_descriptor_sets: u32,
    pub max_per_stage_descriptor_samplers: u32,
    pub max_per_stage_descriptor_uniform_buffers: u32,
    pub max_per_stage_descriptor_storage_buffers: u32,
    pub max_per_stage_descriptor_sampled_images: u32,
    pub max_per_stage_descriptor_storage_images: u32,
    pub max_per_stage_descriptor_input_attachments: u32,
    pub max_per_stage_resources: u32,
    pub max_descriptor_set_samplers: u32,
    pub max_descriptor_set_uniform_buffers: u32,
    pub max_descriptor_set_uniform_buffers_dynamic: u32,
    pub max_descriptor_set_storage_buffers: u32,
    pub max_descriptor_set_storage_buffers_dynamic: u32,
    pub max_descriptor_set_sampled_images: u32,
    pub max_descriptor_set_storage_images: u32,
    pub max_descriptor_set_input_attachments: u32,
    pub max_vertex_input_attributes: u32,
    pub max_vertex_input_bindings: u32,
    pub max_vertex_input_attribute_offset: u32,
    pub max_vertex_input_binding_stride: u32,
    pub max_vertex_output_components: u32,
    pub max_tessellation_generation_level: u32,
    pub max_tessellation_patch_size: u32,
    pub max_tessellation_control_per_vertex_input_components: u32,
    pub max_tessellation_control_per_vertex_output_components: u32,
    pub max_tessellation_control_per_patch_output_components: u32,
    pub max_tessellation_control_total_output_components: u32,
    pub max_tessellation_evaluation_input_components: u32,
    pub max_tessellation_evaluation_output_components: u32,
    pub max_geometry_shader_invocations: u32,
    pub max_geometry_input_components: u32,
    pub max_geometry_output_components: u32,
    pub max_geometry_output_vertices: u32,
    pub max_geometry_total_output_components: u32,
    pub max_fragment_input_components: u32,
    pub max_fragment_output_attachments: u32,
    pub max_fragment_dual_src_attachments: u32,
    pub max_fragment_combined_output_resources: u32,
    pub max_compute_shared_memory_size: u32,
    pub max_compute_work_group_count: [u32; 3],
    pub max_compute_work_group_invocations: u32,
    pub max_compute_work_group_size: [u32; 3],
    pub sub_pixel_precision_bits: u32,
    pub sub_texel_precision_bits: u32,
    pub mipmap_precision_bits: u32,
    pub max_draw_indexed_index_value: u32,
    pub max_draw_indirect_count: u32,
    pub max_sampler_lod_bias: f32,
    pub max_sampler_anisotropy: f32,
    pub max_viewports: u32,
    pub max_viewport_dimensions: [u32; 2],
    pub viewport_bounds_range: [f32; 2],
    pub viewport_sub_pixel_bits: u32,
    pub min_memory_map_alignment: usize,
    pub min_texel_buffer_offset_alignment: VkDeviceSize,
    pub min_uniform_buffer_offset_alignment: VkDeviceSize,
    pub min_storage_buffer_offset_alignment: VkDeviceSize,
    pub min_texel_offset: i32,
    pub max_texel_offset: u32,
    pub min_texel_gather_offset: i32,
    pub max_texel_gather_offset: u32,
    pub min_interpolation_offset: f32,
    pub max_interpolation_offset: f32,
    pub sub_pixel_interpolation_offset_bits: u32,
    pub max_framebuffer_width: u32,
    pub max_framebuffer_height: u32,
    pub max_framebuffer_layers: u32,
    pub framebuffer_color_sample_counts: VkSampleCountFlags,
    pub framebuffer_depth_sample_counts: VkSampleCountFlags,
    pub framebuffer_stencil_sample_counts: VkSampleCountFlags,
    pub framebuffer_no_attachments_sample_counts: VkSampleCountFlags,
    pub max_color_attachments: u32,
    pub sampled_image_color_sample_counts: VkSampleCountFlags,
    pub sampled_image_integer_sample_counts: VkSampleCountFlags,
    pub sampled_image_depth_sample_counts: VkSampleCountFlags,
    pub sampled_image_stencil_sample_counts: VkSampleCountFlags,
    pub storage_image_sample_counts: VkSampleCountFlags,
    pub max_sample_mask_words: u32,
    pub timestamp_compute_and_graphics: VkBool32,
    pub timestamp_period: f32,
    pub max_clip_distances: u32,
    pub max_cull_distances: u32,
    pub max_combined_clip_and_cull_distances: u32,
    pub discrete_queue_priorities: u32,
    pub point_size_range: [f32; 2],
    pub line_width_range: [f32; 2],
    pub point_size_granularity: f32,
    pub line_width_granularity: f32,
    pub strict_lines: VkBool32,
    pub standard_sample_locations: VkBool32,
    pub optimal_buffer_copy_offset_alignment: VkDeviceSize,
    pub optimal_buffer_copy_row_pitch_alignment: VkDeviceSize,
    pub non_coherent_atom_size: VkDeviceSize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkImageFormatProperties {
    pub max_extent: VkExtent3D,
    pub max_mip_levels: u32,
    pub max_array_layers: u32,
    pub sample_counts: VkSampleCountFlags,
    pub max_resource_size: VkDeviceSize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkFormatProperties {
    pub linear_tiling_features: VkFormatFeatureFlags,
    pub optimal_tiling_features: VkFormatFeatureFlags,
    pub buffer_features: VkFormatFeatureFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkInstanceCreateInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: VkInstanceCreateFlags,
    pub p_application_info: *const VkApplicationInfo,
    pub enabled_layer_count: u32,
    pub pp_enabled_layer_names: *const *const u8,
    pub enabled_extension_count: u32,
    pub pp_enabled_extension_names: *const *const u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkApplicationInfo {
    pub s_type: VkStructureType,
    pub p_next: *const core::ffi::c_void,
    pub p_application_name: *const u8,
    pub application_version: u32,
    pub p_engine_name: *const u8,
    pub engine_version: u32,
    pub api_version: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union VkClearValue {
    pub color: VkClearColorValue,
    pub depth_stencil: VkClearDepthStencilValue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union VkClearColorValue {
    pub float_32: [f32; 4],
    pub int_32: [i32; 4],
    pub uint_32: [u32; 4],
}

pub type PfnVkInternalFreeNotification = extern "system" fn(
    p_user_data: *mut core::ffi::c_void,
    size: usize,
    allocation_type: VkInternalAllocationType,
    allocation_scope: VkSystemAllocationScope,
);

pub type PfnVkInternalAllocationNotification = extern "system" fn(
    p_user_data: *mut core::ffi::c_void,
    size: usize,
    allocation_type: VkInternalAllocationType,
    allocation_scope: VkSystemAllocationScope,
);

pub type PfnVkFreeFunction = extern "system" fn(
    p_user_data: *mut core::ffi::c_void,
    p_memory: *mut core::ffi::c_void,
);

pub type PfnVkReallocationFunction = extern "system" fn(
    p_user_data: *mut core::ffi::c_void,
    p_original: *mut core::ffi::c_void,
    size: usize,
    alignment: usize,
    allocation_scope: VkSystemAllocationScope,
) -> *mut core::ffi::c_void;

pub type PfnVkAllocationFunction = extern "system" fn(
    p_user_data: *mut core::ffi::c_void,
    size: usize,
    alignment: usize,
    allocation_scope: VkSystemAllocationScope,
) -> *mut core::ffi::c_void;

pub type PfnVkDebugUtilsMessengerCallbackEXT = extern "system" fn(
    message_severity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    message_types: VkDebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const VkDebugUtilsMessengerCallbackDataEXT,
    p_user_data: *mut core::ffi::c_void,
) -> VkBool32;

pub type PfnVkVoidFunction = extern "system" fn(
);

pub type PfnVkGetPhysicalDeviceWin32PresentationSupportKHR = extern "system" fn(
    physical_device: VkPhysicalDevice,
    queue_family_index: u32,
) -> VkBool32;

pub type PfnVkCreateWin32SurfaceKHR = extern "system" fn(
    instance: VkInstance,
    p_create_info: *const VkWin32SurfaceCreateInfoKHR,
    p_allocator: *const VkAllocationCallbacks,
    p_surface: *mut VkSurfaceKHR,
) -> VkResult;

pub type PfnVkGetPhysicalDeviceSurfacePresentModesKHR = extern "system" fn(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    p_present_mode_count: *mut u32,
    p_present_modes: *mut VkPresentModeKHR,
) -> VkResult;

pub type PfnVkGetPhysicalDeviceSurfaceFormatsKHR = extern "system" fn(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    p_surface_format_count: *mut u32,
    p_surface_formats: *mut VkSurfaceFormatKHR,
) -> VkResult;

pub type PfnVkGetPhysicalDeviceSurfaceCapabilitiesKHR = extern "system" fn(
    physical_device: VkPhysicalDevice,
    surface: VkSurfaceKHR,
    p_surface_capabilities: *mut VkSurfaceCapabilitiesKHR,
) -> VkResult;

pub type PfnVkGetPhysicalDeviceSurfaceSupportKHR = extern "system" fn(
    physical_device: VkPhysicalDevice,
    queue_family_index: u32,
    surface: VkSurfaceKHR,
    p_supported: *mut VkBool32,
) -> VkResult;

pub type PfnVkDestroySurfaceKHR = extern "system" fn(
    instance: VkInstance,
    surface: VkSurfaceKHR,
    p_allocator: *const VkAllocationCallbacks,
);

pub type PfnVkQueuePresentKHR = extern "system" fn(
    queue: VkQueue,
    p_present_info: *const VkPresentInfoKHR,
) -> VkResult;

pub type PfnVkAcquireNextImageKHR = extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    timeout: u64,
    semaphore: VkSemaphore,
    fence: VkFence,
    p_image_index: *mut u32,
) -> VkResult;

pub type PfnVkGetSwapchainImagesKHR = extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    p_swapchain_image_count: *mut u32,
    p_swapchain_images: *mut VkImage,
) -> VkResult;

pub type PfnVkDestroySwapchainKHR = extern "system" fn(
    device: VkDevice,
    swapchain: VkSwapchainKHR,
    p_allocator: *const VkAllocationCallbacks,
);

pub type PfnVkCreateSwapchainKHR = extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkSwapchainCreateInfoKHR,
    p_allocator: *const VkAllocationCallbacks,
    p_swapchain: *mut VkSwapchainKHR,
) -> VkResult;

pub type PfnVkSubmitDebugUtilsMessageEXT = extern "system" fn(
    instance: VkInstance,
    message_severity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    message_types: VkDebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const VkDebugUtilsMessengerCallbackDataEXT,
);

pub type PfnVkDestroyDebugUtilsMessengerEXT = extern "system" fn(
    instance: VkInstance,
    messenger: VkDebugUtilsMessengerEXT,
    p_allocator: *const VkAllocationCallbacks,
);

pub type PfnVkCreateDebugUtilsMessengerEXT = extern "system" fn(
    instance: VkInstance,
    p_create_info: *const VkDebugUtilsMessengerCreateInfoEXT,
    p_allocator: *const VkAllocationCallbacks,
    p_messenger: *mut VkDebugUtilsMessengerEXT,
) -> VkResult;

pub type PfnVkCmdInsertDebugUtilsLabelEXT = extern "system" fn(
    command_buffer: VkCommandBuffer,
    p_label_info: *const VkDebugUtilsLabelEXT,
);

pub type PfnVkCmdEndDebugUtilsLabelEXT = extern "system" fn(
    command_buffer: VkCommandBuffer,
);

pub type PfnVkCmdBeginDebugUtilsLabelEXT = extern "system" fn(
    command_buffer: VkCommandBuffer,
    p_label_info: *const VkDebugUtilsLabelEXT,
);

pub type PfnVkQueueInsertDebugUtilsLabelEXT = extern "system" fn(
    queue: VkQueue,
    p_label_info: *const VkDebugUtilsLabelEXT,
);

pub type PfnVkQueueEndDebugUtilsLabelEXT = extern "system" fn(
    queue: VkQueue,
);

pub type PfnVkQueueBeginDebugUtilsLabelEXT = extern "system" fn(
    queue: VkQueue,
    p_label_info: *const VkDebugUtilsLabelEXT,
);

pub type PfnVkSetDebugUtilsObjectTagEXT = extern "system" fn(
    device: VkDevice,
    p_tag_info: *const VkDebugUtilsObjectTagInfoEXT,
) -> VkResult;

pub type PfnVkSetDebugUtilsObjectNameEXT = extern "system" fn(
    device: VkDevice,
    p_name_info: *const VkDebugUtilsObjectNameInfoEXT,
) -> VkResult;

pub type PfnVkCmdExecuteCommands = extern "system" fn(
    command_buffer: VkCommandBuffer,
    command_buffer_count: u32,
    p_command_buffers: *const VkCommandBuffer,
);

pub type PfnVkCmdEndRenderPass = extern "system" fn(
    command_buffer: VkCommandBuffer,
);

pub type PfnVkCmdNextSubpass = extern "system" fn(
    command_buffer: VkCommandBuffer,
    contents: VkSubpassContents,
);

pub type PfnVkCmdBeginRenderPass = extern "system" fn(
    command_buffer: VkCommandBuffer,
    p_render_pass_begin: *const VkRenderPassBeginInfo,
    contents: VkSubpassContents,
);

pub type PfnVkCmdPushConstants = extern "system" fn(
    command_buffer: VkCommandBuffer,
    layout: VkPipelineLayout,
    stage_flags: VkShaderStageFlags,
    offset: u32,
    size: u32,
    p_values: *const core::ffi::c_void,
);

pub type PfnVkCmdCopyQueryPoolResults = extern "system" fn(
    command_buffer: VkCommandBuffer,
    query_pool: VkQueryPool,
    first_query: u32,
    query_count: u32,
    dst_buffer: VkBuffer,
    dst_offset: VkDeviceSize,
    stride: VkDeviceSize,
    flags: VkQueryResultFlags,
);

pub type PfnVkCmdWriteTimestamp = extern "system" fn(
    command_buffer: VkCommandBuffer,
    pipeline_stage: VkPipelineStageFlagBits,
    query_pool: VkQueryPool,
    query: u32,
);

pub type PfnVkCmdResetQueryPool = extern "system" fn(
    command_buffer: VkCommandBuffer,
    query_pool: VkQueryPool,
    first_query: u32,
    query_count: u32,
);

pub type PfnVkCmdEndQuery = extern "system" fn(
    command_buffer: VkCommandBuffer,
    query_pool: VkQueryPool,
    query: u32,
);

pub type PfnVkCmdBeginQuery = extern "system" fn(
    command_buffer: VkCommandBuffer,
    query_pool: VkQueryPool,
    query: u32,
    flags: VkQueryControlFlags,
);

pub type PfnVkCmdPipelineBarrier = extern "system" fn(
    command_buffer: VkCommandBuffer,
    src_stage_mask: VkPipelineStageFlags,
    dst_stage_mask: VkPipelineStageFlags,
    dependency_flags: VkDependencyFlags,
    memory_barrier_count: u32,
    p_memory_barriers: *const VkMemoryBarrier,
    buffer_memory_barrier_count: u32,
    p_buffer_memory_barriers: *const VkBufferMemoryBarrier,
    image_memory_barrier_count: u32,
    p_image_memory_barriers: *const VkImageMemoryBarrier,
);

pub type PfnVkCmdWaitEvents = extern "system" fn(
    command_buffer: VkCommandBuffer,
    event_count: u32,
    p_events: *const VkEvent,
    src_stage_mask: VkPipelineStageFlags,
    dst_stage_mask: VkPipelineStageFlags,
    memory_barrier_count: u32,
    p_memory_barriers: *const VkMemoryBarrier,
    buffer_memory_barrier_count: u32,
    p_buffer_memory_barriers: *const VkBufferMemoryBarrier,
    image_memory_barrier_count: u32,
    p_image_memory_barriers: *const VkImageMemoryBarrier,
);

pub type PfnVkCmdResetEvent = extern "system" fn(
    command_buffer: VkCommandBuffer,
    event: VkEvent,
    stage_mask: VkPipelineStageFlags,
);

pub type PfnVkCmdSetEvent = extern "system" fn(
    command_buffer: VkCommandBuffer,
    event: VkEvent,
    stage_mask: VkPipelineStageFlags,
);

pub type PfnVkCmdResolveImage = extern "system" fn(
    command_buffer: VkCommandBuffer,
    src_image: VkImage,
    src_image_layout: VkImageLayout,
    dst_image: VkImage,
    dst_image_layout: VkImageLayout,
    region_count: u32,
    p_regions: *const VkImageResolve,
);

pub type PfnVkCmdClearAttachments = extern "system" fn(
    command_buffer: VkCommandBuffer,
    attachment_count: u32,
    p_attachments: *const VkClearAttachment,
    rect_count: u32,
    p_rects: *const VkClearRect,
);

pub type PfnVkCmdClearDepthStencilImage = extern "system" fn(
    command_buffer: VkCommandBuffer,
    image: VkImage,
    image_layout: VkImageLayout,
    p_depth_stencil: *const VkClearDepthStencilValue,
    range_count: u32,
    p_ranges: *const VkImageSubresourceRange,
);

pub type PfnVkCmdClearColorImage = extern "system" fn(
    command_buffer: VkCommandBuffer,
    image: VkImage,
    image_layout: VkImageLayout,
    p_color: *const VkClearColorValue,
    range_count: u32,
    p_ranges: *const VkImageSubresourceRange,
);

pub type PfnVkCmdFillBuffer = extern "system" fn(
    command_buffer: VkCommandBuffer,
    dst_buffer: VkBuffer,
    dst_offset: VkDeviceSize,
    size: VkDeviceSize,
    data: u32,
);

pub type PfnVkCmdUpdateBuffer = extern "system" fn(
    command_buffer: VkCommandBuffer,
    dst_buffer: VkBuffer,
    dst_offset: VkDeviceSize,
    data_size: VkDeviceSize,
    p_data: *const core::ffi::c_void,
);

pub type PfnVkCmdCopyImageToBuffer = extern "system" fn(
    command_buffer: VkCommandBuffer,
    src_image: VkImage,
    src_image_layout: VkImageLayout,
    dst_buffer: VkBuffer,
    region_count: u32,
    p_regions: *const VkBufferImageCopy,
);

pub type PfnVkCmdCopyBufferToImage = extern "system" fn(
    command_buffer: VkCommandBuffer,
    src_buffer: VkBuffer,
    dst_image: VkImage,
    dst_image_layout: VkImageLayout,
    region_count: u32,
    p_regions: *const VkBufferImageCopy,
);

pub type PfnVkCmdBlitImage = extern "system" fn(
    command_buffer: VkCommandBuffer,
    src_image: VkImage,
    src_image_layout: VkImageLayout,
    dst_image: VkImage,
    dst_image_layout: VkImageLayout,
    region_count: u32,
    p_regions: *const VkImageBlit,
    filter: VkFilter,
);

pub type PfnVkCmdCopyImage = extern "system" fn(
    command_buffer: VkCommandBuffer,
    src_image: VkImage,
    src_image_layout: VkImageLayout,
    dst_image: VkImage,
    dst_image_layout: VkImageLayout,
    region_count: u32,
    p_regions: *const VkImageCopy,
);

pub type PfnVkCmdCopyBuffer = extern "system" fn(
    command_buffer: VkCommandBuffer,
    src_buffer: VkBuffer,
    dst_buffer: VkBuffer,
    region_count: u32,
    p_regions: *const VkBufferCopy,
);

pub type PfnVkCmdDispatchIndirect = extern "system" fn(
    command_buffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
);

pub type PfnVkCmdDispatch = extern "system" fn(
    command_buffer: VkCommandBuffer,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
);

pub type PfnVkCmdDrawIndexedIndirect = extern "system" fn(
    command_buffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    draw_count: u32,
    stride: u32,
);

pub type PfnVkCmdDrawIndirect = extern "system" fn(
    command_buffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    draw_count: u32,
    stride: u32,
);

pub type PfnVkCmdDrawIndexed = extern "system" fn(
    command_buffer: VkCommandBuffer,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    vertex_offset: i32,
    first_instance: u32,
);

pub type PfnVkCmdDraw = extern "system" fn(
    command_buffer: VkCommandBuffer,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
);

pub type PfnVkCmdBindVertexBuffers = extern "system" fn(
    command_buffer: VkCommandBuffer,
    first_binding: u32,
    binding_count: u32,
    p_buffers: *const VkBuffer,
    p_offsets: *const VkDeviceSize,
);

pub type PfnVkCmdBindIndexBuffer = extern "system" fn(
    command_buffer: VkCommandBuffer,
    buffer: VkBuffer,
    offset: VkDeviceSize,
    index_type: VkIndexType,
);

pub type PfnVkCmdBindDescriptorSets = extern "system" fn(
    command_buffer: VkCommandBuffer,
    pipeline_bind_point: VkPipelineBindPoint,
    layout: VkPipelineLayout,
    first_set: u32,
    descriptor_set_count: u32,
    p_descriptor_sets: *const VkDescriptorSet,
    dynamic_offset_count: u32,
    p_dynamic_offsets: *const u32,
);

pub type PfnVkCmdSetStencilReference = extern "system" fn(
    command_buffer: VkCommandBuffer,
    face_mask: VkStencilFaceFlags,
    reference: u32,
);

pub type PfnVkCmdSetStencilWriteMask = extern "system" fn(
    command_buffer: VkCommandBuffer,
    face_mask: VkStencilFaceFlags,
    write_mask: u32,
);

pub type PfnVkCmdSetStencilCompareMask = extern "system" fn(
    command_buffer: VkCommandBuffer,
    face_mask: VkStencilFaceFlags,
    compare_mask: u32,
);

pub type PfnVkCmdSetDepthBounds = extern "system" fn(
    command_buffer: VkCommandBuffer,
    min_depth_bounds: f32,
    max_depth_bounds: f32,
);

pub type PfnVkCmdSetBlendConstants = extern "system" fn(
    command_buffer: VkCommandBuffer,
    blend_constants: [f32; 4],
);

pub type PfnVkCmdSetDepthBias = extern "system" fn(
    command_buffer: VkCommandBuffer,
    depth_bias_constant_factor: f32,
    depth_bias_clamp: f32,
    depth_bias_slope_factor: f32,
);

pub type PfnVkCmdSetLineWidth = extern "system" fn(
    command_buffer: VkCommandBuffer,
    line_width: f32,
);

pub type PfnVkCmdSetScissor = extern "system" fn(
    command_buffer: VkCommandBuffer,
    first_scissor: u32,
    scissor_count: u32,
    p_scissors: *const VkRect2D,
);

pub type PfnVkCmdSetViewport = extern "system" fn(
    command_buffer: VkCommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    p_viewports: *const VkViewport,
);

pub type PfnVkCmdBindPipeline = extern "system" fn(
    command_buffer: VkCommandBuffer,
    pipeline_bind_point: VkPipelineBindPoint,
    pipeline: VkPipeline,
);

pub type PfnVkResetCommandBuffer = extern "system" fn(
    command_buffer: VkCommandBuffer,
    flags: VkCommandBufferResetFlags,
) -> VkResult;

pub type PfnVkEndCommandBuffer = extern "system" fn(
    command_buffer: VkCommandBuffer,
) -> VkResult;

pub type PfnVkBeginCommandBuffer = extern "system" fn(
    command_buffer: VkCommandBuffer,
    p_begin_info: *const VkCommandBufferBeginInfo,
) -> VkResult;

pub type PfnVkFreeCommandBuffers = extern "system" fn(
    device: VkDevice,
    command_pool: VkCommandPool,
    command_buffer_count: u32,
    p_command_buffers: *const VkCommandBuffer,
);

pub type PfnVkAllocateCommandBuffers = extern "system" fn(
    device: VkDevice,
    p_allocate_info: *const VkCommandBufferAllocateInfo,
    p_command_buffers: *mut VkCommandBuffer,
) -> VkResult;

pub type PfnVkResetCommandPool = extern "system" fn(
    device: VkDevice,
    command_pool: VkCommandPool,
    flags: VkCommandPoolResetFlags,
) -> VkResult;

pub type PfnVkDestroyCommandPool = extern "system" fn(
    device: VkDevice,
    command_pool: VkCommandPool,
    p_allocator: *const VkAllocationCallbacks,
);

pub type PfnVkCreateCommandPool = extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkCommandPoolCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_command_pool: *mut VkCommandPool,
) -> VkResult;

pub type PfnVkGetRenderAreaGranularity = extern "system" fn(
    device: VkDevice,
    render_pass: VkRenderPass,
    p_granularity: *mut VkExtent2D,
);

pub type PfnVkDestroyRenderPass = extern "system" fn(
    device: VkDevice,
    render_pass: VkRenderPass,
    p_allocator: *const VkAllocationCallbacks,
);

pub type PfnVkCreateRenderPass = extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkRenderPassCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_render_pass: *mut VkRenderPass,
) -> VkResult;

pub type PfnVkDestroyFramebuffer = extern "system" fn(
    device: VkDevice,
    framebuffer: VkFramebuffer,
    p_allocator: *const VkAllocationCallbacks,
);

pub type PfnVkCreateFramebuffer = extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkFramebufferCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_framebuffer: *mut VkFramebuffer,
) -> VkResult;

pub type PfnVkUpdateDescriptorSets = extern "system" fn(
    device: VkDevice,
    descriptor_write_count: u32,
    p_descriptor_writes: *const VkWriteDescriptorSet,
    descriptor_copy_count: u32,
    p_descriptor_copies: *const VkCopyDescriptorSet,
);

pub type PfnVkFreeDescriptorSets = extern "system" fn(
    device: VkDevice,
    descriptor_pool: VkDescriptorPool,
    descriptor_set_count: u32,
    p_descriptor_sets: *const VkDescriptorSet,
) -> VkResult;

pub type PfnVkAllocateDescriptorSets = extern "system" fn(
    device: VkDevice,
    p_allocate_info: *const VkDescriptorSetAllocateInfo,
    p_descriptor_sets: *mut VkDescriptorSet,
) -> VkResult;

pub type PfnVkResetDescriptorPool = extern "system" fn(
    device: VkDevice,
    descriptor_pool: VkDescriptorPool,
    flags: VkDescriptorPoolResetFlags,
) -> VkResult;

pub type PfnVkDestroyDescriptorPool = extern "system" fn(
    device: VkDevice,
    descriptor_pool: VkDescriptorPool,
    p_allocator: *const VkAllocationCallbacks,
);

pub type PfnVkCreateDescriptorPool = extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkDescriptorPoolCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_descriptor_pool: *mut VkDescriptorPool,
) -> VkResult;

pub type PfnVkDestroyDescriptorSetLayout = extern "system" fn(
    device: VkDevice,
    descriptor_set_layout: VkDescriptorSetLayout,
    p_allocator: *const VkAllocationCallbacks,
);

pub type PfnVkCreateDescriptorSetLayout = extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkDescriptorSetLayoutCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_set_layout: *mut VkDescriptorSetLayout,
) -> VkResult;

pub type PfnVkDestroySampler = extern "system" fn(
    device: VkDevice,
    sampler: VkSampler,
    p_allocator: *const VkAllocationCallbacks,
);

pub type PfnVkCreateSampler = extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkSamplerCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_sampler: *mut VkSampler,
) -> VkResult;

pub type PfnVkDestroyPipelineLayout = extern "system" fn(
    device: VkDevice,
    pipeline_layout: VkPipelineLayout,
    p_allocator: *const VkAllocationCallbacks,
);

pub type PfnVkCreatePipelineLayout = extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkPipelineLayoutCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_pipeline_layout: *mut VkPipelineLayout,
) -> VkResult;

pub type PfnVkDestroyPipeline = extern "system" fn(
    device: VkDevice,
    pipeline: VkPipeline,
    p_allocator: *const VkAllocationCallbacks,
);

pub type PfnVkCreateComputePipelines = extern "system" fn(
    device: VkDevice,
    pipeline_cache: VkPipelineCache,
    create_info_count: u32,
    p_create_infos: *const VkComputePipelineCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_pipelines: *mut VkPipeline,
) -> VkResult;

pub type PfnVkCreateGraphicsPipelines = extern "system" fn(
    device: VkDevice,
    pipeline_cache: VkPipelineCache,
    create_info_count: u32,
    p_create_infos: *const VkGraphicsPipelineCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_pipelines: *mut VkPipeline,
) -> VkResult;

pub type PfnVkMergePipelineCaches = extern "system" fn(
    device: VkDevice,
    dst_cache: VkPipelineCache,
    src_cache_count: u32,
    p_src_caches: *const VkPipelineCache,
) -> VkResult;

pub type PfnVkGetPipelineCacheData = extern "system" fn(
    device: VkDevice,
    pipeline_cache: VkPipelineCache,
    p_data_size: *mut usize,
    p_data: *mut core::ffi::c_void,
) -> VkResult;

pub type PfnVkDestroyPipelineCache = extern "system" fn(
    device: VkDevice,
    pipeline_cache: VkPipelineCache,
    p_allocator: *const VkAllocationCallbacks,
);

pub type PfnVkCreatePipelineCache = extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkPipelineCacheCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_pipeline_cache: *mut VkPipelineCache,
) -> VkResult;

pub type PfnVkDestroyShaderModule = extern "system" fn(
    device: VkDevice,
    shader_module: VkShaderModule,
    p_allocator: *const VkAllocationCallbacks,
);

pub type PfnVkCreateShaderModule = extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkShaderModuleCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_shader_module: *mut VkShaderModule,
) -> VkResult;

pub type PfnVkDestroyImageView = extern "system" fn(
    device: VkDevice,
    image_view: VkImageView,
    p_allocator: *const VkAllocationCallbacks,
);

pub type PfnVkCreateImageView = extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkImageViewCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_view: *mut VkImageView,
) -> VkResult;

pub type PfnVkGetImageSubresourceLayout = extern "system" fn(
    device: VkDevice,
    image: VkImage,
    p_subresource: *const VkImageSubresource,
    p_layout: *mut VkSubresourceLayout,
);

pub type PfnVkDestroyImage = extern "system" fn(
    device: VkDevice,
    image: VkImage,
    p_allocator: *const VkAllocationCallbacks,
);

pub type PfnVkCreateImage = extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkImageCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_image: *mut VkImage,
) -> VkResult;

pub type PfnVkDestroyBufferView = extern "system" fn(
    device: VkDevice,
    buffer_view: VkBufferView,
    p_allocator: *const VkAllocationCallbacks,
);

pub type PfnVkCreateBufferView = extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkBufferViewCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_view: *mut VkBufferView,
) -> VkResult;

pub type PfnVkDestroyBuffer = extern "system" fn(
    device: VkDevice,
    buffer: VkBuffer,
    p_allocator: *const VkAllocationCallbacks,
);

pub type PfnVkCreateBuffer = extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkBufferCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_buffer: *mut VkBuffer,
) -> VkResult;

pub type PfnVkGetQueryPoolResults = extern "system" fn(
    device: VkDevice,
    query_pool: VkQueryPool,
    first_query: u32,
    query_count: u32,
    data_size: usize,
    p_data: *mut core::ffi::c_void,
    stride: VkDeviceSize,
    flags: VkQueryResultFlags,
) -> VkResult;

pub type PfnVkDestroyQueryPool = extern "system" fn(
    device: VkDevice,
    query_pool: VkQueryPool,
    p_allocator: *const VkAllocationCallbacks,
);

pub type PfnVkCreateQueryPool = extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkQueryPoolCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_query_pool: *mut VkQueryPool,
) -> VkResult;

pub type PfnVkResetEvent = extern "system" fn(
    device: VkDevice,
    event: VkEvent,
) -> VkResult;

pub type PfnVkSetEvent = extern "system" fn(
    device: VkDevice,
    event: VkEvent,
) -> VkResult;

pub type PfnVkGetEventStatus = extern "system" fn(
    device: VkDevice,
    event: VkEvent,
) -> VkResult;

pub type PfnVkDestroyEvent = extern "system" fn(
    device: VkDevice,
    event: VkEvent,
    p_allocator: *const VkAllocationCallbacks,
);

pub type PfnVkCreateEvent = extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkEventCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_event: *mut VkEvent,
) -> VkResult;

pub type PfnVkDestroySemaphore = extern "system" fn(
    device: VkDevice,
    semaphore: VkSemaphore,
    p_allocator: *const VkAllocationCallbacks,
);

pub type PfnVkCreateSemaphore = extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkSemaphoreCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_semaphore: *mut VkSemaphore,
) -> VkResult;

pub type PfnVkWaitForFences = extern "system" fn(
    device: VkDevice,
    fence_count: u32,
    p_fences: *const VkFence,
    wait_all: VkBool32,
    timeout: u64,
) -> VkResult;

pub type PfnVkGetFenceStatus = extern "system" fn(
    device: VkDevice,
    fence: VkFence,
) -> VkResult;

pub type PfnVkResetFences = extern "system" fn(
    device: VkDevice,
    fence_count: u32,
    p_fences: *const VkFence,
) -> VkResult;

pub type PfnVkDestroyFence = extern "system" fn(
    device: VkDevice,
    fence: VkFence,
    p_allocator: *const VkAllocationCallbacks,
);

pub type PfnVkCreateFence = extern "system" fn(
    device: VkDevice,
    p_create_info: *const VkFenceCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_fence: *mut VkFence,
) -> VkResult;

pub type PfnVkQueueBindSparse = extern "system" fn(
    queue: VkQueue,
    bind_info_count: u32,
    p_bind_info: *const VkBindSparseInfo,
    fence: VkFence,
) -> VkResult;

pub type PfnVkGetPhysicalDeviceSparseImageFormatProperties = extern "system" fn(
    physical_device: VkPhysicalDevice,
    format: VkFormat,
    kind: VkImageType,
    samples: VkSampleCountFlagBits,
    usage: VkImageUsageFlags,
    tiling: VkImageTiling,
    p_property_count: *mut u32,
    p_properties: *mut VkSparseImageFormatProperties,
);

pub type PfnVkGetImageSparseMemoryRequirements = extern "system" fn(
    device: VkDevice,
    image: VkImage,
    p_sparse_memory_requirement_count: *mut u32,
    p_sparse_memory_requirements: *mut VkSparseImageMemoryRequirements,
);

pub type PfnVkGetImageMemoryRequirements = extern "system" fn(
    device: VkDevice,
    image: VkImage,
    p_memory_requirements: *mut VkMemoryRequirements,
);

pub type PfnVkGetBufferMemoryRequirements = extern "system" fn(
    device: VkDevice,
    buffer: VkBuffer,
    p_memory_requirements: *mut VkMemoryRequirements,
);

pub type PfnVkBindImageMemory = extern "system" fn(
    device: VkDevice,
    image: VkImage,
    memory: VkDeviceMemory,
    memory_offset: VkDeviceSize,
) -> VkResult;

pub type PfnVkBindBufferMemory = extern "system" fn(
    device: VkDevice,
    buffer: VkBuffer,
    memory: VkDeviceMemory,
    memory_offset: VkDeviceSize,
) -> VkResult;

pub type PfnVkGetDeviceMemoryCommitment = extern "system" fn(
    device: VkDevice,
    memory: VkDeviceMemory,
    p_committed_memory_in_bytes: *mut VkDeviceSize,
);

pub type PfnVkInvalidateMappedMemoryRanges = extern "system" fn(
    device: VkDevice,
    memory_range_count: u32,
    p_memory_ranges: *const VkMappedMemoryRange,
) -> VkResult;

pub type PfnVkFlushMappedMemoryRanges = extern "system" fn(
    device: VkDevice,
    memory_range_count: u32,
    p_memory_ranges: *const VkMappedMemoryRange,
) -> VkResult;

pub type PfnVkUnmapMemory = extern "system" fn(
    device: VkDevice,
    memory: VkDeviceMemory,
);

pub type PfnVkMapMemory = extern "system" fn(
    device: VkDevice,
    memory: VkDeviceMemory,
    offset: VkDeviceSize,
    size: VkDeviceSize,
    flags: VkMemoryMapFlags,
    pp_data: *mut *mut core::ffi::c_void,
) -> VkResult;

pub type PfnVkFreeMemory = extern "system" fn(
    device: VkDevice,
    memory: VkDeviceMemory,
    p_allocator: *const VkAllocationCallbacks,
);

pub type PfnVkAllocateMemory = extern "system" fn(
    device: VkDevice,
    p_allocate_info: *const VkMemoryAllocateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_memory: *mut VkDeviceMemory,
) -> VkResult;

pub type PfnVkDeviceWaitIdle = extern "system" fn(
    device: VkDevice,
) -> VkResult;

pub type PfnVkQueueWaitIdle = extern "system" fn(
    queue: VkQueue,
) -> VkResult;

pub type PfnVkQueueSubmit = extern "system" fn(
    queue: VkQueue,
    submit_count: u32,
    p_submits: *const VkSubmitInfo,
    fence: VkFence,
) -> VkResult;

pub type PfnVkGetDeviceQueue = extern "system" fn(
    device: VkDevice,
    queue_family_index: u32,
    queue_index: u32,
    p_queue: *mut VkQueue,
);

pub type PfnVkEnumerateDeviceLayerProperties = extern "system" fn(
    physical_device: VkPhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut VkLayerProperties,
) -> VkResult;

pub type PfnVkEnumerateInstanceLayerProperties = extern "system" fn(
    p_property_count: *mut u32,
    p_properties: *mut VkLayerProperties,
) -> VkResult;

pub type PfnVkEnumerateDeviceExtensionProperties = extern "system" fn(
    physical_device: VkPhysicalDevice,
    p_layer_name: *const u8,
    p_property_count: *mut u32,
    p_properties: *mut VkExtensionProperties,
) -> VkResult;

pub type PfnVkEnumerateInstanceExtensionProperties = extern "system" fn(
    p_layer_name: *const u8,
    p_property_count: *mut u32,
    p_properties: *mut VkExtensionProperties,
) -> VkResult;

pub type PfnVkDestroyDevice = extern "system" fn(
    device: VkDevice,
    p_allocator: *const VkAllocationCallbacks,
);

pub type PfnVkCreateDevice = extern "system" fn(
    physical_device: VkPhysicalDevice,
    p_create_info: *const VkDeviceCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_device: *mut VkDevice,
) -> VkResult;

pub type PfnVkGetDeviceProcAddr = extern "system" fn(
    device: VkDevice,
    p_name: *const u8,
) -> PfnVkVoidFunction;

pub type PfnVkGetInstanceProcAddr = extern "system" fn(
    instance: VkInstance,
    p_name: *const u8,
) -> PfnVkVoidFunction;

pub type PfnVkGetPhysicalDeviceMemoryProperties = extern "system" fn(
    physical_device: VkPhysicalDevice,
    p_memory_properties: *mut VkPhysicalDeviceMemoryProperties,
);

pub type PfnVkGetPhysicalDeviceQueueFamilyProperties = extern "system" fn(
    physical_device: VkPhysicalDevice,
    p_queue_family_property_count: *mut u32,
    p_queue_family_properties: *mut VkQueueFamilyProperties,
);

pub type PfnVkGetPhysicalDeviceProperties = extern "system" fn(
    physical_device: VkPhysicalDevice,
    p_properties: *mut VkPhysicalDeviceProperties,
);

pub type PfnVkGetPhysicalDeviceImageFormatProperties = extern "system" fn(
    physical_device: VkPhysicalDevice,
    format: VkFormat,
    kind: VkImageType,
    tiling: VkImageTiling,
    usage: VkImageUsageFlags,
    flags: VkImageCreateFlags,
    p_image_format_properties: *mut VkImageFormatProperties,
) -> VkResult;

pub type PfnVkGetPhysicalDeviceFormatProperties = extern "system" fn(
    physical_device: VkPhysicalDevice,
    format: VkFormat,
    p_format_properties: *mut VkFormatProperties,
);

pub type PfnVkGetPhysicalDeviceFeatures = extern "system" fn(
    physical_device: VkPhysicalDevice,
    p_features: *mut VkPhysicalDeviceFeatures,
);

pub type PfnVkEnumeratePhysicalDevices = extern "system" fn(
    instance: VkInstance,
    p_physical_device_count: *mut u32,
    p_physical_devices: *mut VkPhysicalDevice,
) -> VkResult;

pub type PfnVkDestroyInstance = extern "system" fn(
    instance: VkInstance,
    p_allocator: *const VkAllocationCallbacks,
);

pub type PfnVkCreateInstance = extern "system" fn(
    p_create_info: *const VkInstanceCreateInfo,
    p_allocator: *const VkAllocationCallbacks,
    p_instance: *mut VkInstance,
) -> VkResult;

