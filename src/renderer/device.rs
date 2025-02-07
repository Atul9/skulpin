
use ash::vk;
use ash::prelude::VkResult;
use super::VkInstance;
use super::window_support;

use ash::version::DeviceV1_0;
use ash::version::InstanceV1_0;

use std::ffi::CStr;

//use ash::extensions::ext as ash_ext;
use ash::extensions::khr;

/// Has the indexes for all the queue families we will need. It's possible a single family
/// is used for both graphics and presentation, in which case the index will be the same
#[derive(Default)]
pub struct QueueFamilyIndices {
    pub graphics_queue_family_index: u32,
    pub present_queue_family_index: u32
}

/// An instantiated queue per queue family. We only need one queue per family.
pub struct Queues {
    pub graphics_queue: ash::vk::Queue,
    pub present_queue: ash::vk::Queue
}

pub struct VkDevice {
    pub surface: ash::vk::SurfaceKHR,
    pub surface_loader: ash::extensions::khr::Surface,
    pub physical_device: ash::vk::PhysicalDevice,
    pub logical_device: ash::Device,
    pub queue_family_indices: QueueFamilyIndices,
    pub queues: Queues,
    pub memory_properties: vk::PhysicalDeviceMemoryProperties
}

impl VkDevice {
    pub fn new(instance: &VkInstance, window: &winit::window::Window) -> VkResult<Self> {
        // Get the surface, needed to select the best queue family
        use raw_window_handle::HasRawWindowHandle;
        let surface = unsafe {
            window_support::create_surface(
                &instance.entry,
                &instance.instance,
                &window.raw_window_handle()
            )?
        };

        let surface_loader = khr::Surface::new(
            &instance.entry,
            &instance.instance);

        // Pick a physical device
        let (
            physical_device,
            queue_family_indices
        ) = Self::choose_physical_device(&instance.instance, &surface_loader, &surface)?;

        // Create a logical device
        let (
            logical_device,
            queues
        ) = Self::create_logical_device(
            &instance.instance,
            &physical_device,
            &queue_family_indices
        )?;

        let memory_properties = unsafe {
            instance.instance.get_physical_device_memory_properties(physical_device)
        };

        Ok(VkDevice {
            surface,
            surface_loader,
            physical_device,
            logical_device,
            queue_family_indices,
            queues,
            memory_properties
        })
    }

    fn choose_physical_device(
        instance: &ash::Instance,
        surface_loader: &ash::extensions::khr::Surface,
        surface: &ash::vk::SurfaceKHR
    ) -> VkResult<(ash::vk::PhysicalDevice, QueueFamilyIndices)> {
        let physical_devices = unsafe {
            instance
                .enumerate_physical_devices()?
        };

        if physical_devices.len() <= 0 {
            panic!("Could not find a physical device");
        }

        let mut best_physical_device = None;
        let mut best_physical_device_score = -1;
        let mut best_physical_device_queue_family_indices = None;
        for physical_device in physical_devices {
            if let Some((score, queue_family_indices)) = Self::get_score_and_queue_families_for_physical_device(instance, &physical_device, surface_loader, surface)? {
                if score > best_physical_device_score {
                    best_physical_device = Some(physical_device);
                    best_physical_device_score = score;
                    best_physical_device_queue_family_indices = Some(queue_family_indices);
                }
            }
        }

        //TODO: Return an error
        let physical_device = best_physical_device.expect("Could not find suitable device");
        let queue_family_indices = best_physical_device_queue_family_indices.unwrap();

        Ok((physical_device, queue_family_indices))
    }

