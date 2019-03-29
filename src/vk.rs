pub type HINSTANCE = usize;
pub type HWND = usize;

pub const KHR_WIN32_SURFACE_SPEC_VERSION: u32 = 6;
pub const KHR_SURFACE_SPEC_VERSION: u32 = 25;
pub const KHR_SWAPCHAIN_SPEC_VERSION: u32 = 70;
pub const EXT_DEBUG_UTILS_SPEC_VERSION: u32 = 1;
pub const SUBPASS_EXTERNAL: u32 = 4294967295;
pub const QUEUE_FAMILY_IGNORED: u32 = 4294967295;
pub const FALSE: u32 = 0;
pub const TRUE: u32 = 1;
pub const ATTACHMENT_UNUSED: u32 = 4294967295;
pub const WHOLE_SIZE: u64 = 18446744073709551615;
pub const REMAINING_ARRAY_LAYERS: u32 = 4294967295;
pub const REMAINING_MIP_LEVELS: u32 = 4294967295;

pub const LOD_CLAMP_NONE: f32 = 1000.000000;

pub const KHR_WIN32_SURFACE_EXTENSION_NAME: &str = "VK_KHR_win32_surface";
pub const KHR_WIN32_SURFACE_EXTENSION_NAME__C: &[u8] = b"VK_KHR_win32_surface\0";
pub const KHR_SURFACE_EXTENSION_NAME: &str = "VK_KHR_surface";
pub const KHR_SURFACE_EXTENSION_NAME__C: &[u8] = b"VK_KHR_surface\0";
pub const KHR_SWAPCHAIN_EXTENSION_NAME: &str = "VK_KHR_swapchain";
pub const KHR_SWAPCHAIN_EXTENSION_NAME__C: &[u8] = b"VK_KHR_swapchain\0";
pub const EXT_DEBUG_UTILS_EXTENSION_NAME: &str = "VK_EXT_debug_utils";
pub const EXT_DEBUG_UTILS_EXTENSION_NAME__C: &[u8] = b"VK_EXT_debug_utils\0";

pub type Bool32 = u32;
pub type Flags = u32;
pub type DeviceSize = u64;
pub type SampleMask = u32;

pub type ImageUsageFlags = ImageUsageFlagBits;
pub type CompositeAlphaFlagsKHR = CompositeAlphaFlagBitsKHR;
pub type SurfaceTransformFlagsKHR = SurfaceTransformFlagBitsKHR;
pub type SwapchainCreateFlagsKHR = SwapchainCreateFlagBitsKHR;
pub type DebugUtilsMessageTypeFlagsEXT = DebugUtilsMessageTypeFlagBitsEXT;
pub type DebugUtilsMessageSeverityFlagsEXT = DebugUtilsMessageSeverityFlagBitsEXT;
pub type AccessFlags = AccessFlagBits;
pub type ImageAspectFlags = ImageAspectFlagBits;
pub type ShaderStageFlags = ShaderStageFlagBits;
pub type QueryResultFlags = QueryResultFlagBits;
pub type QueryControlFlags = QueryControlFlagBits;
pub type DependencyFlags = DependencyFlagBits;
pub type PipelineStageFlags = PipelineStageFlagBits;
pub type StencilFaceFlags = StencilFaceFlagBits;
pub type CommandBufferResetFlags = CommandBufferResetFlagBits;
pub type QueryPipelineStatisticFlags = QueryPipelineStatisticFlagBits;
pub type CommandBufferUsageFlags = CommandBufferUsageFlagBits;
pub type CommandPoolResetFlags = CommandPoolResetFlagBits;
pub type CommandPoolCreateFlags = CommandPoolCreateFlagBits;
pub type SubpassDescriptionFlags = SubpassDescriptionFlagBits;
pub type AttachmentDescriptionFlags = AttachmentDescriptionFlagBits;
pub type DescriptorPoolCreateFlags = DescriptorPoolCreateFlagBits;
pub type DescriptorSetLayoutCreateFlags = DescriptorSetLayoutCreateFlagBits;
pub type SamplerCreateFlags = SamplerCreateFlagBits;
pub type PipelineCreateFlags = PipelineCreateFlagBits;
pub type ColorComponentFlags = ColorComponentFlagBits;
pub type CullModeFlags = CullModeFlagBits;
pub type ImageViewCreateFlags = ImageViewCreateFlagBits;
pub type ImageCreateFlags = ImageCreateFlagBits;
pub type BufferUsageFlags = BufferUsageFlagBits;
pub type BufferCreateFlags = BufferCreateFlagBits;
pub type FenceCreateFlags = FenceCreateFlagBits;
pub type SparseMemoryBindFlags = SparseMemoryBindFlagBits;
pub type SparseImageFormatFlags = SparseImageFormatFlagBits;
pub type DeviceQueueCreateFlags = DeviceQueueCreateFlagBits;
pub type MemoryHeapFlags = MemoryHeapFlagBits;
pub type MemoryPropertyFlags = MemoryPropertyFlagBits;
pub type QueueFlags = QueueFlagBits;
pub type SampleCountFlags = SampleCountFlagBits;
pub type FormatFeatureFlags = FormatFeatureFlagBits;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct PhysicalDevice(usize);
impl PhysicalDevice {
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
pub struct SurfaceKHR(u64);
impl SurfaceKHR {
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
pub struct Instance(usize);
impl Instance {
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
pub struct SwapchainKHR(u64);
impl SwapchainKHR {
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
pub struct Semaphore(u64);
impl Semaphore {
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
pub struct Queue(usize);
impl Queue {
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
pub struct Fence(u64);
impl Fence {
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
pub struct Device(usize);
impl Device {
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
pub struct Image(u64);
impl Image {
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
pub struct DebugUtilsMessengerEXT(u64);
impl DebugUtilsMessengerEXT {
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
pub struct CommandBuffer(usize);
impl CommandBuffer {
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
pub struct Buffer(u64);
impl Buffer {
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
pub struct Framebuffer(u64);
impl Framebuffer {
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
pub struct RenderPass(u64);
impl RenderPass {
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
pub struct PipelineLayout(u64);
impl PipelineLayout {
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
pub struct QueryPool(u64);
impl QueryPool {
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
pub struct Event(u64);
impl Event {
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
pub struct DescriptorSet(u64);
impl DescriptorSet {
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
pub struct Pipeline(u64);
impl Pipeline {
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
pub struct CommandPool(u64);
impl CommandPool {
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
pub struct ImageView(u64);
impl ImageView {
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
pub struct BufferView(u64);
impl BufferView {
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
pub struct Sampler(u64);
impl Sampler {
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
pub struct DescriptorPool(u64);
impl DescriptorPool {
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
pub struct DescriptorSetLayout(u64);
impl DescriptorSetLayout {
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
pub struct ShaderModule(u64);
impl ShaderModule {
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
pub struct PipelineCache(u64);
impl PipelineCache {
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
pub struct DeviceMemory(u64);
impl DeviceMemory {
    #[inline]
    pub fn from_raw(r: u64) -> Self {
        Self(r)
    }

