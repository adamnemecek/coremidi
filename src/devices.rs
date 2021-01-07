use Device;
use Object;

use std::ops::Deref;

impl Deref for Device {
    type Target = Object;

    fn deref(&self) -> &Object {
        &self.object
    }
}
