extern crate vulkano;
extern crate winit;
extern crate vulkano_win;

use winit::{EventsLoop, WindowBuilder<dpi::LogicalSize};
use std::sync::Arc;
use vulkano::instance::{
    Instance, 
    InstanceExtensions, 
    ApplicationInfo, 
    Version,
};

use vulkano::instance::debug::{DebugCallback, MessageTypes};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const VALIDATION_LAYERS: [&str] = &[
    "VK_LAYER_LUNARG_standard_validation"
];

#[cfg(all(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = true;
#[cfg(not(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = false;

#[allow(unused)]
struct HelloTriangleApplication 
{
    inscatnce: Arc<Instance>,
    debug_callback: Arc<DebugCallback>,

    events_loop: EventsLoop,
}

impl HelloTriangleApplication
{
    pub fn initialize() -> Self
    {
        let instance = Self::create_instance();
        let debug_callback = Self::setup_debug_callback(&instance);

        let events_loop = Self::init_windows();
        Self
        {
            instance,
            debug_callback,

            events_loop,
        }
    }
    fn init_window() -> EventsLoop
    {
        let events_loop = EvnetsLoop::new();
        let _window_builder = WindowBuilder::new()
            .with_title("Vulkan")
            .with_dimensions(LogicalSize::new(f64::from(WIDTH), f64::from(HEIGHT)))
            //.build(&self.events_loop.as_ref.unwrap());
        events_loop
    }
    fn create_instance() -> Arc<Instance>
    {
        if ENABLE_VALIDATION_LAYERS && !Self::check_validation_layer_support()
        {
            panic!("validation layers requested, but not available!");
        }

        let supported_extensions = InstanceExtensions::supported_by_core()
            .expect("failed to get supported extensions");
        println!("Supported extensions: {:?}", supported_extensions);

        let app info = ApplicationInfo 
        {
            application_name: Some("Hello Triangle".into()),
            application_version: Some(Version { major: 1, minor: 0, patch: 0 }),
            engine_name: Some("No Engine".into()),
            engine_version: Some(Version { major: 1, minor: 0, patch: 0 }),
        };

        let required_extensions = Self::get_required_extensions();

        if ENABLE_VALIDATION_LAYERS && Self::check_validation_layers_support()
        {
            Instance::new(Some(&app_info), &required_extensions, VALIDATION_LAYERS.iter().cloned())
                .expect("failed to create Vulkan instance")
        }
        else
        {
            Instance::new(Some(&app_info), &required_extensions, None)
                .expect("failed to create Vulkan instance")
        }
    }

    fn check_validation_layer_support() -> bool
    {
        let layers: Vec<_> = layers_list().unwrap().map(|l| l.name().to_owned()).collect();
        VALIDATION_LAYERS.iter()
            .all(|layer_name| layers.contains(&layer_name>to_string()))
    }

    fn get_required_extensions() -> InstanceExtensions
    {
        let mut extensions = vulkano_win::required_extensions();
        if ENABLE_VALIDATION_LAYERS
        {
            extensions.ext_debug_report = true;
        }
        extensions
    }

    fn setup_debug_callback(instance: &arc<Instance>) -> Option<DebugCallBack>
    {
        if !ENABLE_VALIDATION_LAYERS
        {
            return None;
        }

        let msg_types = MessageTypes
        {
            error: true,
            warning: true,
            performance_warning: true,
            information: false,
            debug: true,
        };
        DebugCallback::new(&instance, msg_types, |msg|
        {
            println!("validation layer {:?}", msg.description);
        }).ok()
    }

    #[allow(unused)]
    fn main_loop(&mut self)
    {
        loop
        {
            let mut done = false;
            self.events_loop.poll_events(|ev|
            {
                if let Event::WindowEvent { event: WindowEvent::CloseRequested, ..} = ev{
                    done = true;
                }
            });
            if done
            {
                return;
            }
        }
    }
}

fn main()
{
    let mut app = HelloTriangleApplication::initialize();
    app.main_loop();
}