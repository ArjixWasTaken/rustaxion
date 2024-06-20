use std::io::{Cursor, Read};

use tokio::io;
use tokio_util::bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use crate::enums::comet::{MainCmd, ParaCmd};

pub struct PacketGlue;

#[derive(Debug, Clone)]
pub struct Packet {
    pub pkg_len: i32,      // 4 bytes
    pub main_cmd: MainCmd, // 1 byte
    pub para_cmd: ParaCmd, // 1 byte
    pub data_len: i16,     // 2 bytes
    pub data: Vec<u8>,     // ?? bytes
}

impl Encoder<Packet> for PacketGlue {
    type Error = io::Error;

    fn encode(&mut self, item: Packet, dst: &mut BytesMut) -> io::Result<()> {
        dst.extend_from_slice(item.pkg_len.to_le_bytes().as_slice());
        dst.put_u8(i8::from(item.main_cmd.clone()) as u8);
        dst.put_u8(item.para_cmd.get_value());
        dst.extend_from_slice(item.data_len.to_le_bytes().as_slice());

        dst.extend_from_slice(&*item.data);

        return Ok(());
    }
}

impl Decoder for PacketGlue {
    type Item = Packet;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> io::Result<Option<Self::Item>> {
        use byteorder::{LittleEndian, ReadBytesExt};

        let mut reader = src.reader();
        let mut reader = reader.get_mut();

        let mut reader = Cursor::new(reader);

        let pkg_len = ReadBytesExt::read_i32::<LittleEndian>(&mut reader)?;

        let main_cmd = MainCmd::try_from(ReadBytesExt::read_i8(&mut reader)?);
        let Ok(main_cmd) = main_cmd else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} is not valid for MainCmd", main_cmd.unwrap_err()),
            ));
        };

        let para_cmd = ParaCmd::from_value(&main_cmd, ReadBytesExt::read_u8(&mut reader)?);
        let Ok(para_cmd) = para_cmd else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} is not valid for ParaCmd", para_cmd.unwrap_err()),
            ));
        };

        let data_len = ReadBytesExt::read_i16::<LittleEndian>(&mut reader)?;

        let mut data = Vec::<u8>::with_capacity(data_len as usize);
        Read::read_exact(&mut reader, &mut data)?;

        Ok(Some(Packet {
            pkg_len,
            main_cmd,
            para_cmd,
            data_len,
            data,
        }))
    }
}
