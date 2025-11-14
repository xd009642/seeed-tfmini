#![no_std]

pub struct Frame {
    pub distance: u16,
    pub strength: u16,
    pub integration_time: u8,
}

impl Frame {
    pub(crate) fn new(data: &[u8; 9]) -> Self {
        Frame {
            distance: (data[2] as u16) | ((data[3] as u16) << 8),
            strength: (data[4] as u16) | ((data[5] as u16) << 8),
            integration_time: data[6],
        }
    }

    pub(crate) fn check_header(data: &[u8; 9]) -> bool {
        data[0] == 0x59 && data[1] == 0x59
    }

    pub(crate) fn validate_checksum(data: &[u8; 9]) -> bool {
        let checksum =
            data[0] + data[1] + data[2] + data[3] + data[4] + data[5] + data[6] + data[7];
        checksum == data[8]
    }
}

#[cfg(feature = "embedded-io")]
mod sync_driver {
    use super::*;
    use embedded_io::{ErrorType, Read, ReadExactError};

    pub enum FrameError<Error> {
        InvalidStartFrame,
        InvalidChecksum,
        ReadError(ReadExactError<Error>),
    }

    struct TfMini<Bus> {
        bus: Bus,
    }

    impl<Bus: Read> TfMini<Bus> {
        pub fn decode_frame(&mut self) -> Result<Frame, FrameError<<Bus as ErrorType>::Error>> {
            let mut scratchpad = [0u8; 9];
            self.decode_frame_with_scratchpad(&mut scratchpad)
        }

        pub fn decode_frame_with_scratchpad(
            &mut self,
            src: &mut [u8; 9],
        ) -> Result<Frame, FrameError<<Bus as ErrorType>::Error>> {
            self.bus
                .read_exact(src.as_mut_slice())
                .map_err(FrameError::ReadError)?;

            if !Frame::check_header(src) {
                return Err(FrameError::InvalidStartFrame);
            }
            if !Frame::validate_checksum(src) {
                return Err(FrameError::InvalidChecksum);
            }

            Ok(Frame::new(src))
        }
    }
}

#[cfg(feature = "embedded-io-async")]
mod async_driver {
    use super::*;
    use embedded_io_async::{ErrorType, Read, ReadExactError};

    pub enum FrameError<Error> {
        InvalidStartFrame,
        InvalidChecksum,
        ReadError(ReadExactError<Error>),
    }

    struct TfMini<Bus> {
        bus: Bus,
    }

    impl<Bus: Read> TfMini<Bus> {
        pub async fn decode_frame(
            &mut self,
        ) -> Result<Frame, FrameError<<Bus as ErrorType>::Error>> {
            let mut scratchpad = [0u8; 9];
            self.decode_frame_with_scratchpad(&mut scratchpad).await
        }

        pub async fn decode_frame_with_scratchpad(
            &mut self,
            src: &mut [u8; 9],
        ) -> Result<Frame, FrameError<<Bus as ErrorType>::Error>> {
            self.bus
                .read_exact(src.as_mut_slice())
                .await
                .map_err(FrameError::ReadError)?;

            if !Frame::check_header(src) {
                return Err(FrameError::InvalidStartFrame);
            }
            if !Frame::validate_checksum(src) {
                return Err(FrameError::InvalidChecksum);
            }

            Ok(Frame::new(src))
        }
    }
}
