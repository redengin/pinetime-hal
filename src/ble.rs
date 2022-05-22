
use rubble::{
    att::{AttUuid, Attribute, AttributeAccessPermissions, AttributeProvider, Handle, HandleRange},
    uuid::{Uuid16},
    Error,
};

const PRIMARY_SERVICE_UUID16: Uuid16 = Uuid16(0x2800);
const CHARACTERISTIC_UUID16: Uuid16 = Uuid16(0x2803);
// const GENERIC_ATTRIBUTE_UUID16: Uuid16 = Uuid16(0x1801);
// const BATTERY_LEVEL_UUID16: Uuid16 = Uuid16(0x2A19);

pub struct BleAttributes {
    static_attributes: [Attribute<&'static [u8]>; 6],
}

impl AttributeProvider for BleAttributes {
    fn attr_access_permissions(&self, handle: Handle) -> AttributeAccessPermissions {
        match handle.as_u16() {
            0x0003 => AttributeAccessPermissions::ReadableAndWriteable,
            _ => AttributeAccessPermissions::Readable,
        }
    }

    /// Attempts to write data to the attribute with the given handle.
    /// If any of your attributes are writeable, this function must be implemented.
    // fn write_attr(&mut self, handle: Handle, data: &[u8]) -> Result<(), Error> {
    //     match handle.as_u16() {
    //         0x0003 => {
    //             if data.is_empty() {
    //                 return Err(Error::InvalidLength);
    //             }
    //             // If we receive a 1, activate the LED; otherwise deactivate it
    //             // Assumes LED is active low
    //             if data[0] == 1 {
    //                 self.led_pin.set_low().unwrap();
    //             } else {
    //                 self.led_pin.set_high().unwrap();
    //             }
    //             // Copy written value into buffer to display back for reading
    //             self.led_buf.copy_from_slice(data);
    //             Ok(())
    //         }
    //         _ => panic!("Attempted to write an unwriteable attribute"),
    //     }
    // }

    fn is_grouping_attr(&self, uuid: AttUuid) -> bool {
        uuid == PRIMARY_SERVICE_UUID16 || uuid == CHARACTERISTIC_UUID16
    }

    fn group_end(&self, handle: Handle) -> Option<&Attribute<dyn AsRef<[u8]>>> {
        match handle.as_u16() {
            // Handles for the LED primary service and characteristic
            // The group end is a dummy attribute; strictly speaking it's not required
            // but we can't use the lazily generated attribute because this funtion requires
            // returning a reference
            0x0001 | 0x0002 => Some(&self.static_attributes[2]),
            // Handles for Battery Service
            0x0005 | 0x0006 => Some(&self.static_attributes[5]),
            _ => None,
        }
    }

    /// Applies a function to all attributes with handles within the specified range
    fn for_attrs_in_range(
        &mut self,
        range: HandleRange,
        mut f: impl FnMut(&Self, &Attribute<dyn AsRef<[u8]>>) -> Result<(), Error>,
    ) -> Result<(), Error> {
        // Handles start at 1, not 0, but we're not directly indexing
        let start = range.start().as_u16();
        let end = range.end().as_u16();
        let range_u16 = start..=end;
        // Can't just iterate from start to end because of the presence of lazy attributes
        // Ranges are empty if start >= end
        for attr in &self.static_attributes {
            if range_u16.contains(&attr.handle.as_u16()) {
                f(self, attr)?;
            }
        }
        // Check lazy attributes
        // Note that with this implementation, if a static attribute has handle greater than a
        // lazy attribute, the order in which f() is applied is not preserved.
        // This may matter for the purposes of short-circuiting an operation if it cannot be applied
        // to a particular attribute
        // if range_u16.contains(&0x0003) {
        //     f(self, &self.led_data_attr())?;
        // };
        Ok(())
    }
}