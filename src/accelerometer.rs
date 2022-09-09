// Bosch BMA421 and BMA425
// created per https://github.com/InfiniTimeOrg/InfiniTime/tree/develop/src/drivers
// https://datasheet.lcsc.com/lcsc/1912111437_Bosch-Sensortec-BMA425_C437656.pdf
// https://files.pine64.org/doc/datasheet/pinetime/BST-BMA421-FL000.pdf


use embedded_hal as hal;

pub struct BMA4xx<I2C, InterruptPin> {
    i2c: I2C,
    address: Address,
    interrupt_pin1: Option<InterruptPin>,
    interrupt_pin2: Option<InterruptPin>
}

// impl accelerometer::RawAccelerometer<accelerometer::vector::I16x3> for BMA4xx {
//     type Error = ();

//     fn accel_raw(&mut self) -> Result<accelerometer::vector::I16x3, accelerometer::Error<()>>
//     {
//         todo!()
//     }
// }

impl<I2C, InterruptPin, CommE> BMA4xx<I2C, InterruptPin>
    where
        I2C: hal::blocking::i2c::Write<Error = CommE>
            + hal::blocking::i2c::Read<Error = CommE>
            + hal::blocking::i2c::WriteRead<Error = CommE>,
        InterruptPin: hal::digital::v2::InputPin,
{
    pub(crate) fn new(i2c: I2C,
                      interrupt_pin1: Option<InterruptPin>,
                      interrupt_pin2: Option<InterruptPin>,
                    ) -> Self
    {
        let mut _self = Self{
            i2c,
            address: Address::DEFAULT,
            interrupt_pin1,
            interrupt_pin2,
        };

        // verify chip id
        const BMA421_CHIP_ID:u8 = 0x11;
        const BMA4XX_CHIP_ID:u8 = 0x13;
        const SUPPORTED_CHIP_IDS:[u8;2] = [BMA421_CHIP_ID, BMA4XX_CHIP_ID];
        match _self.read_register(Registers::CHIP_ID) {
            Ok(chip_id) =>  {
                if ! SUPPORTED_CHIP_IDS.contains(&chip_id) {
                    panic!("unknown accelerometer chip id [{:?}]", chip_id);
                }
            }
            Err(_) => {
                // try alternative address
                _self.address = Address::ALTERNATIVE;
                match _self.read_register(Registers::CHIP_ID) {
                    Ok(chip_id)     =>  {
                        if ! SUPPORTED_CHIP_IDS.contains(&chip_id) {
                            panic!("unknown accelerometer chip id [{:?}]", chip_id);
                        }
                    },
                    Err(_) => {
                        panic!("unable to talk to accelerometer over I2C");
                    }
                }
            }
        };

        _self
    }

    fn read_register(&mut self, register: Registers ) -> Result<u8, CommE> {
        let request = &[register as u8];
        let mut response:[u8;1] = [0;1];
        match self.i2c.write_read(self.address as u8, request, &mut response) {
            Ok(_)   => return Ok(response[0]),
            Err(comme)  => return Err(comme),
        }
    }

    // const DEFAULT_I2C_ADDRESS:u8 = 0b001_1000;
    // /// chosen by pin-strapping SDO to VDDIO
    // const ALTERNATIVE_I2C_ADDRESS:u8 = 0b001_1001;
}

#[derive(Copy,Clone)]
#[repr(u8)]
/// https://datasheet.lcsc.com/lcsc/1912111437_Bosch-Sensortec-BMA425_C437656.pdf#page=51
enum Address {
    DEFAULT     = 0b001_1000,
    /// chosen by pin-strapping SDO to VDDIO
    ALTERNATIVE = 0b001_1001,
}


#[allow(unused)]
#[allow(non_camel_case_types)]
#[repr(u8)]
/// https://datasheet.lcsc.com/lcsc/1912111437_Bosch-Sensortec-BMA425_C437656.pdf#page=51
enum Registers {
    CMD             = 0x7E,
    PWR_CTRL        = 0x7D,
    PWR_CONF        = 0x7C,
    OFFSET_2        = 0x73,
    OFFSET_1        = 0x72,
    OFFSET_0        = 0x71,
    NV_CONF         = 0x70,
    ACC_SELF_TEST   = 0x6D,
    IF_CONF         = 0x6B,
    NVM_CONF        = 0x6A,
    INTERNAL_ERROR  = 0x5F,
    FEATURES_IN     = 0x5E,
    INIT_CTRL       = 0x59,
    INT_MAP_DATA    = 0x58,
    INT2_MAP        = 0x57,
    INT1_MAP        = 0x56,
    INT_LATCH       = 0x55,
    INT2_IO_CTRL    = 0x54,
    INT1_IO_CTRL    = 0x53,
    AUX_WR_DATA     = 0x4F,
    AUX_WR_ADDR     = 0x4E,
    AUX_RD_ADDR     = 0x4D,
    AUX_IF_CONF     = 0x4C,
    AUX_DEV_ID      = 0x4B,
    FIFO_CONFIG_1   = 0x49,
    FIFO_CONFIG_0   = 0x48,
    FIFO_WTM_1      = 0x47,
    FIFO_WTM_0      = 0x46,
    FIFO_DOWNS      = 0x45,
    AUX_CONF        = 0x44,
    ACC_RANGE       = 0x41,
    ACC_CONF        = 0x40,
    INTERNAL_STATUS = 0x2A,
    ACTIVITY_TYPE   = 0x27,
    FIFO_DATA       = 0x26,
    FIFO_LENGTH_1   = 0x25,
    FIFO_LENGTH_0   = 0x24,
    TEMPERATURE     = 0x22,
    STEP_COUNTER_3  = 0x21,
    STEP_COUNTER_2  = 0x20,
    STEP_COUNTER_1  = 0x1F,
    STEP_COUNTER_0  = 0x1E,
    INT_STATUS_1    = 0x1D,
    INT_STATUS_0    = 0x1C,
    EVENT           = 0x1B,
    SENSOR_TIME_2   = 0x1A,
    SENSOR_TIME_1   = 0x19,
    SENSOR_TIME_0   = 0x18,
    DATA_13         = 0x17,
    DATA_12         = 0x16,
    DATA_11         = 0x15,
    DATA_10         = 0x14,
    DATA_9          = 0x13,
    DATA_8          = 0x12,
    DATA_7          = 0x11,
    DATA_6          = 0x10,
    DATA_5          = 0x0F,
    DATA_4          = 0x0E,
    DATA_3          = 0x0D,
    DATA_2          = 0x0C,
    DATA_1          = 0x0B,
    DATA_0          = 0x0A,
    STATUS          = 0x03,
    ERR_REG         = 0x02,
    CHIP_ID         = 0x00
}