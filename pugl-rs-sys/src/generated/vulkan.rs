use crate::*;
/* automatically generated by rust-bindgen 0.71.1 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkInstance_T {
    _unused: [u8; 0],
}
pub type VkInstance = *mut VkInstance_T;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDevice_T {
    _unused: [u8; 0],
}
pub type VkDevice = *mut VkDevice_T;
pub const VK_SUCCESS: VkResult = 0;
pub const VK_NOT_READY: VkResult = 1;
pub const VK_TIMEOUT: VkResult = 2;
pub const VK_EVENT_SET: VkResult = 3;
pub const VK_EVENT_RESET: VkResult = 4;
pub const VK_INCOMPLETE: VkResult = 5;
pub const VK_ERROR_OUT_OF_HOST_MEMORY: VkResult = -1;
pub const VK_ERROR_OUT_OF_DEVICE_MEMORY: VkResult = -2;
pub const VK_ERROR_INITIALIZATION_FAILED: VkResult = -3;
pub const VK_ERROR_DEVICE_LOST: VkResult = -4;
pub const VK_ERROR_MEMORY_MAP_FAILED: VkResult = -5;
pub const VK_ERROR_LAYER_NOT_PRESENT: VkResult = -6;
pub const VK_ERROR_EXTENSION_NOT_PRESENT: VkResult = -7;
pub const VK_ERROR_FEATURE_NOT_PRESENT: VkResult = -8;
pub const VK_ERROR_INCOMPATIBLE_DRIVER: VkResult = -9;
pub const VK_ERROR_TOO_MANY_OBJECTS: VkResult = -10;
pub const VK_ERROR_FORMAT_NOT_SUPPORTED: VkResult = -11;
pub const VK_ERROR_FRAGMENTED_POOL: VkResult = -12;
pub const VK_ERROR_UNKNOWN: VkResult = -13;
pub const VK_ERROR_OUT_OF_POOL_MEMORY: VkResult = -1000069000;
pub const VK_ERROR_INVALID_EXTERNAL_HANDLE: VkResult = -1000072003;
pub const VK_ERROR_FRAGMENTATION: VkResult = -1000161000;
pub const VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS: VkResult = -1000257000;
pub const VK_PIPELINE_COMPILE_REQUIRED: VkResult = 1000297000;
pub const VK_ERROR_NOT_PERMITTED: VkResult = -1000174001;
pub const VK_ERROR_SURFACE_LOST_KHR: VkResult = -1000000000;
pub const VK_ERROR_NATIVE_WINDOW_IN_USE_KHR: VkResult = -1000000001;
pub const VK_SUBOPTIMAL_KHR: VkResult = 1000001003;
pub const VK_ERROR_OUT_OF_DATE_KHR: VkResult = -1000001004;
pub const VK_ERROR_INCOMPATIBLE_DISPLAY_KHR: VkResult = -1000003001;
pub const VK_ERROR_VALIDATION_FAILED_EXT: VkResult = -1000011001;
pub const VK_ERROR_INVALID_SHADER_NV: VkResult = -1000012000;
pub const VK_ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR: VkResult = -1000023000;
pub const VK_ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR: VkResult = -1000023001;
pub const VK_ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR: VkResult = -1000023002;
pub const VK_ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR: VkResult = -1000023003;
pub const VK_ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR: VkResult = -1000023004;
pub const VK_ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR: VkResult = -1000023005;
pub const VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT: VkResult = -1000158000;
pub const VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT: VkResult = -1000255000;
pub const VK_THREAD_IDLE_KHR: VkResult = 1000268000;
pub const VK_THREAD_DONE_KHR: VkResult = 1000268001;
pub const VK_OPERATION_DEFERRED_KHR: VkResult = 1000268002;
pub const VK_OPERATION_NOT_DEFERRED_KHR: VkResult = 1000268003;
pub const VK_ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR: VkResult = -1000299000;
pub const VK_ERROR_COMPRESSION_EXHAUSTED_EXT: VkResult = -1000338000;
pub const VK_INCOMPATIBLE_SHADER_BINARY_EXT: VkResult = 1000482000;
pub const VK_PIPELINE_BINARY_MISSING_KHR: VkResult = 1000483000;
pub const VK_ERROR_NOT_ENOUGH_SPACE_KHR: VkResult = -1000483000;
pub const VK_ERROR_OUT_OF_POOL_MEMORY_KHR: VkResult = -1000069000;
pub const VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR: VkResult = -1000072003;
pub const VK_ERROR_FRAGMENTATION_EXT: VkResult = -1000161000;
pub const VK_ERROR_NOT_PERMITTED_EXT: VkResult = -1000174001;
pub const VK_ERROR_NOT_PERMITTED_KHR: VkResult = -1000174001;
pub const VK_ERROR_INVALID_DEVICE_ADDRESS_EXT: VkResult = -1000257000;
pub const VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR: VkResult = -1000257000;
pub const VK_PIPELINE_COMPILE_REQUIRED_EXT: VkResult = 1000297000;
pub const VK_ERROR_PIPELINE_COMPILE_REQUIRED_EXT: VkResult = 1000297000;
pub const VK_ERROR_INCOMPATIBLE_SHADER_BINARY_EXT: VkResult = 1000482000;
pub const VK_RESULT_MAX_ENUM: VkResult = 2147483647;
pub type VkResult = ::std::os::raw::c_int;
pub const VK_SYSTEM_ALLOCATION_SCOPE_COMMAND: VkSystemAllocationScope = 0;
pub const VK_SYSTEM_ALLOCATION_SCOPE_OBJECT: VkSystemAllocationScope = 1;
pub const VK_SYSTEM_ALLOCATION_SCOPE_CACHE: VkSystemAllocationScope = 2;
pub const VK_SYSTEM_ALLOCATION_SCOPE_DEVICE: VkSystemAllocationScope = 3;
pub const VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE: VkSystemAllocationScope = 4;
pub const VK_SYSTEM_ALLOCATION_SCOPE_MAX_ENUM: VkSystemAllocationScope = 2147483647;
pub type VkSystemAllocationScope = ::std::os::raw::c_uint;
pub const VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE: VkInternalAllocationType = 0;
pub const VK_INTERNAL_ALLOCATION_TYPE_MAX_ENUM: VkInternalAllocationType = 2147483647;
pub type VkInternalAllocationType = ::std::os::raw::c_uint;
pub type PFN_vkAllocationFunction = ::std::option::Option<
    unsafe extern "C" fn(
        pUserData: *mut ::std::os::raw::c_void,
        size: usize,
        alignment: usize,
        allocationScope: VkSystemAllocationScope,
    ) -> *mut ::std::os::raw::c_void,
>;
pub type PFN_vkFreeFunction = ::std::option::Option<
    unsafe extern "C" fn(
        pUserData: *mut ::std::os::raw::c_void,
        pMemory: *mut ::std::os::raw::c_void,
    ),
>;
pub type PFN_vkInternalAllocationNotification = ::std::option::Option<
    unsafe extern "C" fn(
        pUserData: *mut ::std::os::raw::c_void,
        size: usize,
        allocationType: VkInternalAllocationType,
        allocationScope: VkSystemAllocationScope,
    ),
>;
pub type PFN_vkInternalFreeNotification = ::std::option::Option<
    unsafe extern "C" fn(
        pUserData: *mut ::std::os::raw::c_void,
        size: usize,
        allocationType: VkInternalAllocationType,
        allocationScope: VkSystemAllocationScope,
    ),
>;
pub type PFN_vkReallocationFunction = ::std::option::Option<
    unsafe extern "C" fn(
        pUserData: *mut ::std::os::raw::c_void,
        pOriginal: *mut ::std::os::raw::c_void,
        size: usize,
        alignment: usize,
        allocationScope: VkSystemAllocationScope,
    ) -> *mut ::std::os::raw::c_void,
>;
pub type PFN_vkVoidFunction = ::std::option::Option<unsafe extern "C" fn()>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkAllocationCallbacks {
    pub pUserData: *mut ::std::os::raw::c_void,
    pub pfnAllocation: PFN_vkAllocationFunction,
    pub pfnReallocation: PFN_vkReallocationFunction,
    pub pfnFree: PFN_vkFreeFunction,
    pub pfnInternalAllocation: PFN_vkInternalAllocationNotification,
    pub pfnInternalFree: PFN_vkInternalFreeNotification,
}
pub type PFN_vkGetInstanceProcAddr = ::std::option::Option<
    unsafe extern "C" fn(
        instance: VkInstance,
        pName: *const ::std::os::raw::c_char,
    ) -> PFN_vkVoidFunction,
>;
pub type PFN_vkGetDeviceProcAddr = ::std::option::Option<
    unsafe extern "C" fn(
        device: VkDevice,
        pName: *const ::std::os::raw::c_char,
    ) -> PFN_vkVoidFunction,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSurfaceKHR_T {
    _unused: [u8; 0],
}
pub type VkSurfaceKHR = *mut VkSurfaceKHR_T;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PuglVulkanLoaderImpl {
    _unused: [u8; 0],
}
#[doc = "Dynamic Vulkan loader.\n\nThis can be used to dynamically load the Vulkan library.  Applications or\nplugins should not link against the Vulkan library, but instead use this at\nruntime.  This ensures that things will work on as many systems as possible,\nand allows errors to be handled gracefully.\n\nThis is not a \"loader\" in the sense of loading all the required Vulkan\nfunctions (which is the application's responsibility), but just a minimal\nimplementation to portably load the Vulkan library and get the two functions\nthat are used to load everything else.\n\nNote that this owns the loaded Vulkan library, so it must outlive all use of\nthe Vulkan API.\n\n@see https://www.khronos.org/registry/vulkan/specs/1.0/html/chap4.html"]
pub type PuglVulkanLoader = PuglVulkanLoaderImpl;
unsafe extern "C" {
    #[doc = "Create a new dynamic loader for Vulkan functions.\n\nThis dynamically loads the Vulkan library and gets the load functions from\nit.\n\n@param world The world the returned loader is a part of.\n\n@param libraryName The name of the Vulkan library to load, or null.\nTypically, this is left unset, which will load the standard Vulkan library\nfor the current platform.  It can be set to an alternative name, or an\nabsolute path, to support special packaging scenarios or unusual system\nconfigurations.  This name is passed directly to the underlying platform\nlibrary loading function (`dlopen` or `LoadLibrary`).\n\n@return A new Vulkan loader, or null on failure."]
    pub fn puglNewVulkanLoader(
        world: *mut PuglWorld,
        libraryName: *const ::std::os::raw::c_char,
    ) -> *mut PuglVulkanLoader;
}
unsafe extern "C" {
    #[doc = "Free a loader created with puglNewVulkanLoader().\n\nNote that this closes the Vulkan library, so no Vulkan objects or API may be\nused after this is called."]
    pub fn puglFreeVulkanLoader(loader: *mut PuglVulkanLoader);
}
unsafe extern "C" {
    #[doc = "Return the `vkGetInstanceProcAddr` function.\n\n@return Null if the Vulkan library does not contain this function (which is\nunlikely and indicates a broken system)."]
    pub fn puglGetInstanceProcAddrFunc(
        loader: *const PuglVulkanLoader,
    ) -> PFN_vkGetInstanceProcAddr;
}
unsafe extern "C" {
    #[doc = "Return the `vkGetDeviceProcAddr` function.\n\n@return Null if the Vulkan library does not contain this function (which is\nunlikely and indicates a broken system)."]
    pub fn puglGetDeviceProcAddrFunc(loader: *const PuglVulkanLoader) -> PFN_vkGetDeviceProcAddr;
}
unsafe extern "C" {
    #[doc = "Return the Vulkan instance extensions required to draw to a PuglView.\n\nThis simply returns static strings, it does not access Vulkan or the window\nsystem.  The returned array always contains at least \"VK_KHR_surface\".\n\n@param[out] count The number of extensions in the returned array.\n@return An array of extension name strings."]
    pub fn puglGetInstanceExtensions(count: *mut u32) -> *const *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    #[doc = "Create a Vulkan surface for a Pugl view.\n\n@param vkGetInstanceProcAddr Accessor for Vulkan functions.\n@param view The view the surface is to be displayed on.\n@param instance The Vulkan instance.\n@param allocator Vulkan allocation callbacks, may be NULL.\n@param[out] surface Pointed to a newly created Vulkan surface.\n@return `VK_SUCCESS` on success, or a Vulkan error code."]
    pub fn puglCreateSurface(
        vkGetInstanceProcAddr: PFN_vkGetInstanceProcAddr,
        view: *mut PuglView,
        instance: VkInstance,
        allocator: *const VkAllocationCallbacks,
        surface: *mut VkSurfaceKHR,
    ) -> VkResult;
}
unsafe extern "C" {
    #[doc = "Vulkan graphics backend.\n\nPass the returned value to puglSetBackend() to draw to a view with Vulkan."]
    pub fn puglVulkanBackend() -> *const PuglBackend;
}
