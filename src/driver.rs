use crate::info;
use crate::synchronization::{interface::Mutex, NullLock};

const NUM_DRIVERS: usize = 5;

struct DriverManagerInner {
    next_index: usize,
    descriptors: [Option<DeviceDriverDescriptor>; NUM_DRIVERS],
}

pub mod interface {
    pub trait DeviceDriver {
        fn compatible(&self) -> &'static str;
        unsafe fn init(&self) -> Result<(), &'static str> {
            Ok(())
        }
    }
}

pub type DeviceDriverPostInitCallback = unsafe fn() -> Result<(), &'static str>;

#[derive(Copy, Clone)]
pub struct DeviceDriverDescriptor {
    device_driver: &'static (dyn interface::DeviceDriver + Sync),
    post_init_callback: Option<DeviceDriverPostInitCallback>,
}

pub struct DriverManager {
    inner: NullLock<DriverManagerInner>,
}

static DRIVER_MANAGER: DriverManager = DriverManager::new();

impl DriverManagerInner {
    pub const fn new() -> Self {
        Self {
            next_index: 0,
            descriptors: [None; NUM_DRIVERS],
        }
    }
}

impl DeviceDriverDescriptor {
    pub fn new(
        device_driver: &'static (dyn interface::DeviceDriver + Sync),
        post_init_callback: Option<DeviceDriverPostInitCallback>,
    ) -> Self {
        Self {
            device_driver,
            post_init_callback,
        }
    }
}

pub fn driver_manager() -> &'static DriverManager {
    &DRIVER_MANAGER
}

impl DriverManager {
    pub const fn new() -> Self {
        Self {
            inner: NullLock::new(DriverManagerInner::new())
        }
    }
    
    pub fn register_driver(&self, descriptor: DeviceDriverDescriptor) {
        self.inner.lock(|inner| {
            inner.descriptors[inner.next_index] = Some(descriptor);
            inner.next_index += 1;
        });
    }
    
    fn for_each_descriptor<'a>(&'a self, f: impl FnMut(&'a DeviceDriverDescriptor)) {
        self.inner.lock(|inner| {
            inner
                .descriptors
                .iter()
                .filter_map(|d| d.as_ref())
                .for_each(f)
        })
    }
    
    pub unsafe fn init_drivers(&self) {
        self.for_each_descriptor(|d| {
            if let Err(e) = d.device_driver.init() {
                panic!(
                    "Error initializing driver: {}: {}",
                    d.device_driver.compatible(),
                    e,
                )
            }
            if let Some(callback) = &d.post_init_callback {
                if let Err(e) = callback() {
                    panic!(
                        "Error during driver post-init callback: {}: {}",
                        d.device_driver.compatible(),
                        e,
                    )
                }
            }
        });
    }
    
    pub fn enumerate(&self) {
        let mut i: usize = 1;
        self.for_each_descriptor(|d| {
            info!("    {}. {}", i, d.device_driver.compatible());
            i += 1;
        });
    }
}