    #[inline]
    pub fn as_raw(&self) -> u64 {
        self.0
    }
}

pub type Win32SurfaceCreateFlagsKHR = Flags;
pub type DebugUtilsMessengerCallbackDataFlagsEXT = Flags;
pub type DebugUtilsMessengerCreateFlagsEXT = Flags;
pub type RenderPassCreateFlags = Flags;
pub type FramebufferCreateFlags = Flags;
pub type DescriptorPoolResetFlags = Flags;
pub type PipelineLayoutCreateFlags = Flags;
pub type PipelineShaderStageCreateFlags = Flags;
pub type PipelineDynamicStateCreateFlags = Flags;
pub type PipelineColorBlendStateCreateFlags = Flags;
pub type PipelineDepthStencilStateCreateFlags = Flags;
pub type PipelineMultisampleStateCreateFlags = Flags;
pub type PipelineRasterizationStateCreateFlags = Flags;
pub type PipelineViewportStateCreateFlags = Flags;
pub type PipelineTessellationStateCreateFlags = Flags;
pub type PipelineInputAssemblyStateCreateFlags = Flags;
pub type PipelineVertexInputStateCreateFlags = Flags;
pub type PipelineCacheCreateFlags = Flags;
pub type ShaderModuleCreateFlags = Flags;
pub type BufferViewCreateFlags = Flags;
pub type QueryPoolCreateFlags = Flags;
pub type EventCreateFlags = Flags;
pub type SemaphoreCreateFlags = Flags;
pub type MemoryMapFlags = Flags;
pub type DeviceCreateFlags = Flags;
pub type InstanceCreateFlags = Flags;

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct SystemAllocationScope(u32);
impl SystemAllocationScope {
    pub const COMMAND: SystemAllocationScope = SystemAllocationScope(0);
    pub const OBJECT: SystemAllocationScope = SystemAllocationScope(1);
    pub const CACHE: SystemAllocationScope = SystemAllocationScope(2);
    pub const DEVICE: SystemAllocationScope = SystemAllocationScope(3);
    pub const INSTANCE: SystemAllocationScope = SystemAllocationScope(4);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct InternalAllocationType(u32);
impl InternalAllocationType {
    pub const EXECUTABLE: InternalAllocationType = InternalAllocationType(0);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct StructureType(u32);
impl StructureType {
    pub const APPLICATION_INFO: StructureType = StructureType(0);
    pub const INSTANCE_CREATE_INFO: StructureType = StructureType(1);
    pub const DEVICE_QUEUE_CREATE_INFO: StructureType = StructureType(2);
    pub const DEVICE_CREATE_INFO: StructureType = StructureType(3);
    pub const SUBMIT_INFO: StructureType = StructureType(4);
    pub const MEMORY_ALLOCATE_INFO: StructureType = StructureType(5);
    pub const MAPPED_MEMORY_RANGE: StructureType = StructureType(6);
    pub const BIND_SPARSE_INFO: StructureType = StructureType(7);
    pub const FENCE_CREATE_INFO: StructureType = StructureType(8);
    pub const SEMAPHORE_CREATE_INFO: StructureType = StructureType(9);
    pub const EVENT_CREATE_INFO: StructureType = StructureType(10);
    pub const QUERY_POOL_CREATE_INFO: StructureType = StructureType(11);
    pub const BUFFER_CREATE_INFO: StructureType = StructureType(12);
    pub const BUFFER_VIEW_CREATE_INFO: StructureType = StructureType(13);
    pub const IMAGE_CREATE_INFO: StructureType = StructureType(14);
    pub const IMAGE_VIEW_CREATE_INFO: StructureType = StructureType(15);
    pub const SHADER_MODULE_CREATE_INFO: StructureType = StructureType(16);
    pub const PIPELINE_CACHE_CREATE_INFO: StructureType = StructureType(17);
    pub const PIPELINE_SHADER_STAGE_CREATE_INFO: StructureType = StructureType(18);
    pub const PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO: StructureType = StructureType(19);
    pub const PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO: StructureType = StructureType(20);
    pub const PIPELINE_TESSELLATION_STATE_CREATE_INFO: StructureType = StructureType(21);
    pub const PIPELINE_VIEWPORT_STATE_CREATE_INFO: StructureType = StructureType(22);
    pub const PIPELINE_RASTERIZATION_STATE_CREATE_INFO: StructureType = StructureType(23);
    pub const PIPELINE_MULTISAMPLE_STATE_CREATE_INFO: StructureType = StructureType(24);
    pub const PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO: StructureType = StructureType(25);
    pub const PIPELINE_COLOR_BLEND_STATE_CREATE_INFO: StructureType = StructureType(26);
    pub const PIPELINE_DYNAMIC_STATE_CREATE_INFO: StructureType = StructureType(27);
    pub const GRAPHICS_PIPELINE_CREATE_INFO: StructureType = StructureType(28);
    pub const COMPUTE_PIPELINE_CREATE_INFO: StructureType = StructureType(29);
    pub const PIPELINE_LAYOUT_CREATE_INFO: StructureType = StructureType(30);
    pub const SAMPLER_CREATE_INFO: StructureType = StructureType(31);
    pub const DESCRIPTOR_SET_LAYOUT_CREATE_INFO: StructureType = StructureType(32);
    pub const DESCRIPTOR_POOL_CREATE_INFO: StructureType = StructureType(33);
    pub const DESCRIPTOR_SET_ALLOCATE_INFO: StructureType = StructureType(34);
    pub const WRITE_DESCRIPTOR_SET: StructureType = StructureType(35);
    pub const COPY_DESCRIPTOR_SET: StructureType = StructureType(36);
    pub const FRAMEBUFFER_CREATE_INFO: StructureType = StructureType(37);
    pub const RENDER_PASS_CREATE_INFO: StructureType = StructureType(38);
    pub const COMMAND_POOL_CREATE_INFO: StructureType = StructureType(39);
    pub const COMMAND_BUFFER_ALLOCATE_INFO: StructureType = StructureType(40);
    pub const COMMAND_BUFFER_INHERITANCE_INFO: StructureType = StructureType(41);
    pub const COMMAND_BUFFER_BEGIN_INFO: StructureType = StructureType(42);
    pub const RENDER_PASS_BEGIN_INFO: StructureType = StructureType(43);
    pub const BUFFER_MEMORY_BARRIER: StructureType = StructureType(44);
    pub const IMAGE_MEMORY_BARRIER: StructureType = StructureType(45);
    pub const MEMORY_BARRIER: StructureType = StructureType(46);
    pub const LOADER_INSTANCE_CREATE_INFO: StructureType = StructureType(47);
    pub const LOADER_DEVICE_CREATE_INFO: StructureType = StructureType(48);
    pub const SWAPCHAIN_CREATE_INFO_KHR: StructureType = StructureType(1000001000);
    pub const PRESENT_INFO_KHR: StructureType = StructureType(1000001001);
    pub const WIN32_SURFACE_CREATE_INFO_KHR: StructureType = StructureType(1000009000);
    pub const DEBUG_UTILS_OBJECT_NAME_INFO_EXT: StructureType = StructureType(1000128000);
    pub const DEBUG_UTILS_OBJECT_TAG_INFO_EXT: StructureType = StructureType(1000128001);
    pub const DEBUG_UTILS_LABEL_EXT: StructureType = StructureType(1000128002);
    pub const DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT: StructureType = StructureType(1000128003);
    pub const DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT: StructureType = StructureType(1000128004);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct Result(u32);
impl Result {
    pub const ERROR_OUT_OF_DATE_KHR: Result = Result(-1000001004i32 as u32);
    pub const ERROR_NATIVE_WINDOW_IN_USE_KHR: Result = Result(-1000000001i32 as u32);
    pub const ERROR_SURFACE_LOST_KHR: Result = Result(-1000000000i32 as u32);
    pub const ERROR_FRAGMENTED_POOL: Result = Result(-12i32 as u32);
    pub const ERROR_FORMAT_NOT_SUPPORTED: Result = Result(-11i32 as u32);
    pub const ERROR_TOO_MANY_OBJECTS: Result = Result(-10i32 as u32);
    pub const ERROR_INCOMPATIBLE_DRIVER: Result = Result(-9i32 as u32);
    pub const ERROR_FEATURE_NOT_PRESENT: Result = Result(-8i32 as u32);
    pub const ERROR_EXTENSION_NOT_PRESENT: Result = Result(-7i32 as u32);
    pub const ERROR_LAYER_NOT_PRESENT: Result = Result(-6i32 as u32);
    pub const ERROR_MEMORY_MAP_FAILED: Result = Result(-5i32 as u32);
    pub const ERROR_DEVICE_LOST: Result = Result(-4i32 as u32);
    pub const ERROR_INITIALIZATION_FAILED: Result = Result(-3i32 as u32);
    pub const ERROR_OUT_OF_DEVICE_MEMORY: Result = Result(-2i32 as u32);
    pub const ERROR_OUT_OF_HOST_MEMORY: Result = Result(-1i32 as u32);
    pub const SUCCESS: Result = Result(0);
    pub const NOT_READY: Result = Result(1);
    pub const TIMEOUT: Result = Result(2);
    pub const EVENT_SET: Result = Result(3);
    pub const EVENT_RESET: Result = Result(4);
    pub const INCOMPLETE: Result = Result(5);
    pub const SUBOPTIMAL_KHR: Result = Result(1000001003);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct PresentModeKHR(u32);
impl PresentModeKHR {
    pub const IMMEDIATE_KHR: PresentModeKHR = PresentModeKHR(0);
    pub const MAILBOX_KHR: PresentModeKHR = PresentModeKHR(1);
    pub const FIFO_KHR: PresentModeKHR = PresentModeKHR(2);
    pub const FIFO_RELAXED_KHR: PresentModeKHR = PresentModeKHR(3);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct ColorSpaceKHR(u32);
impl ColorSpaceKHR {
    pub const SRGB_NONLINEAR_KHR: ColorSpaceKHR = ColorSpaceKHR(0);
    pub const COLORSPACE_SRGB_NONLINEAR_KHR: ColorSpaceKHR = ColorSpaceKHR(0);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct Format(u32);
impl Format {
    pub const UNDEFINED: Format = Format(0);
    pub const R4G4_UNORM_PACK8: Format = Format(1);
    pub const R4G4B4A4_UNORM_PACK16: Format = Format(2);
    pub const B4G4R4A4_UNORM_PACK16: Format = Format(3);
    pub const R5G6B5_UNORM_PACK16: Format = Format(4);
    pub const B5G6R5_UNORM_PACK16: Format = Format(5);
    pub const R5G5B5A1_UNORM_PACK16: Format = Format(6);
    pub const B5G5R5A1_UNORM_PACK16: Format = Format(7);
    pub const A1R5G5B5_UNORM_PACK16: Format = Format(8);
    pub const R8_UNORM: Format = Format(9);
    pub const R8_SNORM: Format = Format(10);
    pub const R8_USCALED: Format = Format(11);
    pub const R8_SSCALED: Format = Format(12);
    pub const R8_UINT: Format = Format(13);
    pub const R8_SINT: Format = Format(14);
    pub const R8_SRGB: Format = Format(15);
    pub const R8G8_UNORM: Format = Format(16);
    pub const R8G8_SNORM: Format = Format(17);
    pub const R8G8_USCALED: Format = Format(18);
    pub const R8G8_SSCALED: Format = Format(19);
    pub const R8G8_UINT: Format = Format(20);
    pub const R8G8_SINT: Format = Format(21);
    pub const R8G8_SRGB: Format = Format(22);
    pub const R8G8B8_UNORM: Format = Format(23);
    pub const R8G8B8_SNORM: Format = Format(24);
    pub const R8G8B8_USCALED: Format = Format(25);
    pub const R8G8B8_SSCALED: Format = Format(26);
    pub const R8G8B8_UINT: Format = Format(27);
    pub const R8G8B8_SINT: Format = Format(28);
    pub const R8G8B8_SRGB: Format = Format(29);
    pub const B8G8R8_UNORM: Format = Format(30);
    pub const B8G8R8_SNORM: Format = Format(31);
    pub const B8G8R8_USCALED: Format = Format(32);
    pub const B8G8R8_SSCALED: Format = Format(33);
    pub const B8G8R8_UINT: Format = Format(34);
    pub const B8G8R8_SINT: Format = Format(35);
    pub const B8G8R8_SRGB: Format = Format(36);
    pub const R8G8B8A8_UNORM: Format = Format(37);
    pub const R8G8B8A8_SNORM: Format = Format(38);
    pub const R8G8B8A8_USCALED: Format = Format(39);
    pub const R8G8B8A8_SSCALED: Format = Format(40);
    pub const R8G8B8A8_UINT: Format = Format(41);
    pub const R8G8B8A8_SINT: Format = Format(42);
    pub const R8G8B8A8_SRGB: Format = Format(43);
    pub const B8G8R8A8_UNORM: Format = Format(44);
    pub const B8G8R8A8_SNORM: Format = Format(45);
    pub const B8G8R8A8_USCALED: Format = Format(46);
    pub const B8G8R8A8_SSCALED: Format = Format(47);
    pub const B8G8R8A8_UINT: Format = Format(48);
    pub const B8G8R8A8_SINT: Format = Format(49);
    pub const B8G8R8A8_SRGB: Format = Format(50);
    pub const A8B8G8R8_UNORM_PACK32: Format = Format(51);
    pub const A8B8G8R8_SNORM_PACK32: Format = Format(52);
    pub const A8B8G8R8_USCALED_PACK32: Format = Format(53);
    pub const A8B8G8R8_SSCALED_PACK32: Format = Format(54);
    pub const A8B8G8R8_UINT_PACK32: Format = Format(55);
    pub const A8B8G8R8_SINT_PACK32: Format = Format(56);
    pub const A8B8G8R8_SRGB_PACK32: Format = Format(57);
    pub const A2R10G10B10_UNORM_PACK32: Format = Format(58);
    pub const A2R10G10B10_SNORM_PACK32: Format = Format(59);
    pub const A2R10G10B10_USCALED_PACK32: Format = Format(60);
    pub const A2R10G10B10_SSCALED_PACK32: Format = Format(61);
    pub const A2R10G10B10_UINT_PACK32: Format = Format(62);
    pub const A2R10G10B10_SINT_PACK32: Format = Format(63);
    pub const A2B10G10R10_UNORM_PACK32: Format = Format(64);
    pub const A2B10G10R10_SNORM_PACK32: Format = Format(65);
    pub const A2B10G10R10_USCALED_PACK32: Format = Format(66);
    pub const A2B10G10R10_SSCALED_PACK32: Format = Format(67);
    pub const A2B10G10R10_UINT_PACK32: Format = Format(68);
    pub const A2B10G10R10_SINT_PACK32: Format = Format(69);
    pub const R16_UNORM: Format = Format(70);
    pub const R16_SNORM: Format = Format(71);
    pub const R16_USCALED: Format = Format(72);
    pub const R16_SSCALED: Format = Format(73);
    pub const R16_UINT: Format = Format(74);
    pub const R16_SINT: Format = Format(75);
    pub const R16_SFLOAT: Format = Format(76);
    pub const R16G16_UNORM: Format = Format(77);
    pub const R16G16_SNORM: Format = Format(78);
    pub const R16G16_USCALED: Format = Format(79);
    pub const R16G16_SSCALED: Format = Format(80);
    pub const R16G16_UINT: Format = Format(81);
    pub const R16G16_SINT: Format = Format(82);
    pub const R16G16_SFLOAT: Format = Format(83);
    pub const R16G16B16_UNORM: Format = Format(84);
    pub const R16G16B16_SNORM: Format = Format(85);
    pub const R16G16B16_USCALED: Format = Format(86);
    pub const R16G16B16_SSCALED: Format = Format(87);
    pub const R16G16B16_UINT: Format = Format(88);
    pub const R16G16B16_SINT: Format = Format(89);
    pub const R16G16B16_SFLOAT: Format = Format(90);
    pub const R16G16B16A16_UNORM: Format = Format(91);
    pub const R16G16B16A16_SNORM: Format = Format(92);
    pub const R16G16B16A16_USCALED: Format = Format(93);
    pub const R16G16B16A16_SSCALED: Format = Format(94);
    pub const R16G16B16A16_UINT: Format = Format(95);
    pub const R16G16B16A16_SINT: Format = Format(96);
    pub const R16G16B16A16_SFLOAT: Format = Format(97);
    pub const R32_UINT: Format = Format(98);
    pub const R32_SINT: Format = Format(99);
    pub const R32_SFLOAT: Format = Format(100);
    pub const R32G32_UINT: Format = Format(101);
    pub const R32G32_SINT: Format = Format(102);
    pub const R32G32_SFLOAT: Format = Format(103);
    pub const R32G32B32_UINT: Format = Format(104);
    pub const R32G32B32_SINT: Format = Format(105);
    pub const R32G32B32_SFLOAT: Format = Format(106);
    pub const R32G32B32A32_UINT: Format = Format(107);
    pub const R32G32B32A32_SINT: Format = Format(108);
    pub const R32G32B32A32_SFLOAT: Format = Format(109);
    pub const R64_UINT: Format = Format(110);
    pub const R64_SINT: Format = Format(111);
    pub const R64_SFLOAT: Format = Format(112);
    pub const R64G64_UINT: Format = Format(113);
    pub const R64G64_SINT: Format = Format(114);
    pub const R64G64_SFLOAT: Format = Format(115);
    pub const R64G64B64_UINT: Format = Format(116);
    pub const R64G64B64_SINT: Format = Format(117);
    pub const R64G64B64_SFLOAT: Format = Format(118);
    pub const R64G64B64A64_UINT: Format = Format(119);
    pub const R64G64B64A64_SINT: Format = Format(120);
    pub const R64G64B64A64_SFLOAT: Format = Format(121);
    pub const B10G11R11_UFLOAT_PACK32: Format = Format(122);
    pub const E5B9G9R9_UFLOAT_PACK32: Format = Format(123);
    pub const D16_UNORM: Format = Format(124);
    pub const X8_D24_UNORM_PACK32: Format = Format(125);
    pub const D32_SFLOAT: Format = Format(126);
    pub const S8_UINT: Format = Format(127);
    pub const D16_UNORM_S8_UINT: Format = Format(128);
    pub const D24_UNORM_S8_UINT: Format = Format(129);
    pub const D32_SFLOAT_S8_UINT: Format = Format(130);
    pub const BC1_RGB_UNORM_BLOCK: Format = Format(131);
    pub const BC1_RGB_SRGB_BLOCK: Format = Format(132);
    pub const BC1_RGBA_UNORM_BLOCK: Format = Format(133);
    pub const BC1_RGBA_SRGB_BLOCK: Format = Format(134);
    pub const BC2_UNORM_BLOCK: Format = Format(135);
    pub const BC2_SRGB_BLOCK: Format = Format(136);
    pub const BC3_UNORM_BLOCK: Format = Format(137);
    pub const BC3_SRGB_BLOCK: Format = Format(138);
    pub const BC4_UNORM_BLOCK: Format = Format(139);
    pub const BC4_SNORM_BLOCK: Format = Format(140);
    pub const BC5_UNORM_BLOCK: Format = Format(141);
    pub const BC5_SNORM_BLOCK: Format = Format(142);
    pub const BC6H_UFLOAT_BLOCK: Format = Format(143);
    pub const BC6H_SFLOAT_BLOCK: Format = Format(144);
    pub const BC7_UNORM_BLOCK: Format = Format(145);
    pub const BC7_SRGB_BLOCK: Format = Format(146);
    pub const ETC2_R8G8B8_UNORM_BLOCK: Format = Format(147);
    pub const ETC2_R8G8B8_SRGB_BLOCK: Format = Format(148);
    pub const ETC2_R8G8B8A1_UNORM_BLOCK: Format = Format(149);
    pub const ETC2_R8G8B8A1_SRGB_BLOCK: Format = Format(150);
    pub const ETC2_R8G8B8A8_UNORM_BLOCK: Format = Format(151);
    pub const ETC2_R8G8B8A8_SRGB_BLOCK: Format = Format(152);
    pub const EAC_R11_UNORM_BLOCK: Format = Format(153);
    pub const EAC_R11_SNORM_BLOCK: Format = Format(154);
    pub const EAC_R11G11_UNORM_BLOCK: Format = Format(155);
    pub const EAC_R11G11_SNORM_BLOCK: Format = Format(156);
    pub const ASTC_4X4_UNORM_BLOCK: Format = Format(157);
    pub const ASTC_4X4_SRGB_BLOCK: Format = Format(158);
    pub const ASTC_5X4_UNORM_BLOCK: Format = Format(159);
    pub const ASTC_5X4_SRGB_BLOCK: Format = Format(160);
    pub const ASTC_5X5_UNORM_BLOCK: Format = Format(161);
    pub const ASTC_5X5_SRGB_BLOCK: Format = Format(162);
    pub const ASTC_6X5_UNORM_BLOCK: Format = Format(163);
    pub const ASTC_6X5_SRGB_BLOCK: Format = Format(164);
    pub const ASTC_6X6_UNORM_BLOCK: Format = Format(165);
    pub const ASTC_6X6_SRGB_BLOCK: Format = Format(166);
    pub const ASTC_8X5_UNORM_BLOCK: Format = Format(167);
    pub const ASTC_8X5_SRGB_BLOCK: Format = Format(168);
    pub const ASTC_8X6_UNORM_BLOCK: Format = Format(169);
    pub const ASTC_8X6_SRGB_BLOCK: Format = Format(170);
    pub const ASTC_8X8_UNORM_BLOCK: Format = Format(171);
    pub const ASTC_8X8_SRGB_BLOCK: Format = Format(172);
    pub const ASTC_10X5_UNORM_BLOCK: Format = Format(173);
    pub const ASTC_10X5_SRGB_BLOCK: Format = Format(174);
    pub const ASTC_10X6_UNORM_BLOCK: Format = Format(175);
    pub const ASTC_10X6_SRGB_BLOCK: Format = Format(176);
    pub const ASTC_10X8_UNORM_BLOCK: Format = Format(177);
    pub const ASTC_10X8_SRGB_BLOCK: Format = Format(178);
    pub const ASTC_10X10_UNORM_BLOCK: Format = Format(179);
    pub const ASTC_10X10_SRGB_BLOCK: Format = Format(180);
    pub const ASTC_12X10_UNORM_BLOCK: Format = Format(181);
    pub const ASTC_12X10_SRGB_BLOCK: Format = Format(182);
    pub const ASTC_12X12_UNORM_BLOCK: Format = Format(183);
    pub const ASTC_12X12_SRGB_BLOCK: Format = Format(184);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct ObjectType(u32);
impl ObjectType {
    pub const UNKNOWN: ObjectType = ObjectType(0);
    pub const INSTANCE: ObjectType = ObjectType(1);
    pub const PHYSICAL_DEVICE: ObjectType = ObjectType(2);
    pub const DEVICE: ObjectType = ObjectType(3);
    pub const QUEUE: ObjectType = ObjectType(4);
    pub const SEMAPHORE: ObjectType = ObjectType(5);
    pub const COMMAND_BUFFER: ObjectType = ObjectType(6);
    pub const FENCE: ObjectType = ObjectType(7);
    pub const DEVICE_MEMORY: ObjectType = ObjectType(8);
    pub const BUFFER: ObjectType = ObjectType(9);
    pub const IMAGE: ObjectType = ObjectType(10);
    pub const EVENT: ObjectType = ObjectType(11);
    pub const QUERY_POOL: ObjectType = ObjectType(12);
    pub const BUFFER_VIEW: ObjectType = ObjectType(13);
    pub const IMAGE_VIEW: ObjectType = ObjectType(14);
    pub const SHADER_MODULE: ObjectType = ObjectType(15);
    pub const PIPELINE_CACHE: ObjectType = ObjectType(16);
    pub const PIPELINE_LAYOUT: ObjectType = ObjectType(17);
    pub const RENDER_PASS: ObjectType = ObjectType(18);
    pub const PIPELINE: ObjectType = ObjectType(19);
    pub const DESCRIPTOR_SET_LAYOUT: ObjectType = ObjectType(20);
    pub const SAMPLER: ObjectType = ObjectType(21);
    pub const DESCRIPTOR_POOL: ObjectType = ObjectType(22);
    pub const DESCRIPTOR_SET: ObjectType = ObjectType(23);
    pub const FRAMEBUFFER: ObjectType = ObjectType(24);
    pub const COMMAND_POOL: ObjectType = ObjectType(25);
    pub const SURFACE_KHR: ObjectType = ObjectType(1000000000);
    pub const SWAPCHAIN_KHR: ObjectType = ObjectType(1000001000);
    pub const DEBUG_UTILS_MESSENGER_EXT: ObjectType = ObjectType(1000128000);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct SharingMode(u32);
impl SharingMode {
    pub const EXCLUSIVE: SharingMode = SharingMode(0);
    pub const CONCURRENT: SharingMode = SharingMode(1);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct ImageLayout(u32);
impl ImageLayout {
    pub const UNDEFINED: ImageLayout = ImageLayout(0);
    pub const GENERAL: ImageLayout = ImageLayout(1);
    pub const COLOR_ATTACHMENT_OPTIMAL: ImageLayout = ImageLayout(2);
    pub const DEPTH_STENCIL_ATTACHMENT_OPTIMAL: ImageLayout = ImageLayout(3);
    pub const DEPTH_STENCIL_READ_ONLY_OPTIMAL: ImageLayout = ImageLayout(4);
    pub const SHADER_READ_ONLY_OPTIMAL: ImageLayout = ImageLayout(5);
    pub const TRANSFER_SRC_OPTIMAL: ImageLayout = ImageLayout(6);
    pub const TRANSFER_DST_OPTIMAL: ImageLayout = ImageLayout(7);
    pub const PREINITIALIZED: ImageLayout = ImageLayout(8);
    pub const PRESENT_SRC_KHR: ImageLayout = ImageLayout(1000001002);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VendorId(u32);
impl VendorId {
    pub const VIV: VendorId = VendorId(65537);
    pub const VSI: VendorId = VendorId(65538);
    pub const KAZAN: VendorId = VendorId(65539);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct SubpassContents(u32);
impl SubpassContents {
    pub const INLINE: SubpassContents = SubpassContents(0);
    pub const SECONDARY_COMMAND_BUFFERS: SubpassContents = SubpassContents(1);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct Filter(u32);
impl Filter {
    pub const NEAREST: Filter = Filter(0);
    pub const LINEAR: Filter = Filter(1);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct IndexType(u32);
impl IndexType {
    pub const UINT16: IndexType = IndexType(0);
    pub const UINT32: IndexType = IndexType(1);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct PipelineBindPoint(u32);
impl PipelineBindPoint {
    pub const GRAPHICS: PipelineBindPoint = PipelineBindPoint(0);
    pub const COMPUTE: PipelineBindPoint = PipelineBindPoint(1);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct CommandBufferLevel(u32);
impl CommandBufferLevel {
    pub const PRIMARY: CommandBufferLevel = CommandBufferLevel(0);
    pub const SECONDARY: CommandBufferLevel = CommandBufferLevel(1);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct AttachmentStoreOp(u32);
impl AttachmentStoreOp {
    pub const STORE: AttachmentStoreOp = AttachmentStoreOp(0);
    pub const DONT_CARE: AttachmentStoreOp = AttachmentStoreOp(1);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct AttachmentLoadOp(u32);
impl AttachmentLoadOp {
    pub const LOAD: AttachmentLoadOp = AttachmentLoadOp(0);
    pub const CLEAR: AttachmentLoadOp = AttachmentLoadOp(1);
    pub const DONT_CARE: AttachmentLoadOp = AttachmentLoadOp(2);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct DescriptorType(u32);
impl DescriptorType {
    pub const SAMPLER: DescriptorType = DescriptorType(0);
    pub const COMBINED_IMAGE_SAMPLER: DescriptorType = DescriptorType(1);
    pub const SAMPLED_IMAGE: DescriptorType = DescriptorType(2);
    pub const STORAGE_IMAGE: DescriptorType = DescriptorType(3);
    pub const UNIFORM_TEXEL_BUFFER: DescriptorType = DescriptorType(4);
    pub const STORAGE_TEXEL_BUFFER: DescriptorType = DescriptorType(5);
    pub const UNIFORM_BUFFER: DescriptorType = DescriptorType(6);
    pub const STORAGE_BUFFER: DescriptorType = DescriptorType(7);
    pub const UNIFORM_BUFFER_DYNAMIC: DescriptorType = DescriptorType(8);
    pub const STORAGE_BUFFER_DYNAMIC: DescriptorType = DescriptorType(9);
    pub const INPUT_ATTACHMENT: DescriptorType = DescriptorType(10);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct BorderColor(u32);
impl BorderColor {
    pub const FLOAT_TRANSPARENT_BLACK: BorderColor = BorderColor(0);
    pub const INT_TRANSPARENT_BLACK: BorderColor = BorderColor(1);
    pub const FLOAT_OPAQUE_BLACK: BorderColor = BorderColor(2);
    pub const INT_OPAQUE_BLACK: BorderColor = BorderColor(3);
    pub const FLOAT_OPAQUE_WHITE: BorderColor = BorderColor(4);
    pub const INT_OPAQUE_WHITE: BorderColor = BorderColor(5);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct CompareOp(u32);
impl CompareOp {
    pub const NEVER: CompareOp = CompareOp(0);
    pub const LESS: CompareOp = CompareOp(1);
    pub const EQUAL: CompareOp = CompareOp(2);
    pub const LESS_OR_EQUAL: CompareOp = CompareOp(3);
    pub const GREATER: CompareOp = CompareOp(4);
    pub const NOT_EQUAL: CompareOp = CompareOp(5);
    pub const GREATER_OR_EQUAL: CompareOp = CompareOp(6);
    pub const ALWAYS: CompareOp = CompareOp(7);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct SamplerAddressMode(u32);
impl SamplerAddressMode {
    pub const REPEAT: SamplerAddressMode = SamplerAddressMode(0);
    pub const MIRRORED_REPEAT: SamplerAddressMode = SamplerAddressMode(1);
    pub const CLAMP_TO_EDGE: SamplerAddressMode = SamplerAddressMode(2);
    pub const CLAMP_TO_BORDER: SamplerAddressMode = SamplerAddressMode(3);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct SamplerMipmapMode(u32);
impl SamplerMipmapMode {
    pub const NEAREST: SamplerMipmapMode = SamplerMipmapMode(0);
    pub const LINEAR: SamplerMipmapMode = SamplerMipmapMode(1);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct DynamicState(u32);
impl DynamicState {
    pub const VIEWPORT: DynamicState = DynamicState(0);
    pub const SCISSOR: DynamicState = DynamicState(1);
    pub const LINE_WIDTH: DynamicState = DynamicState(2);
    pub const DEPTH_BIAS: DynamicState = DynamicState(3);
    pub const BLEND_CONSTANTS: DynamicState = DynamicState(4);
    pub const DEPTH_BOUNDS: DynamicState = DynamicState(5);
    pub const STENCIL_COMPARE_MASK: DynamicState = DynamicState(6);
    pub const STENCIL_WRITE_MASK: DynamicState = DynamicState(7);
    pub const STENCIL_REFERENCE: DynamicState = DynamicState(8);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct BlendOp(u32);
impl BlendOp {
    pub const ADD: BlendOp = BlendOp(0);
    pub const SUBTRACT: BlendOp = BlendOp(1);
    pub const REVERSE_SUBTRACT: BlendOp = BlendOp(2);
    pub const MIN: BlendOp = BlendOp(3);
    pub const MAX: BlendOp = BlendOp(4);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct BlendFactor(u32);
impl BlendFactor {
    pub const ZERO: BlendFactor = BlendFactor(0);
    pub const ONE: BlendFactor = BlendFactor(1);
    pub const SRC_COLOR: BlendFactor = BlendFactor(2);
    pub const ONE_MINUS_SRC_COLOR: BlendFactor = BlendFactor(3);
    pub const DST_COLOR: BlendFactor = BlendFactor(4);
    pub const ONE_MINUS_DST_COLOR: BlendFactor = BlendFactor(5);
    pub const SRC_ALPHA: BlendFactor = BlendFactor(6);
    pub const ONE_MINUS_SRC_ALPHA: BlendFactor = BlendFactor(7);
    pub const DST_ALPHA: BlendFactor = BlendFactor(8);
    pub const ONE_MINUS_DST_ALPHA: BlendFactor = BlendFactor(9);
    pub const CONSTANT_COLOR: BlendFactor = BlendFactor(10);
    pub const ONE_MINUS_CONSTANT_COLOR: BlendFactor = BlendFactor(11);
    pub const CONSTANT_ALPHA: BlendFactor = BlendFactor(12);
    pub const ONE_MINUS_CONSTANT_ALPHA: BlendFactor = BlendFactor(13);
    pub const SRC_ALPHA_SATURATE: BlendFactor = BlendFactor(14);
    pub const SRC1_COLOR: BlendFactor = BlendFactor(15);
    pub const ONE_MINUS_SRC1_COLOR: BlendFactor = BlendFactor(16);
    pub const SRC1_ALPHA: BlendFactor = BlendFactor(17);
    pub const ONE_MINUS_SRC1_ALPHA: BlendFactor = BlendFactor(18);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct LogicOp(u32);
impl LogicOp {
    pub const CLEAR: LogicOp = LogicOp(0);
    pub const AND: LogicOp = LogicOp(1);
    pub const AND_REVERSE: LogicOp = LogicOp(2);
    pub const COPY: LogicOp = LogicOp(3);
    pub const AND_INVERTED: LogicOp = LogicOp(4);
    pub const NO_OP: LogicOp = LogicOp(5);
    pub const XOR: LogicOp = LogicOp(6);
    pub const OR: LogicOp = LogicOp(7);
    pub const NOR: LogicOp = LogicOp(8);
    pub const EQUIVALENT: LogicOp = LogicOp(9);
    pub const INVERT: LogicOp = LogicOp(10);
    pub const OR_REVERSE: LogicOp = LogicOp(11);
    pub const COPY_INVERTED: LogicOp = LogicOp(12);
    pub const OR_INVERTED: LogicOp = LogicOp(13);
    pub const NAND: LogicOp = LogicOp(14);
    pub const SET: LogicOp = LogicOp(15);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct StencilOp(u32);
impl StencilOp {
    pub const KEEP: StencilOp = StencilOp(0);
    pub const ZERO: StencilOp = StencilOp(1);
    pub const REPLACE: StencilOp = StencilOp(2);
    pub const INCREMENT_AND_CLAMP: StencilOp = StencilOp(3);
    pub const DECREMENT_AND_CLAMP: StencilOp = StencilOp(4);
    pub const INVERT: StencilOp = StencilOp(5);
    pub const INCREMENT_AND_WRAP: StencilOp = StencilOp(6);
    pub const DECREMENT_AND_WRAP: StencilOp = StencilOp(7);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct FrontFace(u32);
impl FrontFace {
    pub const COUNTER_CLOCKWISE: FrontFace = FrontFace(0);
    pub const CLOCKWISE: FrontFace = FrontFace(1);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct PolygonMode(u32);
impl PolygonMode {
    pub const FILL: PolygonMode = PolygonMode(0);
    pub const LINE: PolygonMode = PolygonMode(1);
    pub const POINT: PolygonMode = PolygonMode(2);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct PrimitiveTopology(u32);
impl PrimitiveTopology {
    pub const POINT_LIST: PrimitiveTopology = PrimitiveTopology(0);
    pub const LINE_LIST: PrimitiveTopology = PrimitiveTopology(1);
    pub const LINE_STRIP: PrimitiveTopology = PrimitiveTopology(2);
    pub const TRIANGLE_LIST: PrimitiveTopology = PrimitiveTopology(3);
    pub const TRIANGLE_STRIP: PrimitiveTopology = PrimitiveTopology(4);
    pub const TRIANGLE_FAN: PrimitiveTopology = PrimitiveTopology(5);
    pub const LINE_LIST_WITH_ADJACENCY: PrimitiveTopology = PrimitiveTopology(6);
    pub const LINE_STRIP_WITH_ADJACENCY: PrimitiveTopology = PrimitiveTopology(7);
    pub const TRIANGLE_LIST_WITH_ADJACENCY: PrimitiveTopology = PrimitiveTopology(8);
    pub const TRIANGLE_STRIP_WITH_ADJACENCY: PrimitiveTopology = PrimitiveTopology(9);
    pub const PATCH_LIST: PrimitiveTopology = PrimitiveTopology(10);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct VertexInputRate(u32);
impl VertexInputRate {
    pub const VERTEX: VertexInputRate = VertexInputRate(0);
    pub const INSTANCE: VertexInputRate = VertexInputRate(1);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct ComponentSwizzle(u32);
impl ComponentSwizzle {
    pub const IDENTITY: ComponentSwizzle = ComponentSwizzle(0);
    pub const ZERO: ComponentSwizzle = ComponentSwizzle(1);
    pub const ONE: ComponentSwizzle = ComponentSwizzle(2);
    pub const R: ComponentSwizzle = ComponentSwizzle(3);
    pub const G: ComponentSwizzle = ComponentSwizzle(4);
    pub const B: ComponentSwizzle = ComponentSwizzle(5);
    pub const A: ComponentSwizzle = ComponentSwizzle(6);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct ImageViewType(u32);
impl ImageViewType {
    pub const K_1D: ImageViewType = ImageViewType(0);
    pub const K_2D: ImageViewType = ImageViewType(1);
    pub const K_3D: ImageViewType = ImageViewType(2);
    pub const CUBE: ImageViewType = ImageViewType(3);
    pub const K_1D_ARRAY: ImageViewType = ImageViewType(4);
    pub const K_2D_ARRAY: ImageViewType = ImageViewType(5);
    pub const CUBE_ARRAY: ImageViewType = ImageViewType(6);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct ImageTiling(u32);
impl ImageTiling {
    pub const OPTIMAL: ImageTiling = ImageTiling(0);
    pub const LINEAR: ImageTiling = ImageTiling(1);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct ImageType(u32);
impl ImageType {
    pub const K_1D: ImageType = ImageType(0);
    pub const K_2D: ImageType = ImageType(1);
    pub const K_3D: ImageType = ImageType(2);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct QueryType(u32);
impl QueryType {
    pub const OCCLUSION: QueryType = QueryType(0);
    pub const PIPELINE_STATISTICS: QueryType = QueryType(1);
    pub const TIMESTAMP: QueryType = QueryType(2);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct PhysicalDeviceType(u32);
impl PhysicalDeviceType {
    pub const OTHER: PhysicalDeviceType = PhysicalDeviceType(0);
    pub const INTEGRATED_GPU: PhysicalDeviceType = PhysicalDeviceType(1);
    pub const DISCRETE_GPU: PhysicalDeviceType = PhysicalDeviceType(2);
    pub const VIRTUAL_GPU: PhysicalDeviceType = PhysicalDeviceType(3);
    pub const CPU: PhysicalDeviceType = PhysicalDeviceType(4);
}

#[repr(transparent)]
#[derive(PartialOrd, Copy, Clone, Ord, PartialEq, Eq, Hash)]
pub struct PipelineCacheHeaderVersion(u32);
impl PipelineCacheHeaderVersion {
    pub const ONE: PipelineCacheHeaderVersion = PipelineCacheHeaderVersion(1);
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ImageUsageFlagBits(Flags);
impl ImageUsageFlagBits {
    pub const TRANSFER_SRC_BIT: ImageUsageFlagBits = ImageUsageFlagBits(1);
    pub const TRANSFER_DST_BIT: ImageUsageFlagBits = ImageUsageFlagBits(2);
    pub const SAMPLED_BIT: ImageUsageFlagBits = ImageUsageFlagBits(4);
    pub const STORAGE_BIT: ImageUsageFlagBits = ImageUsageFlagBits(8);
    pub const COLOR_ATTACHMENT_BIT: ImageUsageFlagBits = ImageUsageFlagBits(16);
    pub const DEPTH_STENCIL_ATTACHMENT_BIT: ImageUsageFlagBits = ImageUsageFlagBits(32);
    pub const TRANSIENT_ATTACHMENT_BIT: ImageUsageFlagBits = ImageUsageFlagBits(64);
    pub const INPUT_ATTACHMENT_BIT: ImageUsageFlagBits = ImageUsageFlagBits(128);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for ImageUsageFlagBits {
    type Output = ImageUsageFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for ImageUsageFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for ImageUsageFlagBits {
    type Output = ImageUsageFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for ImageUsageFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for ImageUsageFlagBits {
    type Output = ImageUsageFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for ImageUsageFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct CompositeAlphaFlagBitsKHR(Flags);
impl CompositeAlphaFlagBitsKHR {
    pub const OPAQUE_BIT_KHR: CompositeAlphaFlagBitsKHR = CompositeAlphaFlagBitsKHR(1);
    pub const PRE_MULTIPLIED_BIT_KHR: CompositeAlphaFlagBitsKHR = CompositeAlphaFlagBitsKHR(2);
    pub const POST_MULTIPLIED_BIT_KHR: CompositeAlphaFlagBitsKHR = CompositeAlphaFlagBitsKHR(4);
    pub const INHERIT_BIT_KHR: CompositeAlphaFlagBitsKHR = CompositeAlphaFlagBitsKHR(8);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for CompositeAlphaFlagBitsKHR {
    type Output = CompositeAlphaFlagBitsKHR;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for CompositeAlphaFlagBitsKHR {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for CompositeAlphaFlagBitsKHR {
    type Output = CompositeAlphaFlagBitsKHR;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for CompositeAlphaFlagBitsKHR {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for CompositeAlphaFlagBitsKHR {
    type Output = CompositeAlphaFlagBitsKHR;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for CompositeAlphaFlagBitsKHR {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct SurfaceTransformFlagBitsKHR(Flags);
impl SurfaceTransformFlagBitsKHR {
    pub const IDENTITY_BIT_KHR: SurfaceTransformFlagBitsKHR = SurfaceTransformFlagBitsKHR(1);
    pub const ROTATE_90_BIT_KHR: SurfaceTransformFlagBitsKHR = SurfaceTransformFlagBitsKHR(2);
    pub const ROTATE_180_BIT_KHR: SurfaceTransformFlagBitsKHR = SurfaceTransformFlagBitsKHR(4);
    pub const ROTATE_270_BIT_KHR: SurfaceTransformFlagBitsKHR = SurfaceTransformFlagBitsKHR(8);
    pub const HORIZONTAL_MIRROR_BIT_KHR: SurfaceTransformFlagBitsKHR = SurfaceTransformFlagBitsKHR(16);
    pub const HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR: SurfaceTransformFlagBitsKHR = SurfaceTransformFlagBitsKHR(32);
    pub const HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR: SurfaceTransformFlagBitsKHR = SurfaceTransformFlagBitsKHR(64);
    pub const HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR: SurfaceTransformFlagBitsKHR = SurfaceTransformFlagBitsKHR(128);
    pub const INHERIT_BIT_KHR: SurfaceTransformFlagBitsKHR = SurfaceTransformFlagBitsKHR(256);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for SurfaceTransformFlagBitsKHR {
    type Output = SurfaceTransformFlagBitsKHR;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for SurfaceTransformFlagBitsKHR {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for SurfaceTransformFlagBitsKHR {
    type Output = SurfaceTransformFlagBitsKHR;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for SurfaceTransformFlagBitsKHR {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for SurfaceTransformFlagBitsKHR {
    type Output = SurfaceTransformFlagBitsKHR;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for SurfaceTransformFlagBitsKHR {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct SwapchainCreateFlagBitsKHR(Flags);
impl SwapchainCreateFlagBitsKHR {

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for SwapchainCreateFlagBitsKHR {
    type Output = SwapchainCreateFlagBitsKHR;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for SwapchainCreateFlagBitsKHR {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for SwapchainCreateFlagBitsKHR {
    type Output = SwapchainCreateFlagBitsKHR;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for SwapchainCreateFlagBitsKHR {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for SwapchainCreateFlagBitsKHR {
    type Output = SwapchainCreateFlagBitsKHR;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for SwapchainCreateFlagBitsKHR {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct DebugUtilsMessageTypeFlagBitsEXT(Flags);
impl DebugUtilsMessageTypeFlagBitsEXT {
    pub const GENERAL_BIT_EXT: DebugUtilsMessageTypeFlagBitsEXT = DebugUtilsMessageTypeFlagBitsEXT(1);
    pub const VALIDATION_BIT_EXT: DebugUtilsMessageTypeFlagBitsEXT = DebugUtilsMessageTypeFlagBitsEXT(2);
    pub const PERFORMANCE_BIT_EXT: DebugUtilsMessageTypeFlagBitsEXT = DebugUtilsMessageTypeFlagBitsEXT(4);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for DebugUtilsMessageTypeFlagBitsEXT {
    type Output = DebugUtilsMessageTypeFlagBitsEXT;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for DebugUtilsMessageTypeFlagBitsEXT {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for DebugUtilsMessageTypeFlagBitsEXT {
    type Output = DebugUtilsMessageTypeFlagBitsEXT;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for DebugUtilsMessageTypeFlagBitsEXT {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for DebugUtilsMessageTypeFlagBitsEXT {
    type Output = DebugUtilsMessageTypeFlagBitsEXT;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for DebugUtilsMessageTypeFlagBitsEXT {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct DebugUtilsMessageSeverityFlagBitsEXT(Flags);
impl DebugUtilsMessageSeverityFlagBitsEXT {
    pub const VERBOSE_BIT_EXT: DebugUtilsMessageSeverityFlagBitsEXT = DebugUtilsMessageSeverityFlagBitsEXT(1);
    pub const INFO_BIT_EXT: DebugUtilsMessageSeverityFlagBitsEXT = DebugUtilsMessageSeverityFlagBitsEXT(16);
    pub const WARNING_BIT_EXT: DebugUtilsMessageSeverityFlagBitsEXT = DebugUtilsMessageSeverityFlagBitsEXT(256);
    pub const ERROR_BIT_EXT: DebugUtilsMessageSeverityFlagBitsEXT = DebugUtilsMessageSeverityFlagBitsEXT(4096);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for DebugUtilsMessageSeverityFlagBitsEXT {
    type Output = DebugUtilsMessageSeverityFlagBitsEXT;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for DebugUtilsMessageSeverityFlagBitsEXT {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for DebugUtilsMessageSeverityFlagBitsEXT {
    type Output = DebugUtilsMessageSeverityFlagBitsEXT;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for DebugUtilsMessageSeverityFlagBitsEXT {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for DebugUtilsMessageSeverityFlagBitsEXT {
    type Output = DebugUtilsMessageSeverityFlagBitsEXT;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for DebugUtilsMessageSeverityFlagBitsEXT {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct AccessFlagBits(Flags);
impl AccessFlagBits {
    pub const INDIRECT_COMMAND_READ_BIT: AccessFlagBits = AccessFlagBits(1);
    pub const INDEX_READ_BIT: AccessFlagBits = AccessFlagBits(2);
    pub const VERTEX_ATTRIBUTE_READ_BIT: AccessFlagBits = AccessFlagBits(4);
    pub const UNIFORM_READ_BIT: AccessFlagBits = AccessFlagBits(8);
    pub const INPUT_ATTACHMENT_READ_BIT: AccessFlagBits = AccessFlagBits(16);
    pub const SHADER_READ_BIT: AccessFlagBits = AccessFlagBits(32);
    pub const SHADER_WRITE_BIT: AccessFlagBits = AccessFlagBits(64);
    pub const COLOR_ATTACHMENT_READ_BIT: AccessFlagBits = AccessFlagBits(128);
    pub const COLOR_ATTACHMENT_WRITE_BIT: AccessFlagBits = AccessFlagBits(256);
    pub const DEPTH_STENCIL_ATTACHMENT_READ_BIT: AccessFlagBits = AccessFlagBits(512);
    pub const DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: AccessFlagBits = AccessFlagBits(1024);
    pub const TRANSFER_READ_BIT: AccessFlagBits = AccessFlagBits(2048);
    pub const TRANSFER_WRITE_BIT: AccessFlagBits = AccessFlagBits(4096);
    pub const HOST_READ_BIT: AccessFlagBits = AccessFlagBits(8192);
    pub const HOST_WRITE_BIT: AccessFlagBits = AccessFlagBits(16384);
    pub const MEMORY_READ_BIT: AccessFlagBits = AccessFlagBits(32768);
    pub const MEMORY_WRITE_BIT: AccessFlagBits = AccessFlagBits(65536);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for AccessFlagBits {
    type Output = AccessFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for AccessFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for AccessFlagBits {
    type Output = AccessFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for AccessFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for AccessFlagBits {
    type Output = AccessFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for AccessFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ImageAspectFlagBits(Flags);
impl ImageAspectFlagBits {
    pub const COLOR_BIT: ImageAspectFlagBits = ImageAspectFlagBits(1);
    pub const DEPTH_BIT: ImageAspectFlagBits = ImageAspectFlagBits(2);
    pub const STENCIL_BIT: ImageAspectFlagBits = ImageAspectFlagBits(4);
    pub const METADATA_BIT: ImageAspectFlagBits = ImageAspectFlagBits(8);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for ImageAspectFlagBits {
    type Output = ImageAspectFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for ImageAspectFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for ImageAspectFlagBits {
    type Output = ImageAspectFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for ImageAspectFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for ImageAspectFlagBits {
    type Output = ImageAspectFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for ImageAspectFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ShaderStageFlagBits(Flags);
impl ShaderStageFlagBits {
    pub const VERTEX_BIT: ShaderStageFlagBits = ShaderStageFlagBits(1);
    pub const TESSELLATION_CONTROL_BIT: ShaderStageFlagBits = ShaderStageFlagBits(2);
    pub const TESSELLATION_EVALUATION_BIT: ShaderStageFlagBits = ShaderStageFlagBits(4);
    pub const GEOMETRY_BIT: ShaderStageFlagBits = ShaderStageFlagBits(8);
    pub const FRAGMENT_BIT: ShaderStageFlagBits = ShaderStageFlagBits(16);
    pub const ALL_GRAPHICS: ShaderStageFlagBits = ShaderStageFlagBits(31);
    pub const COMPUTE_BIT: ShaderStageFlagBits = ShaderStageFlagBits(32);
    pub const ALL: ShaderStageFlagBits = ShaderStageFlagBits(2147483647);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for ShaderStageFlagBits {
    type Output = ShaderStageFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for ShaderStageFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for ShaderStageFlagBits {
    type Output = ShaderStageFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for ShaderStageFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for ShaderStageFlagBits {
    type Output = ShaderStageFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for ShaderStageFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct QueryResultFlagBits(Flags);
impl QueryResultFlagBits {
    pub const K_64_BIT: QueryResultFlagBits = QueryResultFlagBits(1);
    pub const WAIT_BIT: QueryResultFlagBits = QueryResultFlagBits(2);
    pub const WITH_AVAILABILITY_BIT: QueryResultFlagBits = QueryResultFlagBits(4);
    pub const PARTIAL_BIT: QueryResultFlagBits = QueryResultFlagBits(8);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for QueryResultFlagBits {
    type Output = QueryResultFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for QueryResultFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for QueryResultFlagBits {
    type Output = QueryResultFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for QueryResultFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for QueryResultFlagBits {
    type Output = QueryResultFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for QueryResultFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct PipelineStageFlagBits(Flags);
impl PipelineStageFlagBits {
    pub const TOP_OF_PIPE_BIT: PipelineStageFlagBits = PipelineStageFlagBits(1);
    pub const DRAW_INDIRECT_BIT: PipelineStageFlagBits = PipelineStageFlagBits(2);
    pub const VERTEX_INPUT_BIT: PipelineStageFlagBits = PipelineStageFlagBits(4);
    pub const VERTEX_SHADER_BIT: PipelineStageFlagBits = PipelineStageFlagBits(8);
    pub const TESSELLATION_CONTROL_SHADER_BIT: PipelineStageFlagBits = PipelineStageFlagBits(16);
    pub const TESSELLATION_EVALUATION_SHADER_BIT: PipelineStageFlagBits = PipelineStageFlagBits(32);
    pub const GEOMETRY_SHADER_BIT: PipelineStageFlagBits = PipelineStageFlagBits(64);
    pub const FRAGMENT_SHADER_BIT: PipelineStageFlagBits = PipelineStageFlagBits(128);
    pub const EARLY_FRAGMENT_TESTS_BIT: PipelineStageFlagBits = PipelineStageFlagBits(256);
    pub const LATE_FRAGMENT_TESTS_BIT: PipelineStageFlagBits = PipelineStageFlagBits(512);
    pub const COLOR_ATTACHMENT_OUTPUT_BIT: PipelineStageFlagBits = PipelineStageFlagBits(1024);
    pub const COMPUTE_SHADER_BIT: PipelineStageFlagBits = PipelineStageFlagBits(2048);
    pub const TRANSFER_BIT: PipelineStageFlagBits = PipelineStageFlagBits(4096);
    pub const BOTTOM_OF_PIPE_BIT: PipelineStageFlagBits = PipelineStageFlagBits(8192);
    pub const HOST_BIT: PipelineStageFlagBits = PipelineStageFlagBits(16384);
    pub const ALL_GRAPHICS_BIT: PipelineStageFlagBits = PipelineStageFlagBits(32768);
    pub const ALL_COMMANDS_BIT: PipelineStageFlagBits = PipelineStageFlagBits(65536);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for PipelineStageFlagBits {
    type Output = PipelineStageFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for PipelineStageFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for PipelineStageFlagBits {
    type Output = PipelineStageFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for PipelineStageFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for PipelineStageFlagBits {
    type Output = PipelineStageFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for PipelineStageFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct QueryControlFlagBits(Flags);
impl QueryControlFlagBits {
    pub const PRECISE_BIT: QueryControlFlagBits = QueryControlFlagBits(1);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for QueryControlFlagBits {
    type Output = QueryControlFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for QueryControlFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for QueryControlFlagBits {
    type Output = QueryControlFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for QueryControlFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for QueryControlFlagBits {
    type Output = QueryControlFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for QueryControlFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct DependencyFlagBits(Flags);
impl DependencyFlagBits {
    pub const BY_REGION_BIT: DependencyFlagBits = DependencyFlagBits(1);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for DependencyFlagBits {
    type Output = DependencyFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for DependencyFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for DependencyFlagBits {
    type Output = DependencyFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for DependencyFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for DependencyFlagBits {
    type Output = DependencyFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for DependencyFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct StencilFaceFlagBits(Flags);
impl StencilFaceFlagBits {
    pub const FRONT_BIT: StencilFaceFlagBits = StencilFaceFlagBits(1);
    pub const BACK_BIT: StencilFaceFlagBits = StencilFaceFlagBits(2);
    pub const FRONT_AND_BACK: StencilFaceFlagBits = StencilFaceFlagBits(3);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for StencilFaceFlagBits {
    type Output = StencilFaceFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for StencilFaceFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for StencilFaceFlagBits {
    type Output = StencilFaceFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for StencilFaceFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for StencilFaceFlagBits {
    type Output = StencilFaceFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for StencilFaceFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct CommandBufferResetFlagBits(Flags);
impl CommandBufferResetFlagBits {
    pub const RELEASE_RESOURCES_BIT: CommandBufferResetFlagBits = CommandBufferResetFlagBits(1);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for CommandBufferResetFlagBits {
    type Output = CommandBufferResetFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for CommandBufferResetFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for CommandBufferResetFlagBits {
    type Output = CommandBufferResetFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for CommandBufferResetFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for CommandBufferResetFlagBits {
    type Output = CommandBufferResetFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for CommandBufferResetFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct QueryPipelineStatisticFlagBits(Flags);
impl QueryPipelineStatisticFlagBits {
    pub const INPUT_ASSEMBLY_VERTICES_BIT: QueryPipelineStatisticFlagBits = QueryPipelineStatisticFlagBits(1);
    pub const INPUT_ASSEMBLY_PRIMITIVES_BIT: QueryPipelineStatisticFlagBits = QueryPipelineStatisticFlagBits(2);
    pub const VERTEX_SHADER_INVOCATIONS_BIT: QueryPipelineStatisticFlagBits = QueryPipelineStatisticFlagBits(4);
    pub const GEOMETRY_SHADER_INVOCATIONS_BIT: QueryPipelineStatisticFlagBits = QueryPipelineStatisticFlagBits(8);
    pub const GEOMETRY_SHADER_PRIMITIVES_BIT: QueryPipelineStatisticFlagBits = QueryPipelineStatisticFlagBits(16);
    pub const CLIPPING_INVOCATIONS_BIT: QueryPipelineStatisticFlagBits = QueryPipelineStatisticFlagBits(32);
    pub const CLIPPING_PRIMITIVES_BIT: QueryPipelineStatisticFlagBits = QueryPipelineStatisticFlagBits(64);
    pub const FRAGMENT_SHADER_INVOCATIONS_BIT: QueryPipelineStatisticFlagBits = QueryPipelineStatisticFlagBits(128);
    pub const TESSELLATION_CONTROL_SHADER_PATCHES_BIT: QueryPipelineStatisticFlagBits = QueryPipelineStatisticFlagBits(256);
    pub const TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT: QueryPipelineStatisticFlagBits = QueryPipelineStatisticFlagBits(512);
    pub const COMPUTE_SHADER_INVOCATIONS_BIT: QueryPipelineStatisticFlagBits = QueryPipelineStatisticFlagBits(1024);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for QueryPipelineStatisticFlagBits {
    type Output = QueryPipelineStatisticFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for QueryPipelineStatisticFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for QueryPipelineStatisticFlagBits {
    type Output = QueryPipelineStatisticFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for QueryPipelineStatisticFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for QueryPipelineStatisticFlagBits {
    type Output = QueryPipelineStatisticFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for QueryPipelineStatisticFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct CommandBufferUsageFlagBits(Flags);
impl CommandBufferUsageFlagBits {
    pub const ONE_TIME_SUBMIT_BIT: CommandBufferUsageFlagBits = CommandBufferUsageFlagBits(1);
    pub const RENDER_PASS_CONTINUE_BIT: CommandBufferUsageFlagBits = CommandBufferUsageFlagBits(2);
    pub const SIMULTANEOUS_USE_BIT: CommandBufferUsageFlagBits = CommandBufferUsageFlagBits(4);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for CommandBufferUsageFlagBits {
    type Output = CommandBufferUsageFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for CommandBufferUsageFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for CommandBufferUsageFlagBits {
    type Output = CommandBufferUsageFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for CommandBufferUsageFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for CommandBufferUsageFlagBits {
    type Output = CommandBufferUsageFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for CommandBufferUsageFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct CommandPoolResetFlagBits(Flags);
impl CommandPoolResetFlagBits {
    pub const RELEASE_RESOURCES_BIT: CommandPoolResetFlagBits = CommandPoolResetFlagBits(1);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for CommandPoolResetFlagBits {
    type Output = CommandPoolResetFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for CommandPoolResetFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for CommandPoolResetFlagBits {
    type Output = CommandPoolResetFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for CommandPoolResetFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for CommandPoolResetFlagBits {
    type Output = CommandPoolResetFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for CommandPoolResetFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct CommandPoolCreateFlagBits(Flags);
impl CommandPoolCreateFlagBits {
    pub const TRANSIENT_BIT: CommandPoolCreateFlagBits = CommandPoolCreateFlagBits(1);
    pub const RESET_COMMAND_BUFFER_BIT: CommandPoolCreateFlagBits = CommandPoolCreateFlagBits(2);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for CommandPoolCreateFlagBits {
    type Output = CommandPoolCreateFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for CommandPoolCreateFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for CommandPoolCreateFlagBits {
    type Output = CommandPoolCreateFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for CommandPoolCreateFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for CommandPoolCreateFlagBits {
    type Output = CommandPoolCreateFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for CommandPoolCreateFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct SubpassDescriptionFlagBits(Flags);
impl SubpassDescriptionFlagBits {

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for SubpassDescriptionFlagBits {
    type Output = SubpassDescriptionFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for SubpassDescriptionFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for SubpassDescriptionFlagBits {
    type Output = SubpassDescriptionFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for SubpassDescriptionFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for SubpassDescriptionFlagBits {
    type Output = SubpassDescriptionFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for SubpassDescriptionFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct SampleCountFlagBits(Flags);
impl SampleCountFlagBits {
    pub const K_1_BIT: SampleCountFlagBits = SampleCountFlagBits(1);
    pub const K_2_BIT: SampleCountFlagBits = SampleCountFlagBits(2);
    pub const K_4_BIT: SampleCountFlagBits = SampleCountFlagBits(4);
    pub const K_8_BIT: SampleCountFlagBits = SampleCountFlagBits(8);
    pub const K_16_BIT: SampleCountFlagBits = SampleCountFlagBits(16);
    pub const K_32_BIT: SampleCountFlagBits = SampleCountFlagBits(32);
    pub const K_64_BIT: SampleCountFlagBits = SampleCountFlagBits(64);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for SampleCountFlagBits {
    type Output = SampleCountFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for SampleCountFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for SampleCountFlagBits {
    type Output = SampleCountFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for SampleCountFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for SampleCountFlagBits {
    type Output = SampleCountFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for SampleCountFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct AttachmentDescriptionFlagBits(Flags);
impl AttachmentDescriptionFlagBits {
    pub const MAY_ALIAS_BIT: AttachmentDescriptionFlagBits = AttachmentDescriptionFlagBits(1);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for AttachmentDescriptionFlagBits {
    type Output = AttachmentDescriptionFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for AttachmentDescriptionFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for AttachmentDescriptionFlagBits {
    type Output = AttachmentDescriptionFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for AttachmentDescriptionFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for AttachmentDescriptionFlagBits {
    type Output = AttachmentDescriptionFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for AttachmentDescriptionFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct DescriptorPoolCreateFlagBits(Flags);
impl DescriptorPoolCreateFlagBits {
    pub const FREE_DESCRIPTOR_SET_BIT: DescriptorPoolCreateFlagBits = DescriptorPoolCreateFlagBits(1);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for DescriptorPoolCreateFlagBits {
    type Output = DescriptorPoolCreateFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for DescriptorPoolCreateFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for DescriptorPoolCreateFlagBits {
    type Output = DescriptorPoolCreateFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for DescriptorPoolCreateFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for DescriptorPoolCreateFlagBits {
    type Output = DescriptorPoolCreateFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for DescriptorPoolCreateFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct DescriptorSetLayoutCreateFlagBits(Flags);
impl DescriptorSetLayoutCreateFlagBits {

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for DescriptorSetLayoutCreateFlagBits {
    type Output = DescriptorSetLayoutCreateFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for DescriptorSetLayoutCreateFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for DescriptorSetLayoutCreateFlagBits {
    type Output = DescriptorSetLayoutCreateFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for DescriptorSetLayoutCreateFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for DescriptorSetLayoutCreateFlagBits {
    type Output = DescriptorSetLayoutCreateFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for DescriptorSetLayoutCreateFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct SamplerCreateFlagBits(Flags);
impl SamplerCreateFlagBits {

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for SamplerCreateFlagBits {
    type Output = SamplerCreateFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for SamplerCreateFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for SamplerCreateFlagBits {
    type Output = SamplerCreateFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for SamplerCreateFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for SamplerCreateFlagBits {
    type Output = SamplerCreateFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for SamplerCreateFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct PipelineCreateFlagBits(Flags);
impl PipelineCreateFlagBits {
    pub const DISABLE_OPTIMIZATION_BIT: PipelineCreateFlagBits = PipelineCreateFlagBits(1);
    pub const ALLOW_DERIVATIVES_BIT: PipelineCreateFlagBits = PipelineCreateFlagBits(2);
    pub const DERIVATIVE_BIT: PipelineCreateFlagBits = PipelineCreateFlagBits(4);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for PipelineCreateFlagBits {
    type Output = PipelineCreateFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for PipelineCreateFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for PipelineCreateFlagBits {
    type Output = PipelineCreateFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for PipelineCreateFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for PipelineCreateFlagBits {
    type Output = PipelineCreateFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for PipelineCreateFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ColorComponentFlagBits(Flags);
impl ColorComponentFlagBits {
    pub const R_BIT: ColorComponentFlagBits = ColorComponentFlagBits(1);
    pub const G_BIT: ColorComponentFlagBits = ColorComponentFlagBits(2);
    pub const B_BIT: ColorComponentFlagBits = ColorComponentFlagBits(4);
    pub const A_BIT: ColorComponentFlagBits = ColorComponentFlagBits(8);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for ColorComponentFlagBits {
    type Output = ColorComponentFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for ColorComponentFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for ColorComponentFlagBits {
    type Output = ColorComponentFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for ColorComponentFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for ColorComponentFlagBits {
    type Output = ColorComponentFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for ColorComponentFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct CullModeFlagBits(Flags);
impl CullModeFlagBits {
    pub const NONE: CullModeFlagBits = CullModeFlagBits(0);
    pub const FRONT_BIT: CullModeFlagBits = CullModeFlagBits(1);
    pub const BACK_BIT: CullModeFlagBits = CullModeFlagBits(2);
    pub const FRONT_AND_BACK: CullModeFlagBits = CullModeFlagBits(3);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for CullModeFlagBits {
    type Output = CullModeFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for CullModeFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for CullModeFlagBits {
    type Output = CullModeFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for CullModeFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for CullModeFlagBits {
    type Output = CullModeFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for CullModeFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ImageViewCreateFlagBits(Flags);
impl ImageViewCreateFlagBits {

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for ImageViewCreateFlagBits {
    type Output = ImageViewCreateFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for ImageViewCreateFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for ImageViewCreateFlagBits {
    type Output = ImageViewCreateFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for ImageViewCreateFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for ImageViewCreateFlagBits {
    type Output = ImageViewCreateFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for ImageViewCreateFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ImageCreateFlagBits(Flags);
impl ImageCreateFlagBits {
    pub const SPARSE_BINDING_BIT: ImageCreateFlagBits = ImageCreateFlagBits(1);
    pub const SPARSE_RESIDENCY_BIT: ImageCreateFlagBits = ImageCreateFlagBits(2);
    pub const SPARSE_ALIASED_BIT: ImageCreateFlagBits = ImageCreateFlagBits(4);
    pub const MUTABLE_FORMAT_BIT: ImageCreateFlagBits = ImageCreateFlagBits(8);
    pub const CUBE_COMPATIBLE_BIT: ImageCreateFlagBits = ImageCreateFlagBits(16);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for ImageCreateFlagBits {
    type Output = ImageCreateFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for ImageCreateFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for ImageCreateFlagBits {
    type Output = ImageCreateFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for ImageCreateFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for ImageCreateFlagBits {
    type Output = ImageCreateFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for ImageCreateFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct BufferUsageFlagBits(Flags);
impl BufferUsageFlagBits {
    pub const TRANSFER_SRC_BIT: BufferUsageFlagBits = BufferUsageFlagBits(1);
    pub const TRANSFER_DST_BIT: BufferUsageFlagBits = BufferUsageFlagBits(2);
    pub const UNIFORM_TEXEL_BUFFER_BIT: BufferUsageFlagBits = BufferUsageFlagBits(4);
    pub const STORAGE_TEXEL_BUFFER_BIT: BufferUsageFlagBits = BufferUsageFlagBits(8);
    pub const UNIFORM_BUFFER_BIT: BufferUsageFlagBits = BufferUsageFlagBits(16);
    pub const STORAGE_BUFFER_BIT: BufferUsageFlagBits = BufferUsageFlagBits(32);
    pub const INDEX_BUFFER_BIT: BufferUsageFlagBits = BufferUsageFlagBits(64);
    pub const VERTEX_BUFFER_BIT: BufferUsageFlagBits = BufferUsageFlagBits(128);
    pub const INDIRECT_BUFFER_BIT: BufferUsageFlagBits = BufferUsageFlagBits(256);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for BufferUsageFlagBits {
    type Output = BufferUsageFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for BufferUsageFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for BufferUsageFlagBits {
    type Output = BufferUsageFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for BufferUsageFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for BufferUsageFlagBits {
    type Output = BufferUsageFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for BufferUsageFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct BufferCreateFlagBits(Flags);
impl BufferCreateFlagBits {
    pub const SPARSE_BINDING_BIT: BufferCreateFlagBits = BufferCreateFlagBits(1);
    pub const SPARSE_RESIDENCY_BIT: BufferCreateFlagBits = BufferCreateFlagBits(2);
    pub const SPARSE_ALIASED_BIT: BufferCreateFlagBits = BufferCreateFlagBits(4);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for BufferCreateFlagBits {
    type Output = BufferCreateFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for BufferCreateFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for BufferCreateFlagBits {
    type Output = BufferCreateFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for BufferCreateFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for BufferCreateFlagBits {
    type Output = BufferCreateFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for BufferCreateFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct FenceCreateFlagBits(Flags);
impl FenceCreateFlagBits {
    pub const SIGNALED_BIT: FenceCreateFlagBits = FenceCreateFlagBits(1);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for FenceCreateFlagBits {
    type Output = FenceCreateFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for FenceCreateFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for FenceCreateFlagBits {
    type Output = FenceCreateFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for FenceCreateFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for FenceCreateFlagBits {
    type Output = FenceCreateFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for FenceCreateFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct SparseMemoryBindFlagBits(Flags);
impl SparseMemoryBindFlagBits {
    pub const METADATA_BIT: SparseMemoryBindFlagBits = SparseMemoryBindFlagBits(1);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for SparseMemoryBindFlagBits {
    type Output = SparseMemoryBindFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for SparseMemoryBindFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for SparseMemoryBindFlagBits {
    type Output = SparseMemoryBindFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for SparseMemoryBindFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for SparseMemoryBindFlagBits {
    type Output = SparseMemoryBindFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for SparseMemoryBindFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct SparseImageFormatFlagBits(Flags);
impl SparseImageFormatFlagBits {
    pub const SINGLE_MIPTAIL_BIT: SparseImageFormatFlagBits = SparseImageFormatFlagBits(1);
    pub const ALIGNED_MIP_SIZE_BIT: SparseImageFormatFlagBits = SparseImageFormatFlagBits(2);
    pub const NONSTANDARD_BLOCK_SIZE_BIT: SparseImageFormatFlagBits = SparseImageFormatFlagBits(4);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for SparseImageFormatFlagBits {
    type Output = SparseImageFormatFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for SparseImageFormatFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for SparseImageFormatFlagBits {
    type Output = SparseImageFormatFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for SparseImageFormatFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for SparseImageFormatFlagBits {
    type Output = SparseImageFormatFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for SparseImageFormatFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct DeviceQueueCreateFlagBits(Flags);
impl DeviceQueueCreateFlagBits {

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for DeviceQueueCreateFlagBits {
    type Output = DeviceQueueCreateFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for DeviceQueueCreateFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for DeviceQueueCreateFlagBits {
    type Output = DeviceQueueCreateFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for DeviceQueueCreateFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for DeviceQueueCreateFlagBits {
    type Output = DeviceQueueCreateFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for DeviceQueueCreateFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct MemoryHeapFlagBits(Flags);
impl MemoryHeapFlagBits {
    pub const DEVICE_LOCAL_BIT: MemoryHeapFlagBits = MemoryHeapFlagBits(1);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for MemoryHeapFlagBits {
    type Output = MemoryHeapFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for MemoryHeapFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for MemoryHeapFlagBits {
    type Output = MemoryHeapFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for MemoryHeapFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for MemoryHeapFlagBits {
    type Output = MemoryHeapFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for MemoryHeapFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct MemoryPropertyFlagBits(Flags);
impl MemoryPropertyFlagBits {
    pub const DEVICE_LOCAL_BIT: MemoryPropertyFlagBits = MemoryPropertyFlagBits(1);
    pub const HOST_VISIBLE_BIT: MemoryPropertyFlagBits = MemoryPropertyFlagBits(2);
    pub const HOST_COHERENT_BIT: MemoryPropertyFlagBits = MemoryPropertyFlagBits(4);
    pub const HOST_CACHED_BIT: MemoryPropertyFlagBits = MemoryPropertyFlagBits(8);
    pub const LAZILY_ALLOCATED_BIT: MemoryPropertyFlagBits = MemoryPropertyFlagBits(16);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for MemoryPropertyFlagBits {
    type Output = MemoryPropertyFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for MemoryPropertyFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for MemoryPropertyFlagBits {
    type Output = MemoryPropertyFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for MemoryPropertyFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for MemoryPropertyFlagBits {
    type Output = MemoryPropertyFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for MemoryPropertyFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct QueueFlagBits(Flags);
impl QueueFlagBits {
    pub const GRAPHICS_BIT: QueueFlagBits = QueueFlagBits(1);
    pub const COMPUTE_BIT: QueueFlagBits = QueueFlagBits(2);
    pub const TRANSFER_BIT: QueueFlagBits = QueueFlagBits(4);
    pub const SPARSE_BINDING_BIT: QueueFlagBits = QueueFlagBits(8);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for QueueFlagBits {
    type Output = QueueFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for QueueFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for QueueFlagBits {
    type Output = QueueFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for QueueFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for QueueFlagBits {
    type Output = QueueFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for QueueFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(transparent)]
#[derive(Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct FormatFeatureFlagBits(Flags);
impl FormatFeatureFlagBits {
    pub const SAMPLED_IMAGE_BIT: FormatFeatureFlagBits = FormatFeatureFlagBits(1);
    pub const STORAGE_IMAGE_BIT: FormatFeatureFlagBits = FormatFeatureFlagBits(2);
    pub const STORAGE_IMAGE_ATOMIC_BIT: FormatFeatureFlagBits = FormatFeatureFlagBits(4);
    pub const UNIFORM_TEXEL_BUFFER_BIT: FormatFeatureFlagBits = FormatFeatureFlagBits(8);
    pub const STORAGE_TEXEL_BUFFER_BIT: FormatFeatureFlagBits = FormatFeatureFlagBits(16);
    pub const STORAGE_TEXEL_BUFFER_ATOMIC_BIT: FormatFeatureFlagBits = FormatFeatureFlagBits(32);
    pub const VERTEX_BUFFER_BIT: FormatFeatureFlagBits = FormatFeatureFlagBits(64);
    pub const COLOR_ATTACHMENT_BIT: FormatFeatureFlagBits = FormatFeatureFlagBits(128);
    pub const COLOR_ATTACHMENT_BLEND_BIT: FormatFeatureFlagBits = FormatFeatureFlagBits(256);
    pub const DEPTH_STENCIL_ATTACHMENT_BIT: FormatFeatureFlagBits = FormatFeatureFlagBits(512);
    pub const BLIT_SRC_BIT: FormatFeatureFlagBits = FormatFeatureFlagBits(1024);
    pub const BLIT_DST_BIT: FormatFeatureFlagBits = FormatFeatureFlagBits(2048);
    pub const SAMPLED_IMAGE_FILTER_LINEAR_BIT: FormatFeatureFlagBits = FormatFeatureFlagBits(4096);

    #[inline]
    pub fn contains(&self, other: &Self) -> bool { return (self.0 & other.0) == other.0; }
}

impl core::ops::BitOr for FormatFeatureFlagBits {
    type Output = FormatFeatureFlagBits;
    #[inline]
    fn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }
}

impl core::ops::BitOrAssign for FormatFeatureFlagBits {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) { self.0 |= rhs.0; }
}

impl core::ops::BitAnd for FormatFeatureFlagBits {
    type Output = FormatFeatureFlagBits;
    #[inline]
    fn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }
}

impl core::ops::BitAndAssign for FormatFeatureFlagBits {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) { self.0 &= rhs.0; }
}

impl core::ops::BitXor for FormatFeatureFlagBits {
    type Output = FormatFeatureFlagBits;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }
}

impl core::ops::BitXorAssign for FormatFeatureFlagBits {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) { self.0 ^= rhs.0; }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AllocationCallbacks {
    pub p_user_data: *mut core::ffi::c_void,
    pub pfn_allocation: PfnAllocationFunction,
    pub pfn_reallocation: PfnReallocationFunction,
    pub pfn_free: PfnFreeFunction,
    pub pfn_internal_allocation: PfnInternalAllocationNotification,
    pub pfn_internal_free: PfnInternalFreeNotification,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Win32SurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: Win32SurfaceCreateFlagsKHR,
    pub hinstance: HINSTANCE,
    pub hwnd: HWND,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfaceFormatKHR {
    pub format: Format,
    pub color_space: ColorSpaceKHR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfaceCapabilitiesKHR {
    pub min_image_count: u32,
    pub max_image_count: u32,
    pub current_extent: Extent2D,
    pub min_image_extent: Extent2D,
    pub max_image_extent: Extent2D,
    pub max_image_array_layers: u32,
    pub supported_transforms: SurfaceTransformFlagsKHR,
    pub current_transform: SurfaceTransformFlagBitsKHR,
    pub supported_composite_alpha: CompositeAlphaFlagsKHR,
    pub supported_usage_flags: ImageUsageFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Extent2D {
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PresentInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphores: *const Semaphore,
    pub swapchain_count: u32,
    pub p_swapchains: *const SwapchainKHR,
    pub p_image_indices: *const u32,
    pub p_results: *mut Result,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SwapchainCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: SwapchainCreateFlagsKHR,
    pub surface: SurfaceKHR,
    pub min_image_count: u32,
    pub image_format: Format,
    pub image_color_space: ColorSpaceKHR,
    pub image_extent: Extent2D,
    pub image_array_layers: u32,
    pub image_usage: ImageUsageFlags,
    pub image_sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
    pub pre_transform: SurfaceTransformFlagBitsKHR,
    pub composite_alpha: CompositeAlphaFlagBitsKHR,
    pub present_mode: PresentModeKHR,
    pub clipped: Bool32,
    pub old_swapchain: SwapchainKHR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DebugUtilsMessengerCallbackDataEXT {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: DebugUtilsMessengerCallbackDataFlagsEXT,
    pub p_message_id_name: *const u8,
    pub message_id_number: i32,
    pub p_message: *const u8,
    pub queue_label_count: u32,
    pub p_queue_labels: *const DebugUtilsLabelEXT,
    pub cmd_buf_label_count: u32,
    pub p_cmd_buf_labels: *const DebugUtilsLabelEXT,
    pub object_count: u32,
    pub p_objects: *const DebugUtilsObjectNameInfoEXT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DebugUtilsObjectNameInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub object_type: ObjectType,
    pub object_handle: u64,
    pub p_object_name: *const u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DebugUtilsLabelEXT {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub p_label_name: *const u8,
    pub color: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DebugUtilsMessengerCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: DebugUtilsMessengerCreateFlagsEXT,
    pub message_severity: DebugUtilsMessageSeverityFlagsEXT,
    pub message_type: DebugUtilsMessageTypeFlagsEXT,
    pub pfn_user_callback: PfnDebugUtilsMessengerCallbackEXT,
    pub p_user_data: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DebugUtilsObjectTagInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub object_type: ObjectType,
    pub object_handle: u64,
    pub tag_name: u64,
    pub tag_size: usize,
    pub p_tag: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BaseInStructure {
    pub s_type: StructureType,
    pub p_next: *const BaseInStructure,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BaseOutStructure {
    pub s_type: StructureType,
    pub p_next: *mut BaseOutStructure,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryBarrier {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageMemoryBarrier {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub old_layout: ImageLayout,
    pub new_layout: ImageLayout,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub image: Image,
    pub subresource_range: ImageSubresourceRange,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageSubresourceRange {
    pub aspect_mask: ImageAspectFlags,
    pub base_mip_level: u32,
    pub level_count: u32,
    pub base_array_layer: u32,
    pub layer_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DrawIndirectCommand {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub first_vertex: u32,
    pub first_instance: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DrawIndexedIndirectCommand {
    pub index_count: u32,
    pub instance_count: u32,
    pub first_index: u32,
    pub vertex_offset: i32,
    pub first_instance: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DispatchIndirectCommand {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferMemoryBarrier {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub render_pass: RenderPass,
    pub framebuffer: Framebuffer,
    pub render_area: Rect2D,
    pub clear_value_count: u32,
    pub p_clear_values: *const ClearValue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClearDepthStencilValue {
    pub depth: f32,
    pub stencil: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Rect2D {
    pub offset: Offset2D,
    pub extent: Extent2D,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Offset2D {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageResolve {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offset: Offset3D,
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offset: Offset3D,
    pub extent: Extent3D,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Extent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Offset3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageSubresourceLayers {
    pub aspect_mask: ImageAspectFlags,
    pub mip_level: u32,
    pub base_array_layer: u32,
    pub layer_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClearRect {
    pub rect: Rect2D,
    pub base_array_layer: u32,
    pub layer_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClearAttachment {
    pub aspect_mask: ImageAspectFlags,
    pub color_attachment: u32,
    pub clear_value: ClearValue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferImageCopy {
    pub buffer_offset: DeviceSize,
    pub buffer_row_length: u32,
    pub buffer_image_height: u32,
    pub image_subresource: ImageSubresourceLayers,
    pub image_offset: Offset3D,
    pub image_extent: Extent3D,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageBlit {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offsets: [Offset3D; 2],
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offsets: [Offset3D; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageCopy {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offset: Offset3D,
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offset: Offset3D,
    pub extent: Extent3D,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferCopy {
    pub src_offset: DeviceSize,
    pub dst_offset: DeviceSize,
    pub size: DeviceSize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CommandBufferBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: CommandBufferUsageFlags,
    pub p_inheritance_info: *const CommandBufferInheritanceInfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CommandBufferInheritanceInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub render_pass: RenderPass,
    pub subpass: u32,
    pub framebuffer: Framebuffer,
    pub occlusion_query_enable: Bool32,
    pub query_flags: QueryControlFlags,
    pub pipeline_statistics: QueryPipelineStatisticFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CommandBufferAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub command_pool: CommandPool,
    pub level: CommandBufferLevel,
    pub command_buffer_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CommandPoolCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: CommandPoolCreateFlags,
    pub queue_family_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: RenderPassCreateFlags,
    pub attachment_count: u32,
    pub p_attachments: *const AttachmentDescription,
    pub subpass_count: u32,
    pub p_subpasses: *const SubpassDescription,
    pub dependency_count: u32,
    pub p_dependencies: *const SubpassDependency,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubpassDependency {
    pub src_subpass: u32,
    pub dst_subpass: u32,
    pub src_stage_mask: PipelineStageFlags,
    pub dst_stage_mask: PipelineStageFlags,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub dependency_flags: DependencyFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubpassDescription {
    pub flags: SubpassDescriptionFlags,
    pub pipeline_bind_point: PipelineBindPoint,
    pub input_attachment_count: u32,
    pub p_input_attachments: *const AttachmentReference,
    pub color_attachment_count: u32,
    pub p_color_attachments: *const AttachmentReference,
    pub p_resolve_attachments: *const AttachmentReference,
    pub p_depth_stencil_attachment: *const AttachmentReference,
    pub preserve_attachment_count: u32,
    pub p_preserve_attachments: *const u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AttachmentReference {
    pub attachment: u32,
    pub layout: ImageLayout,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AttachmentDescription {
    pub flags: AttachmentDescriptionFlags,
    pub format: Format,
    pub samples: SampleCountFlagBits,
    pub load_op: AttachmentLoadOp,
    pub store_op: AttachmentStoreOp,
    pub stencil_load_op: AttachmentLoadOp,
    pub stencil_store_op: AttachmentStoreOp,
    pub initial_layout: ImageLayout,
    pub final_layout: ImageLayout,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FramebufferCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: FramebufferCreateFlags,
    pub render_pass: RenderPass,
    pub attachment_count: u32,
    pub p_attachments: *const ImageView,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CopyDescriptorSet {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub src_set: DescriptorSet,
    pub src_binding: u32,
    pub src_array_element: u32,
    pub dst_set: DescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WriteDescriptorSet {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub dst_set: DescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
    pub descriptor_type: DescriptorType,
    pub p_image_info: *const DescriptorImageInfo,
    pub p_buffer_info: *const DescriptorBufferInfo,
    pub p_texel_buffer_view: *const BufferView,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorBufferInfo {
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub range: DeviceSize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorImageInfo {
    pub sampler: Sampler,
    pub image_view: ImageView,
    pub image_layout: ImageLayout,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorSetAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub descriptor_pool: DescriptorPool,
    pub descriptor_set_count: u32,
    pub p_set_layouts: *const DescriptorSetLayout,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorPoolCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: DescriptorPoolCreateFlags,
    pub max_sets: u32,
    pub pool_size_count: u32,
    pub p_pool_sizes: *const DescriptorPoolSize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorPoolSize {
    pub kind: DescriptorType,
    pub descriptor_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorSetLayoutCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: DescriptorSetLayoutCreateFlags,
    pub binding_count: u32,
    pub p_bindings: *const DescriptorSetLayoutBinding,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorSetLayoutBinding {
    pub binding: u32,
    pub descriptor_type: DescriptorType,
    pub descriptor_count: u32,
    pub stage_flags: ShaderStageFlags,
    pub p_immutable_samplers: *const Sampler,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: SamplerCreateFlags,
    pub mag_filter: Filter,
    pub min_filter: Filter,
    pub mipmap_mode: SamplerMipmapMode,
    pub address_mode_u: SamplerAddressMode,
    pub address_mode_v: SamplerAddressMode,
    pub address_mode_w: SamplerAddressMode,
    pub mip_lod_bias: f32,
    pub anisotropy_enable: Bool32,
    pub max_anisotropy: f32,
    pub compare_enable: Bool32,
    pub compare_op: CompareOp,
    pub min_lod: f32,
    pub max_lod: f32,
    pub border_color: BorderColor,
    pub unnormalized_coordinates: Bool32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineLayoutCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: PipelineLayoutCreateFlags,
    pub set_layout_count: u32,
    pub p_set_layouts: *const DescriptorSetLayout,
    pub push_constant_range_count: u32,
    pub p_push_constant_ranges: *const PushConstantRange,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PushConstantRange {
    pub stage_flags: ShaderStageFlags,
    pub offset: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ComputePipelineCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: PipelineCreateFlags,
    pub stage: PipelineShaderStageCreateInfo,
    pub layout: PipelineLayout,
    pub base_pipeline_handle: Pipeline,
    pub base_pipeline_index: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineShaderStageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: PipelineShaderStageCreateFlags,
    pub stage: ShaderStageFlagBits,
    pub module: ShaderModule,
    pub p_name: *const u8,
    pub p_specialization_info: *const SpecializationInfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SpecializationInfo {
    pub map_entry_count: u32,
    pub p_map_entries: *const SpecializationMapEntry,
    pub data_size: usize,
    pub p_data: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SpecializationMapEntry {
    pub ant_id: u32,
    pub offset: u32,
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GraphicsPipelineCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: PipelineCreateFlags,
    pub stage_count: u32,
    pub p_stages: *const PipelineShaderStageCreateInfo,
    pub p_vertex_input_state: *const PipelineVertexInputStateCreateInfo,
    pub p_input_assembly_state: *const PipelineInputAssemblyStateCreateInfo,
    pub p_tessellation_state: *const PipelineTessellationStateCreateInfo,
    pub p_viewport_state: *const PipelineViewportStateCreateInfo,
    pub p_rasterization_state: *const PipelineRasterizationStateCreateInfo,
    pub p_multisample_state: *const PipelineMultisampleStateCreateInfo,
    pub p_depth_stencil_state: *const PipelineDepthStencilStateCreateInfo,
    pub p_color_blend_state: *const PipelineColorBlendStateCreateInfo,
    pub p_dynamic_state: *const PipelineDynamicStateCreateInfo,
    pub layout: PipelineLayout,
    pub render_pass: RenderPass,
    pub subpass: u32,
    pub base_pipeline_handle: Pipeline,
    pub base_pipeline_index: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineDynamicStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: PipelineDynamicStateCreateFlags,
    pub dynamic_state_count: u32,
    pub p_dynamic_states: *const DynamicState,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineColorBlendStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: PipelineColorBlendStateCreateFlags,
    pub logic_op_enable: Bool32,
    pub logic_op: LogicOp,
    pub attachment_count: u32,
    pub p_attachments: *const PipelineColorBlendAttachmentState,
    pub blend_constants: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineColorBlendAttachmentState {
    pub blend_enable: Bool32,
    pub src_color_blend_factor: BlendFactor,
    pub dst_color_blend_factor: BlendFactor,
    pub color_blend_op: BlendOp,
    pub src_alpha_blend_factor: BlendFactor,
    pub dst_alpha_blend_factor: BlendFactor,
    pub alpha_blend_op: BlendOp,
    pub color_write_mask: ColorComponentFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineDepthStencilStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: PipelineDepthStencilStateCreateFlags,
    pub depth_test_enable: Bool32,
    pub depth_write_enable: Bool32,
    pub depth_compare_op: CompareOp,
    pub depth_bounds_test_enable: Bool32,
    pub stencil_test_enable: Bool32,
    pub front: StencilOpState,
    pub back: StencilOpState,
    pub min_depth_bounds: f32,
    pub max_depth_bounds: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StencilOpState {
    pub fail_op: StencilOp,
    pub pass_op: StencilOp,
    pub depth_fail_op: StencilOp,
    pub compare_op: CompareOp,
    pub compare_mask: u32,
    pub write_mask: u32,
    pub reference: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineMultisampleStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: PipelineMultisampleStateCreateFlags,
    pub rasterization_samples: SampleCountFlagBits,
    pub sample_shading_enable: Bool32,
    pub min_sample_shading: f32,
    pub p_sample_mask: *const SampleMask,
    pub alpha_to_coverage_enable: Bool32,
    pub alpha_to_one_enable: Bool32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineRasterizationStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: PipelineRasterizationStateCreateFlags,
    pub depth_clamp_enable: Bool32,
    pub rasterizer_discard_enable: Bool32,
    pub polygon_mode: PolygonMode,
    pub cull_mode: CullModeFlags,
    pub front_face: FrontFace,
    pub depth_bias_enable: Bool32,
    pub depth_bias_constant_factor: f32,
    pub depth_bias_clamp: f32,
    pub depth_bias_slope_factor: f32,
    pub line_width: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineViewportStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: PipelineViewportStateCreateFlags,
    pub viewport_count: u32,
    pub p_viewports: *const Viewport,
    pub scissor_count: u32,
    pub p_scissors: *const Rect2D,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineTessellationStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: PipelineTessellationStateCreateFlags,
    pub patch_control_points: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineInputAssemblyStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: PipelineInputAssemblyStateCreateFlags,
    pub topology: PrimitiveTopology,
    pub primitive_restart_enable: Bool32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineVertexInputStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: PipelineVertexInputStateCreateFlags,
    pub vertex_binding_description_count: u32,
    pub p_vertex_binding_descriptions: *const VertexInputBindingDescription,
    pub vertex_attribute_description_count: u32,
    pub p_vertex_attribute_descriptions: *const VertexInputAttributeDescription,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VertexInputAttributeDescription {
    pub location: u32,
    pub binding: u32,
    pub format: Format,
    pub offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VertexInputBindingDescription {
    pub binding: u32,
    pub stride: u32,
    pub input_rate: VertexInputRate,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineCacheCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: PipelineCacheCreateFlags,
    pub initial_data_size: usize,
    pub p_initial_data: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShaderModuleCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: ShaderModuleCreateFlags,
    pub code_size: usize,
    pub p_code: *const u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageViewCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: ImageViewCreateFlags,
    pub image: Image,
    pub view_type: ImageViewType,
    pub format: Format,
    pub components: ComponentMapping,
    pub subresource_range: ImageSubresourceRange,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ComponentMapping {
    pub r: ComponentSwizzle,
    pub g: ComponentSwizzle,
    pub b: ComponentSwizzle,
    pub a: ComponentSwizzle,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubresourceLayout {
    pub offset: DeviceSize,
    pub size: DeviceSize,
    pub row_pitch: DeviceSize,
    pub array_pitch: DeviceSize,
    pub depth_pitch: DeviceSize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageSubresource {
    pub aspect_mask: ImageAspectFlags,
    pub mip_level: u32,
    pub array_layer: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: ImageCreateFlags,
    pub image_type: ImageType,
    pub format: Format,
    pub extent: Extent3D,
    pub mip_levels: u32,
    pub array_layers: u32,
    pub samples: SampleCountFlagBits,
    pub tiling: ImageTiling,
    pub usage: ImageUsageFlags,
    pub sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
    pub initial_layout: ImageLayout,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferViewCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: BufferViewCreateFlags,
    pub buffer: Buffer,
    pub format: Format,
    pub offset: DeviceSize,
    pub range: DeviceSize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: BufferCreateFlags,
    pub size: DeviceSize,
    pub usage: BufferUsageFlags,
    pub sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct QueryPoolCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: QueryPoolCreateFlags,
    pub query_type: QueryType,
    pub query_count: u32,
    pub pipeline_statistics: QueryPipelineStatisticFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct EventCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: EventCreateFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SemaphoreCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: SemaphoreCreateFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FenceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: FenceCreateFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindSparseInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphores: *const Semaphore,
    pub buffer_bind_count: u32,
    pub p_buffer_binds: *const SparseBufferMemoryBindInfo,
    pub image_opaque_bind_count: u32,
    pub p_image_opaque_binds: *const SparseImageOpaqueMemoryBindInfo,
    pub image_bind_count: u32,
    pub p_image_binds: *const SparseImageMemoryBindInfo,
    pub signal_semaphore_count: u32,
    pub p_signal_semaphores: *const Semaphore,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseImageMemoryBindInfo {
    pub image: Image,
    pub bind_count: u32,
    pub p_binds: *const SparseImageMemoryBind,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseImageMemoryBind {
    pub subresource: ImageSubresource,
    pub offset: Offset3D,
    pub extent: Extent3D,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
    pub flags: SparseMemoryBindFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseImageOpaqueMemoryBindInfo {
    pub image: Image,
    pub bind_count: u32,
    pub p_binds: *const SparseMemoryBind,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseMemoryBind {
    pub resource_offset: DeviceSize,
    pub size: DeviceSize,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
    pub flags: SparseMemoryBindFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseBufferMemoryBindInfo {
    pub buffer: Buffer,
    pub bind_count: u32,
    pub p_binds: *const SparseMemoryBind,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseImageFormatProperties {
    pub aspect_mask: ImageAspectFlags,
    pub image_granularity: Extent3D,
    pub flags: SparseImageFormatFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseImageMemoryRequirements {
    pub format_properties: SparseImageFormatProperties,
    pub image_mip_tail_first_lod: u32,
    pub image_mip_tail_size: DeviceSize,
    pub image_mip_tail_offset: DeviceSize,
    pub image_mip_tail_stride: DeviceSize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryRequirements {
    pub size: DeviceSize,
    pub alignment: DeviceSize,
    pub memory_type_bits: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MappedMemoryRange {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub memory: DeviceMemory,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub allocation_size: DeviceSize,
    pub memory_type_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubmitInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphores: *const Semaphore,
    pub p_wait_dst_stage_mask: *const PipelineStageFlags,
    pub command_buffer_count: u32,
    pub p_command_buffers: *const CommandBuffer,
    pub signal_semaphore_count: u32,
    pub p_signal_semaphores: *const Semaphore,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LayerProperties {
    pub layer_name: [u8; 256],
    pub spec_version: u32,
    pub implementation_version: u32,
    pub description: [u8; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExtensionProperties {
    pub extension_name: [u8; 256],
    pub spec_version: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: DeviceCreateFlags,
    pub queue_create_info_count: u32,
    pub p_queue_create_infos: *const DeviceQueueCreateInfo,
    pub enabled_layer_count: u32,
    pub pp_enabled_layer_names: *const *const u8,
    pub enabled_extension_count: u32,
    pub pp_enabled_extension_names: *const *const u8,
    pub p_enabled_features: *const PhysicalDeviceFeatures,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFeatures {
    pub robust_buffer_access: Bool32,
    pub full_draw_index_uint_32: Bool32,
    pub image_cube_array: Bool32,
    pub independent_blend: Bool32,
    pub geometry_shader: Bool32,
    pub tessellation_shader: Bool32,
    pub sample_rate_shading: Bool32,
    pub dual_src_blend: Bool32,
    pub logic_op: Bool32,
    pub multi_draw_indirect: Bool32,
    pub draw_indirect_first_instance: Bool32,
    pub depth_clamp: Bool32,
    pub depth_bias_clamp: Bool32,
    pub fill_mode_non_solid: Bool32,
    pub depth_bounds: Bool32,
    pub wide_lines: Bool32,
    pub large_points: Bool32,
    pub alpha_to_one: Bool32,
    pub multi_viewport: Bool32,
    pub sampler_anisotropy: Bool32,
    pub texture_compression_etc_2: Bool32,
    pub texture_compression_astc_ldr: Bool32,
    pub texture_compression_bc: Bool32,
    pub occlusion_query_precise: Bool32,
    pub pipeline_statistics_query: Bool32,
    pub vertex_pipeline_stores_and_atomics: Bool32,
    pub fragment_stores_and_atomics: Bool32,
    pub shader_tessellation_and_geometry_point_size: Bool32,
    pub shader_image_gather_extended: Bool32,
    pub shader_storage_image_extended_formats: Bool32,
    pub shader_storage_image_multisample: Bool32,
    pub shader_storage_image_read_without_format: Bool32,
    pub shader_storage_image_write_without_format: Bool32,
    pub shader_uniform_buffer_array_dynamic_indexing: Bool32,
    pub shader_sampled_image_array_dynamic_indexing: Bool32,
    pub shader_storage_buffer_array_dynamic_indexing: Bool32,
    pub shader_storage_image_array_dynamic_indexing: Bool32,
    pub shader_clip_distance: Bool32,
    pub shader_cull_distance: Bool32,
    pub shader_float_64: Bool32,
    pub shader_int_64: Bool32,
    pub shader_int_16: Bool32,
    pub shader_resource_residency: Bool32,
    pub shader_resource_min_lod: Bool32,
    pub sparse_binding: Bool32,
    pub sparse_residency_buffer: Bool32,
    pub sparse_residency_image_2_d: Bool32,
    pub sparse_residency_image_3_d: Bool32,
    pub sparse_residency_2_samples: Bool32,
    pub sparse_residency_4_samples: Bool32,
    pub sparse_residency_8_samples: Bool32,
    pub sparse_residency_16_samples: Bool32,
    pub sparse_residency_aliased: Bool32,
    pub variable_multisample_rate: Bool32,
    pub inherited_queries: Bool32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceQueueCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: DeviceQueueCreateFlags,
    pub queue_family_index: u32,
    pub queue_count: u32,
    pub p_queue_priorities: *const f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMemoryProperties {
    pub memory_type_count: u32,
    pub memory_types: [MemoryType; 32],
    pub memory_heap_count: u32,
    pub memory_heaps: [MemoryHeap; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryHeap {
    pub size: DeviceSize,
    pub flags: MemoryHeapFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryType {
    pub property_flags: MemoryPropertyFlags,
    pub heap_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct QueueFamilyProperties {
    pub queue_flags: QueueFlags,
    pub queue_count: u32,
    pub timestamp_valid_bits: u32,
    pub min_image_transfer_granularity: Extent3D,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceProperties {
    pub api_version: u32,
    pub driver_version: u32,
    pub vendor_id: u32,
    pub device_id: u32,
    pub device_type: PhysicalDeviceType,
    pub device_name: [u8; 256],
    pub pipeline_cache_uuid: [u8; 16],
    pub limits: PhysicalDeviceLimits,
    pub sparse_properties: PhysicalDeviceSparseProperties,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSparseProperties {
    pub residency_standard_2_d_block_shape: Bool32,
    pub residency_standard_2_d_multisample_block_shape: Bool32,
    pub residency_standard_3_d_block_shape: Bool32,
    pub residency_aligned_mip_size: Bool32,
    pub residency_non_resident_strict: Bool32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceLimits {
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
    pub buffer_image_granularity: DeviceSize,
    pub sparse_address_space_size: DeviceSize,
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
    pub min_texel_buffer_offset_alignment: DeviceSize,
    pub min_uniform_buffer_offset_alignment: DeviceSize,
    pub min_storage_buffer_offset_alignment: DeviceSize,
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
    pub framebuffer_color_sample_counts: SampleCountFlags,
    pub framebuffer_depth_sample_counts: SampleCountFlags,
    pub framebuffer_stencil_sample_counts: SampleCountFlags,
    pub framebuffer_no_attachments_sample_counts: SampleCountFlags,
    pub max_color_attachments: u32,
    pub sampled_image_color_sample_counts: SampleCountFlags,
    pub sampled_image_integer_sample_counts: SampleCountFlags,
    pub sampled_image_depth_sample_counts: SampleCountFlags,
    pub sampled_image_stencil_sample_counts: SampleCountFlags,
    pub storage_image_sample_counts: SampleCountFlags,
    pub max_sample_mask_words: u32,
    pub timestamp_compute_and_graphics: Bool32,
    pub timestamp_period: f32,
    pub max_clip_distances: u32,
    pub max_cull_distances: u32,
    pub max_combined_clip_and_cull_distances: u32,
    pub discrete_queue_priorities: u32,
    pub point_size_range: [f32; 2],
    pub line_width_range: [f32; 2],
    pub point_size_granularity: f32,
    pub line_width_granularity: f32,
    pub strict_lines: Bool32,
    pub standard_sample_locations: Bool32,
    pub optimal_buffer_copy_offset_alignment: DeviceSize,
    pub optimal_buffer_copy_row_pitch_alignment: DeviceSize,
    pub non_coherent_atom_size: DeviceSize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageFormatProperties {
    pub max_extent: Extent3D,
    pub max_mip_levels: u32,
    pub max_array_layers: u32,
    pub sample_counts: SampleCountFlags,
    pub max_resource_size: DeviceSize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FormatProperties {
    pub linear_tiling_features: FormatFeatureFlags,
    pub optimal_tiling_features: FormatFeatureFlags,
    pub buffer_features: FormatFeatureFlags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct InstanceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub flags: InstanceCreateFlags,
    pub p_application_info: *const ApplicationInfo,
    pub enabled_layer_count: u32,
    pub pp_enabled_layer_names: *const *const u8,
    pub enabled_extension_count: u32,
    pub pp_enabled_extension_names: *const *const u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ApplicationInfo {
    pub s_type: StructureType,
    pub p_next: *const core::ffi::c_void,
    pub p_application_name: *const u8,
    pub application_version: u32,
    pub p_engine_name: *const u8,
    pub engine_version: u32,
    pub api_version: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ClearValue {
    pub color: ClearColorValue,
    pub depth_stencil: ClearDepthStencilValue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ClearColorValue {
    pub float_32: [f32; 4],
    pub int_32: [i32; 4],
    pub uint_32: [u32; 4],
}

pub type PfnInternalFreeNotification = extern "system" fn(
    p_user_data: *mut core::ffi::c_void,
    size: usize,
    allocation_type: InternalAllocationType,
    allocation_scope: SystemAllocationScope,
);

pub type PfnInternalAllocationNotification = extern "system" fn(
    p_user_data: *mut core::ffi::c_void,
    size: usize,
    allocation_type: InternalAllocationType,
    allocation_scope: SystemAllocationScope,
);

pub type PfnFreeFunction = extern "system" fn(
    p_user_data: *mut core::ffi::c_void,
    p_memory: *mut core::ffi::c_void,
);

pub type PfnReallocationFunction = extern "system" fn(
    p_user_data: *mut core::ffi::c_void,
    p_original: *mut core::ffi::c_void,
    size: usize,
    alignment: usize,
    allocation_scope: SystemAllocationScope,
) -> *mut core::ffi::c_void;

pub type PfnAllocationFunction = extern "system" fn(
    p_user_data: *mut core::ffi::c_void,
    size: usize,
    alignment: usize,
    allocation_scope: SystemAllocationScope,
) -> *mut core::ffi::c_void;

pub type PfnDebugUtilsMessengerCallbackEXT = extern "system" fn(
    message_severity: DebugUtilsMessageSeverityFlagBitsEXT,
    message_types: DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
    p_user_data: *mut core::ffi::c_void,
) -> Bool32;

pub type PfnVoidFunction = extern "system" fn(
);

pub type PfnGetPhysicalDeviceWin32PresentationSupportKHR = extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
) -> Bool32;

pub type PfnCreateWin32SurfaceKHR = extern "system" fn(
    instance: Instance,
    p_create_info: *const Win32SurfaceCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> Result;

pub type PfnGetPhysicalDeviceSurfacePresentModesKHR = extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    p_present_mode_count: *mut u32,
    p_present_modes: *mut PresentModeKHR,
) -> Result;

pub type PfnGetPhysicalDeviceSurfaceFormatsKHR = extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    p_surface_format_count: *mut u32,
    p_surface_formats: *mut SurfaceFormatKHR,
) -> Result;

pub type PfnGetPhysicalDeviceSurfaceCapabilitiesKHR = extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    p_surface_capabilities: *mut SurfaceCapabilitiesKHR,
) -> Result;

pub type PfnGetPhysicalDeviceSurfaceSupportKHR = extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    surface: SurfaceKHR,
    p_supported: *mut Bool32,
) -> Result;

pub type PfnDestroySurfaceKHR = extern "system" fn(
    instance: Instance,
    surface: SurfaceKHR,
    p_allocator: *const AllocationCallbacks,
);

pub type PfnQueuePresentKHR = extern "system" fn(
    queue: Queue,
    p_present_info: *const PresentInfoKHR,
) -> Result;

pub type PfnAcquireNextImageKHR = extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    timeout: u64,
    semaphore: Semaphore,
    fence: Fence,
    p_image_index: *mut u32,
) -> Result;

pub type PfnGetSwapchainImagesKHR = extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_swapchain_image_count: *mut u32,
    p_swapchain_images: *mut Image,
) -> Result;

pub type PfnDestroySwapchainKHR = extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_allocator: *const AllocationCallbacks,
);

pub type PfnCreateSwapchainKHR = extern "system" fn(
    device: Device,
    p_create_info: *const SwapchainCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_swapchain: *mut SwapchainKHR,
) -> Result;

pub type PfnSubmitDebugUtilsMessageEXT = extern "system" fn(
    instance: Instance,
    message_severity: DebugUtilsMessageSeverityFlagBitsEXT,
    message_types: DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
);

pub type PfnDestroyDebugUtilsMessengerEXT = extern "system" fn(
    instance: Instance,
    messenger: DebugUtilsMessengerEXT,
    p_allocator: *const AllocationCallbacks,
);

pub type PfnCreateDebugUtilsMessengerEXT = extern "system" fn(
    instance: Instance,
    p_create_info: *const DebugUtilsMessengerCreateInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_messenger: *mut DebugUtilsMessengerEXT,
) -> Result;

pub type PfnCmdInsertDebugUtilsLabelEXT = extern "system" fn(
    command_buffer: CommandBuffer,
    p_label_info: *const DebugUtilsLabelEXT,
);

pub type PfnCmdEndDebugUtilsLabelEXT = extern "system" fn(
    command_buffer: CommandBuffer,
);

pub type PfnCmdBeginDebugUtilsLabelEXT = extern "system" fn(
    command_buffer: CommandBuffer,
    p_label_info: *const DebugUtilsLabelEXT,
);

pub type PfnQueueInsertDebugUtilsLabelEXT = extern "system" fn(
    queue: Queue,
    p_label_info: *const DebugUtilsLabelEXT,
);

pub type PfnQueueEndDebugUtilsLabelEXT = extern "system" fn(
    queue: Queue,
);

pub type PfnQueueBeginDebugUtilsLabelEXT = extern "system" fn(
    queue: Queue,
    p_label_info: *const DebugUtilsLabelEXT,
);

pub type PfnSetDebugUtilsObjectTagEXT = extern "system" fn(
    device: Device,
    p_tag_info: *const DebugUtilsObjectTagInfoEXT,
) -> Result;

pub type PfnSetDebugUtilsObjectNameEXT = extern "system" fn(
    device: Device,
    p_name_info: *const DebugUtilsObjectNameInfoEXT,
) -> Result;

pub type PfnCmdExecuteCommands = extern "system" fn(
    command_buffer: CommandBuffer,
    command_buffer_count: u32,
    p_command_buffers: *const CommandBuffer,
);

pub type PfnCmdEndRenderPass = extern "system" fn(
    command_buffer: CommandBuffer,
);

pub type PfnCmdNextSubpass = extern "system" fn(
    command_buffer: CommandBuffer,
    contents: SubpassContents,
);

pub type PfnCmdBeginRenderPass = extern "system" fn(
    command_buffer: CommandBuffer,
    p_render_pass_begin: *const RenderPassBeginInfo,
    contents: SubpassContents,
);

pub type PfnCmdPushConstants = extern "system" fn(
    command_buffer: CommandBuffer,
    layout: PipelineLayout,
    stage_flags: ShaderStageFlags,
    offset: u32,
    size: u32,
    p_values: *const core::ffi::c_void,
);

pub type PfnCmdCopyQueryPoolResults = extern "system" fn(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    stride: DeviceSize,
    flags: QueryResultFlags,
);

pub type PfnCmdWriteTimestamp = extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_stage: PipelineStageFlagBits,
    query_pool: QueryPool,
    query: u32,
);

pub type PfnCmdResetQueryPool = extern "system" fn(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
);

pub type PfnCmdEndQuery = extern "system" fn(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    query: u32,
);

pub type PfnCmdBeginQuery = extern "system" fn(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    query: u32,
    flags: QueryControlFlags,
);

pub type PfnCmdPipelineBarrier = extern "system" fn(
    command_buffer: CommandBuffer,
    src_stage_mask: PipelineStageFlags,
    dst_stage_mask: PipelineStageFlags,
    dependency_flags: DependencyFlags,
    memory_barrier_count: u32,
    p_memory_barriers: *const MemoryBarrier,
    buffer_memory_barrier_count: u32,
    p_buffer_memory_barriers: *const BufferMemoryBarrier,
    image_memory_barrier_count: u32,
    p_image_memory_barriers: *const ImageMemoryBarrier,
);

pub type PfnCmdWaitEvents = extern "system" fn(
    command_buffer: CommandBuffer,
    event_count: u32,
    p_events: *const Event,
    src_stage_mask: PipelineStageFlags,
    dst_stage_mask: PipelineStageFlags,
    memory_barrier_count: u32,
    p_memory_barriers: *const MemoryBarrier,
    buffer_memory_barrier_count: u32,
    p_buffer_memory_barriers: *const BufferMemoryBarrier,
    image_memory_barrier_count: u32,
    p_image_memory_barriers: *const ImageMemoryBarrier,
);

pub type PfnCmdResetEvent = extern "system" fn(
    command_buffer: CommandBuffer,
    event: Event,
    stage_mask: PipelineStageFlags,
);

pub type PfnCmdSetEvent = extern "system" fn(
    command_buffer: CommandBuffer,
    event: Event,
    stage_mask: PipelineStageFlags,
);

pub type PfnCmdResolveImage = extern "system" fn(
    command_buffer: CommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    p_regions: *const ImageResolve,
);

pub type PfnCmdClearAttachments = extern "system" fn(
    command_buffer: CommandBuffer,
    attachment_count: u32,
    p_attachments: *const ClearAttachment,
    rect_count: u32,
    p_rects: *const ClearRect,
);

pub type PfnCmdClearDepthStencilImage = extern "system" fn(
    command_buffer: CommandBuffer,
    image: Image,
    image_layout: ImageLayout,
    p_depth_stencil: *const ClearDepthStencilValue,
    range_count: u32,
    p_ranges: *const ImageSubresourceRange,
);

pub type PfnCmdClearColorImage = extern "system" fn(
    command_buffer: CommandBuffer,
    image: Image,
    image_layout: ImageLayout,
    p_color: *const ClearColorValue,
    range_count: u32,
    p_ranges: *const ImageSubresourceRange,
);

pub type PfnCmdFillBuffer = extern "system" fn(
    command_buffer: CommandBuffer,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    size: DeviceSize,
    data: u32,
);

pub type PfnCmdUpdateBuffer = extern "system" fn(
    command_buffer: CommandBuffer,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    data_size: DeviceSize,
    p_data: *const core::ffi::c_void,
);

pub type PfnCmdCopyImageToBuffer = extern "system" fn(
    command_buffer: CommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_buffer: Buffer,
    region_count: u32,
    p_regions: *const BufferImageCopy,
);

pub type PfnCmdCopyBufferToImage = extern "system" fn(
    command_buffer: CommandBuffer,
    src_buffer: Buffer,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    p_regions: *const BufferImageCopy,
);

pub type PfnCmdBlitImage = extern "system" fn(
    command_buffer: CommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    p_regions: *const ImageBlit,
    filter: Filter,
);

pub type PfnCmdCopyImage = extern "system" fn(
    command_buffer: CommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    p_regions: *const ImageCopy,
);

pub type PfnCmdCopyBuffer = extern "system" fn(
    command_buffer: CommandBuffer,
    src_buffer: Buffer,
    dst_buffer: Buffer,
    region_count: u32,
    p_regions: *const BufferCopy,
);

pub type PfnCmdDispatchIndirect = extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
);

pub type PfnCmdDispatch = extern "system" fn(
    command_buffer: CommandBuffer,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
);

pub type PfnCmdDrawIndexedIndirect = extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
);

pub type PfnCmdDrawIndirect = extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
);

pub type PfnCmdDrawIndexed = extern "system" fn(
    command_buffer: CommandBuffer,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    vertex_offset: i32,
    first_instance: u32,
);

pub type PfnCmdDraw = extern "system" fn(
    command_buffer: CommandBuffer,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
);

pub type PfnCmdBindVertexBuffers = extern "system" fn(
    command_buffer: CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    p_buffers: *const Buffer,
    p_offsets: *const DeviceSize,
);

pub type PfnCmdBindIndexBuffer = extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    index_type: IndexType,
);

pub type PfnCmdBindDescriptorSets = extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    first_set: u32,
    descriptor_set_count: u32,
    p_descriptor_sets: *const DescriptorSet,
    dynamic_offset_count: u32,
    p_dynamic_offsets: *const u32,
);

pub type PfnCmdSetStencilReference = extern "system" fn(
    command_buffer: CommandBuffer,
    face_mask: StencilFaceFlags,
    reference: u32,
);

pub type PfnCmdSetStencilWriteMask = extern "system" fn(
    command_buffer: CommandBuffer,
    face_mask: StencilFaceFlags,
    write_mask: u32,
);

pub type PfnCmdSetStencilCompareMask = extern "system" fn(
    command_buffer: CommandBuffer,
    face_mask: StencilFaceFlags,
    compare_mask: u32,
);

pub type PfnCmdSetDepthBounds = extern "system" fn(
    command_buffer: CommandBuffer,
    min_depth_bounds: f32,
    max_depth_bounds: f32,
);

pub type PfnCmdSetBlendConstants = extern "system" fn(
    command_buffer: CommandBuffer,
    blend_constants: [f32; 4],
);

pub type PfnCmdSetDepthBias = extern "system" fn(
    command_buffer: CommandBuffer,
    depth_bias_constant_factor: f32,
    depth_bias_clamp: f32,
    depth_bias_slope_factor: f32,
);

pub type PfnCmdSetLineWidth = extern "system" fn(
    command_buffer: CommandBuffer,
    line_width: f32,
);

pub type PfnCmdSetScissor = extern "system" fn(
    command_buffer: CommandBuffer,
    first_scissor: u32,
    scissor_count: u32,
    p_scissors: *const Rect2D,
);

pub type PfnCmdSetViewport = extern "system" fn(
    command_buffer: CommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    p_viewports: *const Viewport,
);

pub type PfnCmdBindPipeline = extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    pipeline: Pipeline,
);

pub type PfnResetCommandBuffer = extern "system" fn(
    command_buffer: CommandBuffer,
    flags: CommandBufferResetFlags,
) -> Result;

pub type PfnEndCommandBuffer = extern "system" fn(
    command_buffer: CommandBuffer,
) -> Result;

pub type PfnBeginCommandBuffer = extern "system" fn(
    command_buffer: CommandBuffer,
    p_begin_info: *const CommandBufferBeginInfo,
) -> Result;

pub type PfnFreeCommandBuffers = extern "system" fn(
    device: Device,
    command_pool: CommandPool,
    command_buffer_count: u32,
    p_command_buffers: *const CommandBuffer,
);

pub type PfnAllocateCommandBuffers = extern "system" fn(
    device: Device,
    p_allocate_info: *const CommandBufferAllocateInfo,
    p_command_buffers: *mut CommandBuffer,
) -> Result;

pub type PfnResetCommandPool = extern "system" fn(
    device: Device,
    command_pool: CommandPool,
    flags: CommandPoolResetFlags,
) -> Result;

pub type PfnDestroyCommandPool = extern "system" fn(
    device: Device,
    command_pool: CommandPool,
    p_allocator: *const AllocationCallbacks,
);

pub type PfnCreateCommandPool = extern "system" fn(
    device: Device,
    p_create_info: *const CommandPoolCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_command_pool: *mut CommandPool,
) -> Result;

pub type PfnGetRenderAreaGranularity = extern "system" fn(
    device: Device,
    render_pass: RenderPass,
    p_granularity: *mut Extent2D,
);

pub type PfnDestroyRenderPass = extern "system" fn(
    device: Device,
    render_pass: RenderPass,
    p_allocator: *const AllocationCallbacks,
);

pub type PfnCreateRenderPass = extern "system" fn(
    device: Device,
    p_create_info: *const RenderPassCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_render_pass: *mut RenderPass,
) -> Result;

pub type PfnDestroyFramebuffer = extern "system" fn(
    device: Device,
    framebuffer: Framebuffer,
    p_allocator: *const AllocationCallbacks,
);

pub type PfnCreateFramebuffer = extern "system" fn(
    device: Device,
    p_create_info: *const FramebufferCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_framebuffer: *mut Framebuffer,
) -> Result;

pub type PfnUpdateDescriptorSets = extern "system" fn(
    device: Device,
    descriptor_write_count: u32,
    p_descriptor_writes: *const WriteDescriptorSet,
    descriptor_copy_count: u32,
    p_descriptor_copies: *const CopyDescriptorSet,
);

pub type PfnFreeDescriptorSets = extern "system" fn(
    device: Device,
    descriptor_pool: DescriptorPool,
    descriptor_set_count: u32,
    p_descriptor_sets: *const DescriptorSet,
) -> Result;

pub type PfnAllocateDescriptorSets = extern "system" fn(
    device: Device,
    p_allocate_info: *const DescriptorSetAllocateInfo,
    p_descriptor_sets: *mut DescriptorSet,
) -> Result;

pub type PfnResetDescriptorPool = extern "system" fn(
    device: Device,
    descriptor_pool: DescriptorPool,
    flags: DescriptorPoolResetFlags,
) -> Result;

pub type PfnDestroyDescriptorPool = extern "system" fn(
    device: Device,
    descriptor_pool: DescriptorPool,
    p_allocator: *const AllocationCallbacks,
);

pub type PfnCreateDescriptorPool = extern "system" fn(
    device: Device,
    p_create_info: *const DescriptorPoolCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_descriptor_pool: *mut DescriptorPool,
) -> Result;

pub type PfnDestroyDescriptorSetLayout = extern "system" fn(
    device: Device,
    descriptor_set_layout: DescriptorSetLayout,
    p_allocator: *const AllocationCallbacks,
);

pub type PfnCreateDescriptorSetLayout = extern "system" fn(
    device: Device,
    p_create_info: *const DescriptorSetLayoutCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_set_layout: *mut DescriptorSetLayout,
) -> Result;

pub type PfnDestroySampler = extern "system" fn(
    device: Device,
    sampler: Sampler,
    p_allocator: *const AllocationCallbacks,
);

pub type PfnCreateSampler = extern "system" fn(
    device: Device,
    p_create_info: *const SamplerCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_sampler: *mut Sampler,
) -> Result;

pub type PfnDestroyPipelineLayout = extern "system" fn(
    device: Device,
    pipeline_layout: PipelineLayout,
    p_allocator: *const AllocationCallbacks,
);

pub type PfnCreatePipelineLayout = extern "system" fn(
    device: Device,
    p_create_info: *const PipelineLayoutCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_pipeline_layout: *mut PipelineLayout,
) -> Result;

pub type PfnDestroyPipeline = extern "system" fn(
    device: Device,
    pipeline: Pipeline,
    p_allocator: *const AllocationCallbacks,
);

pub type PfnCreateComputePipelines = extern "system" fn(
    device: Device,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    p_create_infos: *const ComputePipelineCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_pipelines: *mut Pipeline,
) -> Result;

pub type PfnCreateGraphicsPipelines = extern "system" fn(
    device: Device,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    p_create_infos: *const GraphicsPipelineCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_pipelines: *mut Pipeline,
) -> Result;

pub type PfnMergePipelineCaches = extern "system" fn(
    device: Device,
    dst_cache: PipelineCache,
    src_cache_count: u32,
    p_src_caches: *const PipelineCache,
) -> Result;

pub type PfnGetPipelineCacheData = extern "system" fn(
    device: Device,
    pipeline_cache: PipelineCache,
    p_data_size: *mut usize,
    p_data: *mut core::ffi::c_void,
) -> Result;

pub type PfnDestroyPipelineCache = extern "system" fn(
    device: Device,
    pipeline_cache: PipelineCache,
    p_allocator: *const AllocationCallbacks,
);

pub type PfnCreatePipelineCache = extern "system" fn(
    device: Device,
    p_create_info: *const PipelineCacheCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_pipeline_cache: *mut PipelineCache,
) -> Result;

pub type PfnDestroyShaderModule = extern "system" fn(
    device: Device,
    shader_module: ShaderModule,
    p_allocator: *const AllocationCallbacks,
);

pub type PfnCreateShaderModule = extern "system" fn(
    device: Device,
    p_create_info: *const ShaderModuleCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_shader_module: *mut ShaderModule,
) -> Result;

pub type PfnDestroyImageView = extern "system" fn(
    device: Device,
    image_view: ImageView,
    p_allocator: *const AllocationCallbacks,
);

pub type PfnCreateImageView = extern "system" fn(
    device: Device,
    p_create_info: *const ImageViewCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_view: *mut ImageView,
) -> Result;

pub type PfnGetImageSubresourceLayout = extern "system" fn(
    device: Device,
    image: Image,
    p_subresource: *const ImageSubresource,
    p_layout: *mut SubresourceLayout,
);

pub type PfnDestroyImage = extern "system" fn(
    device: Device,
    image: Image,
    p_allocator: *const AllocationCallbacks,
);

pub type PfnCreateImage = extern "system" fn(
    device: Device,
    p_create_info: *const ImageCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_image: *mut Image,
) -> Result;

pub type PfnDestroyBufferView = extern "system" fn(
    device: Device,
    buffer_view: BufferView,
    p_allocator: *const AllocationCallbacks,
);

pub type PfnCreateBufferView = extern "system" fn(
    device: Device,
    p_create_info: *const BufferViewCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_view: *mut BufferView,
) -> Result;

pub type PfnDestroyBuffer = extern "system" fn(
    device: Device,
    buffer: Buffer,
    p_allocator: *const AllocationCallbacks,
);

pub type PfnCreateBuffer = extern "system" fn(
    device: Device,
    p_create_info: *const BufferCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_buffer: *mut Buffer,
) -> Result;

pub type PfnGetQueryPoolResults = extern "system" fn(
    device: Device,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
    data_size: usize,
    p_data: *mut core::ffi::c_void,
    stride: DeviceSize,
    flags: QueryResultFlags,
) -> Result;

pub type PfnDestroyQueryPool = extern "system" fn(
    device: Device,
    query_pool: QueryPool,
    p_allocator: *const AllocationCallbacks,
);

pub type PfnCreateQueryPool = extern "system" fn(
    device: Device,
    p_create_info: *const QueryPoolCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_query_pool: *mut QueryPool,
) -> Result;

pub type PfnResetEvent = extern "system" fn(
    device: Device,
    event: Event,
) -> Result;

pub type PfnSetEvent = extern "system" fn(
    device: Device,
    event: Event,
) -> Result;

pub type PfnGetEventStatus = extern "system" fn(
    device: Device,
    event: Event,
) -> Result;

pub type PfnDestroyEvent = extern "system" fn(
    device: Device,
    event: Event,
    p_allocator: *const AllocationCallbacks,
);

pub type PfnCreateEvent = extern "system" fn(
    device: Device,
    p_create_info: *const EventCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_event: *mut Event,
) -> Result;

pub type PfnDestroySemaphore = extern "system" fn(
    device: Device,
    semaphore: Semaphore,
    p_allocator: *const AllocationCallbacks,
);

pub type PfnCreateSemaphore = extern "system" fn(
    device: Device,
    p_create_info: *const SemaphoreCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_semaphore: *mut Semaphore,
) -> Result;

pub type PfnWaitForFences = extern "system" fn(
    device: Device,
    fence_count: u32,
    p_fences: *const Fence,
    wait_all: Bool32,
    timeout: u64,
) -> Result;

pub type PfnGetFenceStatus = extern "system" fn(
    device: Device,
    fence: Fence,
) -> Result;

pub type PfnResetFences = extern "system" fn(
    device: Device,
    fence_count: u32,
    p_fences: *const Fence,
) -> Result;

pub type PfnDestroyFence = extern "system" fn(
    device: Device,
    fence: Fence,
    p_allocator: *const AllocationCallbacks,
);

pub type PfnCreateFence = extern "system" fn(
    device: Device,
    p_create_info: *const FenceCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_fence: *mut Fence,
) -> Result;

pub type PfnQueueBindSparse = extern "system" fn(
    queue: Queue,
    bind_info_count: u32,
    p_bind_info: *const BindSparseInfo,
    fence: Fence,
) -> Result;

pub type PfnGetPhysicalDeviceSparseImageFormatProperties = extern "system" fn(
    physical_device: PhysicalDevice,
    format: Format,
    kind: ImageType,
    samples: SampleCountFlagBits,
    usage: ImageUsageFlags,
    tiling: ImageTiling,
    p_property_count: *mut u32,
    p_properties: *mut SparseImageFormatProperties,
);

pub type PfnGetImageSparseMemoryRequirements = extern "system" fn(
    device: Device,
    image: Image,
    p_sparse_memory_requirement_count: *mut u32,
    p_sparse_memory_requirements: *mut SparseImageMemoryRequirements,
);

pub type PfnGetImageMemoryRequirements = extern "system" fn(
    device: Device,
    image: Image,
    p_memory_requirements: *mut MemoryRequirements,
);

pub type PfnGetBufferMemoryRequirements = extern "system" fn(
    device: Device,
    buffer: Buffer,
    p_memory_requirements: *mut MemoryRequirements,
);

pub type PfnBindImageMemory = extern "system" fn(
    device: Device,
    image: Image,
    memory: DeviceMemory,
    memory_offset: DeviceSize,
) -> Result;

pub type PfnBindBufferMemory = extern "system" fn(
    device: Device,
    buffer: Buffer,
    memory: DeviceMemory,
    memory_offset: DeviceSize,
) -> Result;

pub type PfnGetDeviceMemoryCommitment = extern "system" fn(
    device: Device,
    memory: DeviceMemory,
    p_committed_memory_in_bytes: *mut DeviceSize,
);

pub type PfnInvalidateMappedMemoryRanges = extern "system" fn(
    device: Device,
    memory_range_count: u32,
    p_memory_ranges: *const MappedMemoryRange,
) -> Result;

pub type PfnFlushMappedMemoryRanges = extern "system" fn(
    device: Device,
    memory_range_count: u32,
    p_memory_ranges: *const MappedMemoryRange,
) -> Result;

pub type PfnUnmapMemory = extern "system" fn(
    device: Device,
    memory: DeviceMemory,
);

pub type PfnMapMemory = extern "system" fn(
    device: Device,
    memory: DeviceMemory,
    offset: DeviceSize,
    size: DeviceSize,
    flags: MemoryMapFlags,
    pp_data: *mut *mut core::ffi::c_void,
) -> Result;

pub type PfnFreeMemory = extern "system" fn(
    device: Device,
    memory: DeviceMemory,
    p_allocator: *const AllocationCallbacks,
);

pub type PfnAllocateMemory = extern "system" fn(
    device: Device,
    p_allocate_info: *const MemoryAllocateInfo,
    p_allocator: *const AllocationCallbacks,
    p_memory: *mut DeviceMemory,
) -> Result;

pub type PfnDeviceWaitIdle = extern "system" fn(
    device: Device,
) -> Result;

pub type PfnQueueWaitIdle = extern "system" fn(
    queue: Queue,
) -> Result;

pub type PfnQueueSubmit = extern "system" fn(
    queue: Queue,
    submit_count: u32,
    p_submits: *const SubmitInfo,
    fence: Fence,
) -> Result;

pub type PfnGetDeviceQueue = extern "system" fn(
    device: Device,
    queue_family_index: u32,
    queue_index: u32,
    p_queue: *mut Queue,
);

pub type PfnEnumerateDeviceLayerProperties = extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut LayerProperties,
) -> Result;

pub type PfnEnumerateInstanceLayerProperties = extern "system" fn(
    p_property_count: *mut u32,
    p_properties: *mut LayerProperties,
) -> Result;

pub type PfnEnumerateDeviceExtensionProperties = extern "system" fn(
    physical_device: PhysicalDevice,
    p_layer_name: *const u8,
    p_property_count: *mut u32,
    p_properties: *mut ExtensionProperties,
) -> Result;

pub type PfnEnumerateInstanceExtensionProperties = extern "system" fn(
    p_layer_name: *const u8,
    p_property_count: *mut u32,
    p_properties: *mut ExtensionProperties,
) -> Result;

pub type PfnDestroyDevice = extern "system" fn(
    device: Device,
    p_allocator: *const AllocationCallbacks,
);

pub type PfnCreateDevice = extern "system" fn(
    physical_device: PhysicalDevice,
    p_create_info: *const DeviceCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_device: *mut Device,
) -> Result;

pub type PfnGetDeviceProcAddr = extern "system" fn(
    device: Device,
    p_name: *const u8,
) -> PfnVoidFunction;

pub type PfnGetInstanceProcAddr = extern "system" fn(
    instance: Instance,
    p_name: *const u8,
) -> PfnVoidFunction;

pub type PfnGetPhysicalDeviceMemoryProperties = extern "system" fn(
    physical_device: PhysicalDevice,
    p_memory_properties: *mut PhysicalDeviceMemoryProperties,
);

pub type PfnGetPhysicalDeviceQueueFamilyProperties = extern "system" fn(
    physical_device: PhysicalDevice,
    p_queue_family_property_count: *mut u32,
    p_queue_family_properties: *mut QueueFamilyProperties,
);

pub type PfnGetPhysicalDeviceProperties = extern "system" fn(
    physical_device: PhysicalDevice,
    p_properties: *mut PhysicalDeviceProperties,
);

pub type PfnGetPhysicalDeviceImageFormatProperties = extern "system" fn(
    physical_device: PhysicalDevice,
    format: Format,
    kind: ImageType,
    tiling: ImageTiling,
    usage: ImageUsageFlags,
    flags: ImageCreateFlags,
    p_image_format_properties: *mut ImageFormatProperties,
) -> Result;

pub type PfnGetPhysicalDeviceFormatProperties = extern "system" fn(
    physical_device: PhysicalDevice,
    format: Format,
    p_format_properties: *mut FormatProperties,
);

pub type PfnGetPhysicalDeviceFeatures = extern "system" fn(
    physical_device: PhysicalDevice,
    p_features: *mut PhysicalDeviceFeatures,
);

pub type PfnEnumeratePhysicalDevices = extern "system" fn(
    instance: Instance,
    p_physical_device_count: *mut u32,
    p_physical_devices: *mut PhysicalDevice,
) -> Result;

pub type PfnDestroyInstance = extern "system" fn(
    instance: Instance,
    p_allocator: *const AllocationCallbacks,
);

pub type PfnCreateInstance = extern "system" fn(
    p_create_info: *const InstanceCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_instance: *mut Instance,
) -> Result;

