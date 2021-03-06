//! The `application` module contains the application type and its implementations.

use raw::*;
use std::ffi;
use std::os::raw::c_void;
use ChunkWMError;

#[cfg(feature = "accessibility")]
use common::accessibility::application;

/// The `Application` struct.
#[derive(Debug)]
pub struct Application(ApplicationRef);

impl Application {
    /// Get the focused application.
    /// Needed features: `accessibility`.
    #[cfg(feature = "accessibility")]
    pub fn focused() -> Result<Application, ChunkWMError> {
        let application = unsafe { application::get_focused_application() };
        Ok(application.into())
    }

    /// Get all running processes.
    /// Needed features: `accessibility`.
    // TODO(splintah): ProcessFlags type?
    #[cfg(feature = "accessibility")]
    pub fn processes() -> Result<Vec<Application>, ChunkWMError> {
        let applications: &[ApplicationRef] = unsafe { application::get_running_processes(0) };
        let applications: Vec<ApplicationRef> = applications.to_vec();
        Ok(applications
            .iter()
            .map(|app_ref| Application::from(*app_ref))
            .collect())
    }

    /// Get the raw application pointer.
    pub unsafe fn application_ref(&self) -> Result<ApplicationRef, ChunkWMError> {
        if !self.0.is_null() {
            Ok(self.0)
        } else {
            Err(ChunkWMError::NullPointer)
        }
    }

    /// Destroy the application.
    /// Needed features: `accessibility`.
    #[cfg(feature = "accessibility")]
    pub fn destroy(&self) -> Result<(), ChunkWMError> {
        unsafe { application::destroy_application(self.application_ref()?) };
        Ok(())
    }

    /// Get the application's element.
    pub fn element(&self) -> Result<AXUIElementRef, ChunkWMError> {
        unsafe { Ok((*self.application_ref()?).element) }
    }

    /// Get the application's observer.
    pub fn observer(&self) -> Result<RawObserver, ChunkWMError> {
        unsafe { Ok((*self.application_ref()?).observer) }
    }

    /// Get the application's name.
    pub fn name(&self) -> Result<String, ChunkWMError> {
        unsafe {
            Ok(ffi::CStr::from_ptr((*self.application_ref()?).name)
                .to_string_lossy()
                .into_owned())
        }
    }

    /// Get the application's pid.
    pub fn pid(&self) -> Result<i32, ChunkWMError> {
        unsafe { Ok((*self.application_ref()?).pid) }
    }

    /// Get the application's process serial number.
    pub fn process_serial_number(&self) -> Result<ProcessSerialNumber, ChunkWMError> {
        unsafe { Ok((*self.application_ref()?).process_serial_number) }
    }
}

impl From<ApplicationRef> for Application {
    fn from(application_ref: ApplicationRef) -> Application {
        Application(application_ref)
    }
}

impl<'a> From<&'a mut RawApplication> for Application {
    fn from(raw_app: &mut RawApplication) -> Application {
        Application(raw_app)
    }
}

impl Into<Application> for RawApplication {
    fn into(mut self) -> Application {
        Application(&mut self)
    }
}

impl From<*mut c_void> for Application {
    fn from(application_ref: *mut c_void) -> Application {
        Application(application_ref as ApplicationRef)
    }
}
