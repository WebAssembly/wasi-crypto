use std::io::{Cursor, Read};

use super::error::*;
use super::handles::*;
use super::{HandleManagers, WasiCryptoCtx};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArrayOutput(Cursor<Vec<u8>>);

impl ArrayOutput {
    fn len(&self) -> usize {
        self.0.get_ref().len()
    }

    fn pull(&self, buf: &mut [u8]) -> Result<usize, Error> {
        let data = self.0.get_ref();
        let data_len = data.len();
        let buf_len = buf.len();
        ensure!(buf_len >= data_len, CryptoError::Overflow);
        buf.copy_from_slice(data);
        Ok(buf_len)
    }

    pub fn new(data: Vec<u8>) -> Self {
        ArrayOutput(Cursor::new(data))
    }

    pub fn register(handles: &HandleManagers, data: Vec<u8>) -> Result<Handle, Error> {
        let array_output = ArrayOutput::new(data);
        let handle = handles.array_output.register(array_output)?;
        Ok(handle)
    }
}

impl Read for ArrayOutput {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        self.0.read(buf)
    }
}

impl WasiCryptoCtx {
    pub fn array_output_len(&self, array_output_handle: Handle) -> Result<usize, Error> {
        let array_output = self.handles.array_output.get(array_output_handle)?;
        Ok(array_output.len())
    }

    pub fn array_output_pull(
        &self,
        array_output_handle: Handle,
        buf: &mut [u8],
    ) -> Result<usize, Error> {
        let array_output = self.handles.array_output.get(array_output_handle)?;
        let len = array_output.pull(buf)?;
        self.handles.array_output.close(array_output_handle)?;
        Ok(len)
    }
}