    fn get_score_and_queue_families_for_physical_device(
        instance: &ash::Instance,
        device: &ash::vk::PhysicalDevice,
        surface_loader: &ash::extensions::khr::Surface,
        surface: &ash::vk::SurfaceKHR
    ) -> VkResult<Option<(i32, QueueFamilyIndices)>> {
        let properties : ash::vk::PhysicalDeviceProperties = unsafe { instance.get_physical_device_properties(*device) };
        let device_name = unsafe {CStr::from_ptr(properties.device_name.as_ptr()).to_str().unwrap().to_string() };

        //TODO: Check that the extensions we want to use are supported
        let _extensions : Vec<ash::vk::ExtensionProperties> = unsafe { instance.enumerate_device_extension_properties(*device)? };
        let features : vk::PhysicalDeviceFeatures = unsafe { instance.get_physical_device_features(*device) };

        if features.sampler_anisotropy == vk::FALSE {
            info!("Found unsuitable device '{}', does not support sampler_anisotropy", device_name);
            return Ok(None);
        }

        let queue_family_indices = Self::find_queue_families(instance, device, surface_loader, surface);
        if let Some(queue_family_indices) = queue_family_indices {
            // Query info about the GPU
            //let features : ash::vk::PhysicalDeviceFeatures = unsafe { instance.get_physical_device_features(*device) };

            let mut score = 0;

            // What kind of GPU is it?
            score += if properties.device_type == ash::vk::PhysicalDeviceType::DISCRETE_GPU {
                1000
            } else if properties.device_type == ash::vk::PhysicalDeviceType::VIRTUAL_GPU {
                500
            } else if properties.device_type == ash::vk::PhysicalDeviceType::INTEGRATED_GPU {
                100
            } else {
                0
            };

            info!("Found suitable device '{}' score = {}", device_name, score);
            trace!("{:#?}", properties);
            Ok(Some((score, queue_family_indices)))
        } else {
            info!("Found unsuitable device '{}', could not find queue families", device_name);
            trace!("{:#?}", properties);
            Ok(None)
        }
    }

    fn find_queue_families(
        instance: &ash::Instance,
        physical_device: &ash::vk::PhysicalDevice,
        surface_loader: &ash::extensions::khr::Surface,
        surface: &ash::vk::SurfaceKHR
    ) -> Option<QueueFamilyIndices> {
        let queue_families : Vec<ash::vk::QueueFamilyProperties> = unsafe { instance.get_physical_device_queue_family_properties(*physical_device) };

        let mut graphics_queue_family_index = None;
        let mut present_queue_family_index = None;
        for (queue_family_index, queue_family) in queue_families.iter().enumerate() {
            let queue_family_index = queue_family_index as u32;
            if queue_family.queue_flags & ash::vk::QueueFlags::GRAPHICS == ash::vk::QueueFlags::GRAPHICS {
                graphics_queue_family_index = Some(queue_family_index);
            }

            if unsafe {surface_loader.get_physical_device_surface_support(*physical_device, queue_family_index, *surface) } {
                present_queue_family_index = Some(queue_family_index);
            }
        }

        Some(QueueFamilyIndices {
            graphics_queue_family_index: graphics_queue_family_index?,
            present_queue_family_index: present_queue_family_index?
        })
    }

    fn create_logical_device(
        instance: &ash::Instance,
        physical_device: &ash::vk::PhysicalDevice,
        queue_family_indices: &QueueFamilyIndices
    )
        -> VkResult<(ash::Device, Queues)>
    {
        //TODO: Ideally we would set up validation layers for the logical device too.

        let device_extension_names_raw = [khr::Swapchain::name().as_ptr()];
        let features = vk::PhysicalDeviceFeatures::builder()
            .sampler_anisotropy(true);
        let priorities = [1.0];

        let mut queue_families_to_create = std::collections::HashSet::new();
        queue_families_to_create.insert(queue_family_indices.graphics_queue_family_index);
        queue_families_to_create.insert(queue_family_indices.present_queue_family_index);

        let queue_infos : Vec<_> = queue_families_to_create.iter().map(|queue_family_index| {
            vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(*queue_family_index)
                .queue_priorities(&priorities)
                .build()
        }).collect();

        let device_create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_infos)
            .enabled_extension_names(&device_extension_names_raw)
            .enabled_features(&features);

        let device : ash::Device = unsafe {
            instance
                .create_device(*physical_device, &device_create_info, None)?
        };

        let graphics_queue = unsafe {
            device.get_device_queue(queue_family_indices.graphics_queue_family_index, 0)
        };

        let present_queue = unsafe {
            device.get_device_queue(queue_family_indices.present_queue_family_index, 0)
        };

        let queues = Queues {
            graphics_queue,
            present_queue
        };

        Ok((device, queues))
    }
}

impl Drop for VkDevice {
    fn drop(&mut self) {
        info!("destroying VkDevice");
        unsafe {
            self.logical_device.destroy_device(None);
            self.surface_loader.destroy_surface(self.surface, None);
        }

        info!("destroyed VkDevice");
    }
}