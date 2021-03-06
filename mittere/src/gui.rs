#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    dpi::LogicalSize,
    window::WindowBuilder,
};
use crate::VERSION;
use vulkano::instance::{Instance, InstanceExtensions, ApplicationInfo, Version, PhysicalDevice};
use vulkano_win::VkSurfaceBuild;
use vulkano::device::{DeviceExtensions, Device};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub fn window_handler() {
    // required extensions
    let required_extensions = vulkano_win::required_extensions();

    // application info
    let split: Vec<&str> = VERSION.split(".").collect();

    let major = split.get(0).to_string().parse::<u16>().expect("Failed to get Major version number.");
    let minor = split.get(1).to_string().parse::<u16>().expect("Failed to get Minor version number.");
    let patch = split.get(2).to_string().parse::<u16>().expect("Failed to get Patch version number.");

    let app_info = ApplicationInfo {
        application_name: Some("Mittere".into()),
        application_version: Some(Version {major, minor, patch}),
        engine_name: Some("No engine".into()),
        engine_version: Some(Version { major: 0, minor: 0, patch: 0}),
    };

    // create an instance
    let instance = Instance::new(Some(&app_info), &required_extensions, None).unwrap();

    // a physical device to use
    let physical = PhysicalDevice::enumerate(&instance).next().unwrap();

    // debug info! TODO: Remove
    println!(
        "Using device: {} (type: {:?})",
        physical.name(),
        physical.ty()
    );

    // create the window
    let event_loop = EventLoop::new();
    let surface = WindowBuilder::new()
        .build_vk_surface(&event_loop, instance.clone())
        .expect("Failed to build the vulkan surface");

    // determine which GPU queue will execute our draw commands
    let queue_family = physical
        .queue_families()
        .find(|&q| {
            q.supports_graphics() && surface.is_supported(q).unwrap_or(false)
        })
        .unwrap();

    // initialize the device
    let device_ext = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::none()
    };
    let (device, mut queues) = Device::new(

    ).unwrap();
}