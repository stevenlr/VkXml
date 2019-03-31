use crate::types::*;

pub struct VkAllocationCallbacksBuilder {
    s: VkAllocationCallbacks,
}

impl VkAllocationCallbacksBuilder {
    pub fn new() -> Self {
        Self {
            s: VkAllocationCallbacks::default(),
        }
    }
}

impl core::ops::Deref for VkAllocationCallbacksBuilder {
    type Target = VkAllocationCallbacks;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkAllocationCallbacksBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkWin32SurfaceCreateInfoKHRBuilder {
    s: VkWin32SurfaceCreateInfoKHR,
}

impl VkWin32SurfaceCreateInfoKHRBuilder {
    pub fn new() -> Self {
        Self {
            s: VkWin32SurfaceCreateInfoKHR::default(),
        }
    }
}

impl core::ops::Deref for VkWin32SurfaceCreateInfoKHRBuilder {
    type Target = VkWin32SurfaceCreateInfoKHR;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkWin32SurfaceCreateInfoKHRBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSurfaceFormatKHRBuilder {
    s: VkSurfaceFormatKHR,
}

impl VkSurfaceFormatKHRBuilder {
    pub fn new() -> Self {
        Self {
            s: VkSurfaceFormatKHR::default(),
        }
    }
}

impl core::ops::Deref for VkSurfaceFormatKHRBuilder {
    type Target = VkSurfaceFormatKHR;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkSurfaceFormatKHRBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSurfaceCapabilitiesKHRBuilder {
    s: VkSurfaceCapabilitiesKHR,
}

impl VkSurfaceCapabilitiesKHRBuilder {
    pub fn new() -> Self {
        Self {
            s: VkSurfaceCapabilitiesKHR::default(),
        }
    }
}

impl core::ops::Deref for VkSurfaceCapabilitiesKHRBuilder {
    type Target = VkSurfaceCapabilitiesKHR;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkSurfaceCapabilitiesKHRBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkExtent2DBuilder {
    s: VkExtent2D,
}

impl VkExtent2DBuilder {
    pub fn new() -> Self {
        Self {
            s: VkExtent2D::default(),
        }
    }
}

impl core::ops::Deref for VkExtent2DBuilder {
    type Target = VkExtent2D;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkExtent2DBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPresentInfoKHRBuilder {
    s: VkPresentInfoKHR,
}

impl VkPresentInfoKHRBuilder {
    pub fn new() -> Self {
        Self {
            s: VkPresentInfoKHR::default(),
        }
    }
}

impl core::ops::Deref for VkPresentInfoKHRBuilder {
    type Target = VkPresentInfoKHR;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkPresentInfoKHRBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSwapchainCreateInfoKHRBuilder {
    s: VkSwapchainCreateInfoKHR,
}

impl VkSwapchainCreateInfoKHRBuilder {
    pub fn new() -> Self {
        Self {
            s: VkSwapchainCreateInfoKHR::default(),
        }
    }
}

impl core::ops::Deref for VkSwapchainCreateInfoKHRBuilder {
    type Target = VkSwapchainCreateInfoKHR;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkSwapchainCreateInfoKHRBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDebugUtilsMessengerCallbackDataEXTBuilder {
    s: VkDebugUtilsMessengerCallbackDataEXT,
}

impl VkDebugUtilsMessengerCallbackDataEXTBuilder {
    pub fn new() -> Self {
        Self {
            s: VkDebugUtilsMessengerCallbackDataEXT::default(),
        }
    }
}

impl core::ops::Deref for VkDebugUtilsMessengerCallbackDataEXTBuilder {
    type Target = VkDebugUtilsMessengerCallbackDataEXT;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkDebugUtilsMessengerCallbackDataEXTBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDebugUtilsObjectNameInfoEXTBuilder {
    s: VkDebugUtilsObjectNameInfoEXT,
}

impl VkDebugUtilsObjectNameInfoEXTBuilder {
    pub fn new() -> Self {
        Self {
            s: VkDebugUtilsObjectNameInfoEXT::default(),
        }
    }
}

impl core::ops::Deref for VkDebugUtilsObjectNameInfoEXTBuilder {
    type Target = VkDebugUtilsObjectNameInfoEXT;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkDebugUtilsObjectNameInfoEXTBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDebugUtilsLabelEXTBuilder {
    s: VkDebugUtilsLabelEXT,
}

impl VkDebugUtilsLabelEXTBuilder {
    pub fn new() -> Self {
        Self {
            s: VkDebugUtilsLabelEXT::default(),
        }
    }
}

impl core::ops::Deref for VkDebugUtilsLabelEXTBuilder {
    type Target = VkDebugUtilsLabelEXT;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkDebugUtilsLabelEXTBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDebugUtilsMessengerCreateInfoEXTBuilder {
    s: VkDebugUtilsMessengerCreateInfoEXT,
}

impl VkDebugUtilsMessengerCreateInfoEXTBuilder {
    pub fn new() -> Self {
        Self {
            s: VkDebugUtilsMessengerCreateInfoEXT::default(),
        }
    }
}

impl core::ops::Deref for VkDebugUtilsMessengerCreateInfoEXTBuilder {
    type Target = VkDebugUtilsMessengerCreateInfoEXT;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkDebugUtilsMessengerCreateInfoEXTBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDebugUtilsObjectTagInfoEXTBuilder {
    s: VkDebugUtilsObjectTagInfoEXT,
}

impl VkDebugUtilsObjectTagInfoEXTBuilder {
    pub fn new() -> Self {
        Self {
            s: VkDebugUtilsObjectTagInfoEXT::default(),
        }
    }
}

impl core::ops::Deref for VkDebugUtilsObjectTagInfoEXTBuilder {
    type Target = VkDebugUtilsObjectTagInfoEXT;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkDebugUtilsObjectTagInfoEXTBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkBaseInStructureBuilder {
    s: VkBaseInStructure,
}

impl VkBaseInStructureBuilder {
    pub fn new() -> Self {
        Self {
            s: VkBaseInStructure::default(),
        }
    }
}

impl core::ops::Deref for VkBaseInStructureBuilder {
    type Target = VkBaseInStructure;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkBaseInStructureBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkBaseOutStructureBuilder {
    s: VkBaseOutStructure,
}

impl VkBaseOutStructureBuilder {
    pub fn new() -> Self {
        Self {
            s: VkBaseOutStructure::default(),
        }
    }
}

impl core::ops::Deref for VkBaseOutStructureBuilder {
    type Target = VkBaseOutStructure;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkBaseOutStructureBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkMemoryBarrierBuilder {
    s: VkMemoryBarrier,
}

impl VkMemoryBarrierBuilder {
    pub fn new() -> Self {
        Self {
            s: VkMemoryBarrier::default(),
        }
    }
}

impl core::ops::Deref for VkMemoryBarrierBuilder {
    type Target = VkMemoryBarrier;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkMemoryBarrierBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkImageMemoryBarrierBuilder {
    s: VkImageMemoryBarrier,
}

impl VkImageMemoryBarrierBuilder {
    pub fn new() -> Self {
        Self {
            s: VkImageMemoryBarrier::default(),
        }
    }
}

impl core::ops::Deref for VkImageMemoryBarrierBuilder {
    type Target = VkImageMemoryBarrier;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkImageMemoryBarrierBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkImageSubresourceRangeBuilder {
    s: VkImageSubresourceRange,
}

impl VkImageSubresourceRangeBuilder {
    pub fn new() -> Self {
        Self {
            s: VkImageSubresourceRange::default(),
        }
    }
}

impl core::ops::Deref for VkImageSubresourceRangeBuilder {
    type Target = VkImageSubresourceRange;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkImageSubresourceRangeBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDrawIndirectCommandBuilder {
    s: VkDrawIndirectCommand,
}

impl VkDrawIndirectCommandBuilder {
    pub fn new() -> Self {
        Self {
            s: VkDrawIndirectCommand::default(),
        }
    }
}

impl core::ops::Deref for VkDrawIndirectCommandBuilder {
    type Target = VkDrawIndirectCommand;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkDrawIndirectCommandBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDrawIndexedIndirectCommandBuilder {
    s: VkDrawIndexedIndirectCommand,
}

impl VkDrawIndexedIndirectCommandBuilder {
    pub fn new() -> Self {
        Self {
            s: VkDrawIndexedIndirectCommand::default(),
        }
    }
}

impl core::ops::Deref for VkDrawIndexedIndirectCommandBuilder {
    type Target = VkDrawIndexedIndirectCommand;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkDrawIndexedIndirectCommandBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDispatchIndirectCommandBuilder {
    s: VkDispatchIndirectCommand,
}

impl VkDispatchIndirectCommandBuilder {
    pub fn new() -> Self {
        Self {
            s: VkDispatchIndirectCommand::default(),
        }
    }
}

impl core::ops::Deref for VkDispatchIndirectCommandBuilder {
    type Target = VkDispatchIndirectCommand;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkDispatchIndirectCommandBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkBufferMemoryBarrierBuilder {
    s: VkBufferMemoryBarrier,
}

impl VkBufferMemoryBarrierBuilder {
    pub fn new() -> Self {
        Self {
            s: VkBufferMemoryBarrier::default(),
        }
    }
}

impl core::ops::Deref for VkBufferMemoryBarrierBuilder {
    type Target = VkBufferMemoryBarrier;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkBufferMemoryBarrierBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkRenderPassBeginInfoBuilder {
    s: VkRenderPassBeginInfo,
}

impl VkRenderPassBeginInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkRenderPassBeginInfo::default(),
        }
    }
}

impl core::ops::Deref for VkRenderPassBeginInfoBuilder {
    type Target = VkRenderPassBeginInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkRenderPassBeginInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkClearDepthStencilValueBuilder {
    s: VkClearDepthStencilValue,
}

impl VkClearDepthStencilValueBuilder {
    pub fn new() -> Self {
        Self {
            s: VkClearDepthStencilValue::default(),
        }
    }
}

impl core::ops::Deref for VkClearDepthStencilValueBuilder {
    type Target = VkClearDepthStencilValue;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkClearDepthStencilValueBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkRect2DBuilder {
    s: VkRect2D,
}

impl VkRect2DBuilder {
    pub fn new() -> Self {
        Self {
            s: VkRect2D::default(),
        }
    }
}

impl core::ops::Deref for VkRect2DBuilder {
    type Target = VkRect2D;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkRect2DBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkOffset2DBuilder {
    s: VkOffset2D,
}

impl VkOffset2DBuilder {
    pub fn new() -> Self {
        Self {
            s: VkOffset2D::default(),
        }
    }
}

impl core::ops::Deref for VkOffset2DBuilder {
    type Target = VkOffset2D;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkOffset2DBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkImageResolveBuilder {
    s: VkImageResolve,
}

impl VkImageResolveBuilder {
    pub fn new() -> Self {
        Self {
            s: VkImageResolve::default(),
        }
    }
}

impl core::ops::Deref for VkImageResolveBuilder {
    type Target = VkImageResolve;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkImageResolveBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkExtent3DBuilder {
    s: VkExtent3D,
}

impl VkExtent3DBuilder {
    pub fn new() -> Self {
        Self {
            s: VkExtent3D::default(),
        }
    }
}

impl core::ops::Deref for VkExtent3DBuilder {
    type Target = VkExtent3D;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkExtent3DBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkOffset3DBuilder {
    s: VkOffset3D,
}

impl VkOffset3DBuilder {
    pub fn new() -> Self {
        Self {
            s: VkOffset3D::default(),
        }
    }
}

impl core::ops::Deref for VkOffset3DBuilder {
    type Target = VkOffset3D;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkOffset3DBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkImageSubresourceLayersBuilder {
    s: VkImageSubresourceLayers,
}

impl VkImageSubresourceLayersBuilder {
    pub fn new() -> Self {
        Self {
            s: VkImageSubresourceLayers::default(),
        }
    }
}

impl core::ops::Deref for VkImageSubresourceLayersBuilder {
    type Target = VkImageSubresourceLayers;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkImageSubresourceLayersBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkClearRectBuilder {
    s: VkClearRect,
}

impl VkClearRectBuilder {
    pub fn new() -> Self {
        Self {
            s: VkClearRect::default(),
        }
    }
}

impl core::ops::Deref for VkClearRectBuilder {
    type Target = VkClearRect;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkClearRectBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkClearAttachmentBuilder {
    s: VkClearAttachment,
}

impl VkClearAttachmentBuilder {
    pub fn new() -> Self {
        Self {
            s: VkClearAttachment::default(),
        }
    }
}

impl core::ops::Deref for VkClearAttachmentBuilder {
    type Target = VkClearAttachment;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkClearAttachmentBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkBufferImageCopyBuilder {
    s: VkBufferImageCopy,
}

impl VkBufferImageCopyBuilder {
    pub fn new() -> Self {
        Self {
            s: VkBufferImageCopy::default(),
        }
    }
}

impl core::ops::Deref for VkBufferImageCopyBuilder {
    type Target = VkBufferImageCopy;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkBufferImageCopyBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkImageBlitBuilder {
    s: VkImageBlit,
}

impl VkImageBlitBuilder {
    pub fn new() -> Self {
        Self {
            s: VkImageBlit::default(),
        }
    }
}

impl core::ops::Deref for VkImageBlitBuilder {
    type Target = VkImageBlit;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkImageBlitBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkImageCopyBuilder {
    s: VkImageCopy,
}

impl VkImageCopyBuilder {
    pub fn new() -> Self {
        Self {
            s: VkImageCopy::default(),
        }
    }
}

impl core::ops::Deref for VkImageCopyBuilder {
    type Target = VkImageCopy;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkImageCopyBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkBufferCopyBuilder {
    s: VkBufferCopy,
}

impl VkBufferCopyBuilder {
    pub fn new() -> Self {
        Self {
            s: VkBufferCopy::default(),
        }
    }
}

impl core::ops::Deref for VkBufferCopyBuilder {
    type Target = VkBufferCopy;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkBufferCopyBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkViewportBuilder {
    s: VkViewport,
}

impl VkViewportBuilder {
    pub fn new() -> Self {
        Self {
            s: VkViewport::default(),
        }
    }
}

impl core::ops::Deref for VkViewportBuilder {
    type Target = VkViewport;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkViewportBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkCommandBufferBeginInfoBuilder {
    s: VkCommandBufferBeginInfo,
}

impl VkCommandBufferBeginInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkCommandBufferBeginInfo::default(),
        }
    }
}

impl core::ops::Deref for VkCommandBufferBeginInfoBuilder {
    type Target = VkCommandBufferBeginInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkCommandBufferBeginInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkCommandBufferInheritanceInfoBuilder {
    s: VkCommandBufferInheritanceInfo,
}

impl VkCommandBufferInheritanceInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkCommandBufferInheritanceInfo::default(),
        }
    }
}

impl core::ops::Deref for VkCommandBufferInheritanceInfoBuilder {
    type Target = VkCommandBufferInheritanceInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkCommandBufferInheritanceInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkCommandBufferAllocateInfoBuilder {
    s: VkCommandBufferAllocateInfo,
}

impl VkCommandBufferAllocateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkCommandBufferAllocateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkCommandBufferAllocateInfoBuilder {
    type Target = VkCommandBufferAllocateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkCommandBufferAllocateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkCommandPoolCreateInfoBuilder {
    s: VkCommandPoolCreateInfo,
}

impl VkCommandPoolCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkCommandPoolCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkCommandPoolCreateInfoBuilder {
    type Target = VkCommandPoolCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkCommandPoolCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkRenderPassCreateInfoBuilder {
    s: VkRenderPassCreateInfo,
}

impl VkRenderPassCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkRenderPassCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkRenderPassCreateInfoBuilder {
    type Target = VkRenderPassCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkRenderPassCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSubpassDependencyBuilder {
    s: VkSubpassDependency,
}

impl VkSubpassDependencyBuilder {
    pub fn new() -> Self {
        Self {
            s: VkSubpassDependency::default(),
        }
    }
}

impl core::ops::Deref for VkSubpassDependencyBuilder {
    type Target = VkSubpassDependency;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkSubpassDependencyBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSubpassDescriptionBuilder {
    s: VkSubpassDescription,
}

impl VkSubpassDescriptionBuilder {
    pub fn new() -> Self {
        Self {
            s: VkSubpassDescription::default(),
        }
    }
}

impl core::ops::Deref for VkSubpassDescriptionBuilder {
    type Target = VkSubpassDescription;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkSubpassDescriptionBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkAttachmentReferenceBuilder {
    s: VkAttachmentReference,
}

impl VkAttachmentReferenceBuilder {
    pub fn new() -> Self {
        Self {
            s: VkAttachmentReference::default(),
        }
    }
}

impl core::ops::Deref for VkAttachmentReferenceBuilder {
    type Target = VkAttachmentReference;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkAttachmentReferenceBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkAttachmentDescriptionBuilder {
    s: VkAttachmentDescription,
}

impl VkAttachmentDescriptionBuilder {
    pub fn new() -> Self {
        Self {
            s: VkAttachmentDescription::default(),
        }
    }
}

impl core::ops::Deref for VkAttachmentDescriptionBuilder {
    type Target = VkAttachmentDescription;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkAttachmentDescriptionBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkFramebufferCreateInfoBuilder {
    s: VkFramebufferCreateInfo,
}

impl VkFramebufferCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkFramebufferCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkFramebufferCreateInfoBuilder {
    type Target = VkFramebufferCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkFramebufferCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkCopyDescriptorSetBuilder {
    s: VkCopyDescriptorSet,
}

impl VkCopyDescriptorSetBuilder {
    pub fn new() -> Self {
        Self {
            s: VkCopyDescriptorSet::default(),
        }
    }
}

impl core::ops::Deref for VkCopyDescriptorSetBuilder {
    type Target = VkCopyDescriptorSet;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkCopyDescriptorSetBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkWriteDescriptorSetBuilder {
    s: VkWriteDescriptorSet,
}

impl VkWriteDescriptorSetBuilder {
    pub fn new() -> Self {
        Self {
            s: VkWriteDescriptorSet::default(),
        }
    }
}

impl core::ops::Deref for VkWriteDescriptorSetBuilder {
    type Target = VkWriteDescriptorSet;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkWriteDescriptorSetBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDescriptorBufferInfoBuilder {
    s: VkDescriptorBufferInfo,
}

impl VkDescriptorBufferInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkDescriptorBufferInfo::default(),
        }
    }
}

impl core::ops::Deref for VkDescriptorBufferInfoBuilder {
    type Target = VkDescriptorBufferInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkDescriptorBufferInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDescriptorImageInfoBuilder {
    s: VkDescriptorImageInfo,
}

impl VkDescriptorImageInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkDescriptorImageInfo::default(),
        }
    }
}

impl core::ops::Deref for VkDescriptorImageInfoBuilder {
    type Target = VkDescriptorImageInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkDescriptorImageInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDescriptorSetAllocateInfoBuilder {
    s: VkDescriptorSetAllocateInfo,
}

impl VkDescriptorSetAllocateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkDescriptorSetAllocateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkDescriptorSetAllocateInfoBuilder {
    type Target = VkDescriptorSetAllocateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkDescriptorSetAllocateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDescriptorPoolCreateInfoBuilder {
    s: VkDescriptorPoolCreateInfo,
}

impl VkDescriptorPoolCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkDescriptorPoolCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkDescriptorPoolCreateInfoBuilder {
    type Target = VkDescriptorPoolCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkDescriptorPoolCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDescriptorPoolSizeBuilder {
    s: VkDescriptorPoolSize,
}

impl VkDescriptorPoolSizeBuilder {
    pub fn new() -> Self {
        Self {
            s: VkDescriptorPoolSize::default(),
        }
    }
}

impl core::ops::Deref for VkDescriptorPoolSizeBuilder {
    type Target = VkDescriptorPoolSize;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkDescriptorPoolSizeBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDescriptorSetLayoutCreateInfoBuilder {
    s: VkDescriptorSetLayoutCreateInfo,
}

impl VkDescriptorSetLayoutCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkDescriptorSetLayoutCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkDescriptorSetLayoutCreateInfoBuilder {
    type Target = VkDescriptorSetLayoutCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkDescriptorSetLayoutCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDescriptorSetLayoutBindingBuilder {
    s: VkDescriptorSetLayoutBinding,
}

impl VkDescriptorSetLayoutBindingBuilder {
    pub fn new() -> Self {
        Self {
            s: VkDescriptorSetLayoutBinding::default(),
        }
    }
}

impl core::ops::Deref for VkDescriptorSetLayoutBindingBuilder {
    type Target = VkDescriptorSetLayoutBinding;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkDescriptorSetLayoutBindingBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSamplerCreateInfoBuilder {
    s: VkSamplerCreateInfo,
}

impl VkSamplerCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkSamplerCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkSamplerCreateInfoBuilder {
    type Target = VkSamplerCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkSamplerCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPipelineLayoutCreateInfoBuilder {
    s: VkPipelineLayoutCreateInfo,
}

impl VkPipelineLayoutCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkPipelineLayoutCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkPipelineLayoutCreateInfoBuilder {
    type Target = VkPipelineLayoutCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkPipelineLayoutCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPushConstantRangeBuilder {
    s: VkPushConstantRange,
}

impl VkPushConstantRangeBuilder {
    pub fn new() -> Self {
        Self {
            s: VkPushConstantRange::default(),
        }
    }
}

impl core::ops::Deref for VkPushConstantRangeBuilder {
    type Target = VkPushConstantRange;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkPushConstantRangeBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkComputePipelineCreateInfoBuilder {
    s: VkComputePipelineCreateInfo,
}

impl VkComputePipelineCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkComputePipelineCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkComputePipelineCreateInfoBuilder {
    type Target = VkComputePipelineCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkComputePipelineCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPipelineShaderStageCreateInfoBuilder {
    s: VkPipelineShaderStageCreateInfo,
}

impl VkPipelineShaderStageCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkPipelineShaderStageCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkPipelineShaderStageCreateInfoBuilder {
    type Target = VkPipelineShaderStageCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkPipelineShaderStageCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSpecializationInfoBuilder {
    s: VkSpecializationInfo,
}

impl VkSpecializationInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkSpecializationInfo::default(),
        }
    }
}

impl core::ops::Deref for VkSpecializationInfoBuilder {
    type Target = VkSpecializationInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkSpecializationInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSpecializationMapEntryBuilder {
    s: VkSpecializationMapEntry,
}

impl VkSpecializationMapEntryBuilder {
    pub fn new() -> Self {
        Self {
            s: VkSpecializationMapEntry::default(),
        }
    }
}

impl core::ops::Deref for VkSpecializationMapEntryBuilder {
    type Target = VkSpecializationMapEntry;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkSpecializationMapEntryBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkGraphicsPipelineCreateInfoBuilder {
    s: VkGraphicsPipelineCreateInfo,
}

impl VkGraphicsPipelineCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkGraphicsPipelineCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkGraphicsPipelineCreateInfoBuilder {
    type Target = VkGraphicsPipelineCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkGraphicsPipelineCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPipelineDynamicStateCreateInfoBuilder {
    s: VkPipelineDynamicStateCreateInfo,
}

impl VkPipelineDynamicStateCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkPipelineDynamicStateCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkPipelineDynamicStateCreateInfoBuilder {
    type Target = VkPipelineDynamicStateCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkPipelineDynamicStateCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPipelineColorBlendStateCreateInfoBuilder {
    s: VkPipelineColorBlendStateCreateInfo,
}

impl VkPipelineColorBlendStateCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkPipelineColorBlendStateCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkPipelineColorBlendStateCreateInfoBuilder {
    type Target = VkPipelineColorBlendStateCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkPipelineColorBlendStateCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPipelineColorBlendAttachmentStateBuilder {
    s: VkPipelineColorBlendAttachmentState,
}

impl VkPipelineColorBlendAttachmentStateBuilder {
    pub fn new() -> Self {
        Self {
            s: VkPipelineColorBlendAttachmentState::default(),
        }
    }
}

impl core::ops::Deref for VkPipelineColorBlendAttachmentStateBuilder {
    type Target = VkPipelineColorBlendAttachmentState;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkPipelineColorBlendAttachmentStateBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPipelineDepthStencilStateCreateInfoBuilder {
    s: VkPipelineDepthStencilStateCreateInfo,
}

impl VkPipelineDepthStencilStateCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkPipelineDepthStencilStateCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkPipelineDepthStencilStateCreateInfoBuilder {
    type Target = VkPipelineDepthStencilStateCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkPipelineDepthStencilStateCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkStencilOpStateBuilder {
    s: VkStencilOpState,
}

impl VkStencilOpStateBuilder {
    pub fn new() -> Self {
        Self {
            s: VkStencilOpState::default(),
        }
    }
}

impl core::ops::Deref for VkStencilOpStateBuilder {
    type Target = VkStencilOpState;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkStencilOpStateBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPipelineMultisampleStateCreateInfoBuilder {
    s: VkPipelineMultisampleStateCreateInfo,
}

impl VkPipelineMultisampleStateCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkPipelineMultisampleStateCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkPipelineMultisampleStateCreateInfoBuilder {
    type Target = VkPipelineMultisampleStateCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkPipelineMultisampleStateCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPipelineRasterizationStateCreateInfoBuilder {
    s: VkPipelineRasterizationStateCreateInfo,
}

impl VkPipelineRasterizationStateCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkPipelineRasterizationStateCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkPipelineRasterizationStateCreateInfoBuilder {
    type Target = VkPipelineRasterizationStateCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkPipelineRasterizationStateCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPipelineViewportStateCreateInfoBuilder {
    s: VkPipelineViewportStateCreateInfo,
}

impl VkPipelineViewportStateCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkPipelineViewportStateCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkPipelineViewportStateCreateInfoBuilder {
    type Target = VkPipelineViewportStateCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkPipelineViewportStateCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPipelineTessellationStateCreateInfoBuilder {
    s: VkPipelineTessellationStateCreateInfo,
}

impl VkPipelineTessellationStateCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkPipelineTessellationStateCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkPipelineTessellationStateCreateInfoBuilder {
    type Target = VkPipelineTessellationStateCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkPipelineTessellationStateCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPipelineInputAssemblyStateCreateInfoBuilder {
    s: VkPipelineInputAssemblyStateCreateInfo,
}

impl VkPipelineInputAssemblyStateCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkPipelineInputAssemblyStateCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkPipelineInputAssemblyStateCreateInfoBuilder {
    type Target = VkPipelineInputAssemblyStateCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkPipelineInputAssemblyStateCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPipelineVertexInputStateCreateInfoBuilder {
    s: VkPipelineVertexInputStateCreateInfo,
}

impl VkPipelineVertexInputStateCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkPipelineVertexInputStateCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkPipelineVertexInputStateCreateInfoBuilder {
    type Target = VkPipelineVertexInputStateCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkPipelineVertexInputStateCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkVertexInputAttributeDescriptionBuilder {
    s: VkVertexInputAttributeDescription,
}

impl VkVertexInputAttributeDescriptionBuilder {
    pub fn new() -> Self {
        Self {
            s: VkVertexInputAttributeDescription::default(),
        }
    }
}

impl core::ops::Deref for VkVertexInputAttributeDescriptionBuilder {
    type Target = VkVertexInputAttributeDescription;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkVertexInputAttributeDescriptionBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkVertexInputBindingDescriptionBuilder {
    s: VkVertexInputBindingDescription,
}

impl VkVertexInputBindingDescriptionBuilder {
    pub fn new() -> Self {
        Self {
            s: VkVertexInputBindingDescription::default(),
        }
    }
}

impl core::ops::Deref for VkVertexInputBindingDescriptionBuilder {
    type Target = VkVertexInputBindingDescription;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkVertexInputBindingDescriptionBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPipelineCacheCreateInfoBuilder {
    s: VkPipelineCacheCreateInfo,
}

impl VkPipelineCacheCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkPipelineCacheCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkPipelineCacheCreateInfoBuilder {
    type Target = VkPipelineCacheCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkPipelineCacheCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkShaderModuleCreateInfoBuilder {
    s: VkShaderModuleCreateInfo,
}

impl VkShaderModuleCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkShaderModuleCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkShaderModuleCreateInfoBuilder {
    type Target = VkShaderModuleCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkShaderModuleCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkImageViewCreateInfoBuilder {
    s: VkImageViewCreateInfo,
}

impl VkImageViewCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkImageViewCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkImageViewCreateInfoBuilder {
    type Target = VkImageViewCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkImageViewCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkComponentMappingBuilder {
    s: VkComponentMapping,
}

impl VkComponentMappingBuilder {
    pub fn new() -> Self {
        Self {
            s: VkComponentMapping::default(),
        }
    }
}

impl core::ops::Deref for VkComponentMappingBuilder {
    type Target = VkComponentMapping;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkComponentMappingBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSubresourceLayoutBuilder {
    s: VkSubresourceLayout,
}

impl VkSubresourceLayoutBuilder {
    pub fn new() -> Self {
        Self {
            s: VkSubresourceLayout::default(),
        }
    }
}

impl core::ops::Deref for VkSubresourceLayoutBuilder {
    type Target = VkSubresourceLayout;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkSubresourceLayoutBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkImageSubresourceBuilder {
    s: VkImageSubresource,
}

impl VkImageSubresourceBuilder {
    pub fn new() -> Self {
        Self {
            s: VkImageSubresource::default(),
        }
    }
}

impl core::ops::Deref for VkImageSubresourceBuilder {
    type Target = VkImageSubresource;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkImageSubresourceBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkImageCreateInfoBuilder {
    s: VkImageCreateInfo,
}

impl VkImageCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkImageCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkImageCreateInfoBuilder {
    type Target = VkImageCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkImageCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkBufferViewCreateInfoBuilder {
    s: VkBufferViewCreateInfo,
}

impl VkBufferViewCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkBufferViewCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkBufferViewCreateInfoBuilder {
    type Target = VkBufferViewCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkBufferViewCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkBufferCreateInfoBuilder {
    s: VkBufferCreateInfo,
}

impl VkBufferCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkBufferCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkBufferCreateInfoBuilder {
    type Target = VkBufferCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkBufferCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkQueryPoolCreateInfoBuilder {
    s: VkQueryPoolCreateInfo,
}

impl VkQueryPoolCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkQueryPoolCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkQueryPoolCreateInfoBuilder {
    type Target = VkQueryPoolCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkQueryPoolCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkEventCreateInfoBuilder {
    s: VkEventCreateInfo,
}

impl VkEventCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkEventCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkEventCreateInfoBuilder {
    type Target = VkEventCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkEventCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSemaphoreCreateInfoBuilder {
    s: VkSemaphoreCreateInfo,
}

impl VkSemaphoreCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkSemaphoreCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkSemaphoreCreateInfoBuilder {
    type Target = VkSemaphoreCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkSemaphoreCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkFenceCreateInfoBuilder {
    s: VkFenceCreateInfo,
}

impl VkFenceCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkFenceCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkFenceCreateInfoBuilder {
    type Target = VkFenceCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkFenceCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkBindSparseInfoBuilder {
    s: VkBindSparseInfo,
}

impl VkBindSparseInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkBindSparseInfo::default(),
        }
    }
}

impl core::ops::Deref for VkBindSparseInfoBuilder {
    type Target = VkBindSparseInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkBindSparseInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSparseImageMemoryBindInfoBuilder {
    s: VkSparseImageMemoryBindInfo,
}

impl VkSparseImageMemoryBindInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkSparseImageMemoryBindInfo::default(),
        }
    }
}

impl core::ops::Deref for VkSparseImageMemoryBindInfoBuilder {
    type Target = VkSparseImageMemoryBindInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkSparseImageMemoryBindInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSparseImageMemoryBindBuilder {
    s: VkSparseImageMemoryBind,
}

impl VkSparseImageMemoryBindBuilder {
    pub fn new() -> Self {
        Self {
            s: VkSparseImageMemoryBind::default(),
        }
    }
}

impl core::ops::Deref for VkSparseImageMemoryBindBuilder {
    type Target = VkSparseImageMemoryBind;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkSparseImageMemoryBindBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSparseImageOpaqueMemoryBindInfoBuilder {
    s: VkSparseImageOpaqueMemoryBindInfo,
}

impl VkSparseImageOpaqueMemoryBindInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkSparseImageOpaqueMemoryBindInfo::default(),
        }
    }
}

impl core::ops::Deref for VkSparseImageOpaqueMemoryBindInfoBuilder {
    type Target = VkSparseImageOpaqueMemoryBindInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkSparseImageOpaqueMemoryBindInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSparseMemoryBindBuilder {
    s: VkSparseMemoryBind,
}

impl VkSparseMemoryBindBuilder {
    pub fn new() -> Self {
        Self {
            s: VkSparseMemoryBind::default(),
        }
    }
}

impl core::ops::Deref for VkSparseMemoryBindBuilder {
    type Target = VkSparseMemoryBind;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkSparseMemoryBindBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSparseBufferMemoryBindInfoBuilder {
    s: VkSparseBufferMemoryBindInfo,
}

impl VkSparseBufferMemoryBindInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkSparseBufferMemoryBindInfo::default(),
        }
    }
}

impl core::ops::Deref for VkSparseBufferMemoryBindInfoBuilder {
    type Target = VkSparseBufferMemoryBindInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkSparseBufferMemoryBindInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSparseImageFormatPropertiesBuilder {
    s: VkSparseImageFormatProperties,
}

impl VkSparseImageFormatPropertiesBuilder {
    pub fn new() -> Self {
        Self {
            s: VkSparseImageFormatProperties::default(),
        }
    }
}

impl core::ops::Deref for VkSparseImageFormatPropertiesBuilder {
    type Target = VkSparseImageFormatProperties;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkSparseImageFormatPropertiesBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSparseImageMemoryRequirementsBuilder {
    s: VkSparseImageMemoryRequirements,
}

impl VkSparseImageMemoryRequirementsBuilder {
    pub fn new() -> Self {
        Self {
            s: VkSparseImageMemoryRequirements::default(),
        }
    }
}

impl core::ops::Deref for VkSparseImageMemoryRequirementsBuilder {
    type Target = VkSparseImageMemoryRequirements;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkSparseImageMemoryRequirementsBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkMemoryRequirementsBuilder {
    s: VkMemoryRequirements,
}

impl VkMemoryRequirementsBuilder {
    pub fn new() -> Self {
        Self {
            s: VkMemoryRequirements::default(),
        }
    }
}

impl core::ops::Deref for VkMemoryRequirementsBuilder {
    type Target = VkMemoryRequirements;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkMemoryRequirementsBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkMappedMemoryRangeBuilder {
    s: VkMappedMemoryRange,
}

impl VkMappedMemoryRangeBuilder {
    pub fn new() -> Self {
        Self {
            s: VkMappedMemoryRange::default(),
        }
    }
}

impl core::ops::Deref for VkMappedMemoryRangeBuilder {
    type Target = VkMappedMemoryRange;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkMappedMemoryRangeBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkMemoryAllocateInfoBuilder {
    s: VkMemoryAllocateInfo,
}

impl VkMemoryAllocateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkMemoryAllocateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkMemoryAllocateInfoBuilder {
    type Target = VkMemoryAllocateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkMemoryAllocateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSubmitInfoBuilder {
    s: VkSubmitInfo,
}

impl VkSubmitInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkSubmitInfo::default(),
        }
    }
}

impl core::ops::Deref for VkSubmitInfoBuilder {
    type Target = VkSubmitInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkSubmitInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkLayerPropertiesBuilder {
    s: VkLayerProperties,
}

impl VkLayerPropertiesBuilder {
    pub fn new() -> Self {
        Self {
            s: VkLayerProperties::default(),
        }
    }
}

impl core::ops::Deref for VkLayerPropertiesBuilder {
    type Target = VkLayerProperties;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkLayerPropertiesBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkExtensionPropertiesBuilder {
    s: VkExtensionProperties,
}

impl VkExtensionPropertiesBuilder {
    pub fn new() -> Self {
        Self {
            s: VkExtensionProperties::default(),
        }
    }
}

impl core::ops::Deref for VkExtensionPropertiesBuilder {
    type Target = VkExtensionProperties;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkExtensionPropertiesBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDeviceCreateInfoBuilder {
    s: VkDeviceCreateInfo,
}

impl VkDeviceCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkDeviceCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkDeviceCreateInfoBuilder {
    type Target = VkDeviceCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkDeviceCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPhysicalDeviceFeaturesBuilder {
    s: VkPhysicalDeviceFeatures,
}

impl VkPhysicalDeviceFeaturesBuilder {
    pub fn new() -> Self {
        Self {
            s: VkPhysicalDeviceFeatures::default(),
        }
    }
}

impl core::ops::Deref for VkPhysicalDeviceFeaturesBuilder {
    type Target = VkPhysicalDeviceFeatures;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkPhysicalDeviceFeaturesBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDeviceQueueCreateInfoBuilder {
    s: VkDeviceQueueCreateInfo,
}

impl VkDeviceQueueCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkDeviceQueueCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkDeviceQueueCreateInfoBuilder {
    type Target = VkDeviceQueueCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkDeviceQueueCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPhysicalDeviceMemoryPropertiesBuilder {
    s: VkPhysicalDeviceMemoryProperties,
}

impl VkPhysicalDeviceMemoryPropertiesBuilder {
    pub fn new() -> Self {
        Self {
            s: VkPhysicalDeviceMemoryProperties::default(),
        }
    }
}

impl core::ops::Deref for VkPhysicalDeviceMemoryPropertiesBuilder {
    type Target = VkPhysicalDeviceMemoryProperties;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkPhysicalDeviceMemoryPropertiesBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkMemoryHeapBuilder {
    s: VkMemoryHeap,
}

impl VkMemoryHeapBuilder {
    pub fn new() -> Self {
        Self {
            s: VkMemoryHeap::default(),
        }
    }
}

impl core::ops::Deref for VkMemoryHeapBuilder {
    type Target = VkMemoryHeap;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkMemoryHeapBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkMemoryTypeBuilder {
    s: VkMemoryType,
}

impl VkMemoryTypeBuilder {
    pub fn new() -> Self {
        Self {
            s: VkMemoryType::default(),
        }
    }
}

impl core::ops::Deref for VkMemoryTypeBuilder {
    type Target = VkMemoryType;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkMemoryTypeBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkQueueFamilyPropertiesBuilder {
    s: VkQueueFamilyProperties,
}

impl VkQueueFamilyPropertiesBuilder {
    pub fn new() -> Self {
        Self {
            s: VkQueueFamilyProperties::default(),
        }
    }
}

impl core::ops::Deref for VkQueueFamilyPropertiesBuilder {
    type Target = VkQueueFamilyProperties;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkQueueFamilyPropertiesBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPhysicalDevicePropertiesBuilder {
    s: VkPhysicalDeviceProperties,
}

impl VkPhysicalDevicePropertiesBuilder {
    pub fn new() -> Self {
        Self {
            s: VkPhysicalDeviceProperties::default(),
        }
    }
}

impl core::ops::Deref for VkPhysicalDevicePropertiesBuilder {
    type Target = VkPhysicalDeviceProperties;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkPhysicalDevicePropertiesBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPhysicalDeviceSparsePropertiesBuilder {
    s: VkPhysicalDeviceSparseProperties,
}

impl VkPhysicalDeviceSparsePropertiesBuilder {
    pub fn new() -> Self {
        Self {
            s: VkPhysicalDeviceSparseProperties::default(),
        }
    }
}

impl core::ops::Deref for VkPhysicalDeviceSparsePropertiesBuilder {
    type Target = VkPhysicalDeviceSparseProperties;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkPhysicalDeviceSparsePropertiesBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPhysicalDeviceLimitsBuilder {
    s: VkPhysicalDeviceLimits,
}

impl VkPhysicalDeviceLimitsBuilder {
    pub fn new() -> Self {
        Self {
            s: VkPhysicalDeviceLimits::default(),
        }
    }
}

impl core::ops::Deref for VkPhysicalDeviceLimitsBuilder {
    type Target = VkPhysicalDeviceLimits;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkPhysicalDeviceLimitsBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkImageFormatPropertiesBuilder {
    s: VkImageFormatProperties,
}

impl VkImageFormatPropertiesBuilder {
    pub fn new() -> Self {
        Self {
            s: VkImageFormatProperties::default(),
        }
    }
}

impl core::ops::Deref for VkImageFormatPropertiesBuilder {
    type Target = VkImageFormatProperties;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkImageFormatPropertiesBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkFormatPropertiesBuilder {
    s: VkFormatProperties,
}

impl VkFormatPropertiesBuilder {
    pub fn new() -> Self {
        Self {
            s: VkFormatProperties::default(),
        }
    }
}

impl core::ops::Deref for VkFormatPropertiesBuilder {
    type Target = VkFormatProperties;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkFormatPropertiesBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkInstanceCreateInfoBuilder {
    s: VkInstanceCreateInfo,
}

impl VkInstanceCreateInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkInstanceCreateInfo::default(),
        }
    }
}

impl core::ops::Deref for VkInstanceCreateInfoBuilder {
    type Target = VkInstanceCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkInstanceCreateInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkApplicationInfoBuilder {
    s: VkApplicationInfo,
}

impl VkApplicationInfoBuilder {
    pub fn new() -> Self {
        Self {
            s: VkApplicationInfo::default(),
        }
    }
}

impl core::ops::Deref for VkApplicationInfoBuilder {
    type Target = VkApplicationInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl core::ops::DerefMut for VkApplicationInfoBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

