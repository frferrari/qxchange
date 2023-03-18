
use qrcode_generator::QrCodeEcc;
use quircs::Code;
use super::encryption::Encryption;
use super::QrCode;

pub trait Renderer<E> where E: Encryption {
    fn write_to_byte_array(&self, encryption: &E) -> Vec<u8>;
    fn write_to_png_file(&self, encryption: &E, qrcode_ecc: QrCodeEcc, png_filename: impl Into<String>);
    fn read_from_byte_array(&self, encryption: &E) -> Vec<u8>;
    fn read_from_quirc_code(encryption: &E, quirc_code: &Code) -> QrCode;
}

impl<E> Renderer<E> for QrCode where E: Encryption {
    fn write_to_byte_array(&self, encryption: &E) -> Vec<u8> {
        encryption.cypher(self)
    }

    fn write_to_png_file(&self, encryption: &E, qrcode_ecc: QrCodeEcc, png_filename: impl Into<String>) {
        let encrypted_data = self.write_to_byte_array(encryption);
        qrcode_generator::to_png_to_file(encrypted_data, qrcode_ecc, 1024, png_filename.into()).unwrap();
    }

    fn read_from_byte_array(&self, encryption: &E) -> Vec<u8> {
        encryption.decypher(&self.data).unwrap() // TODO Remove unwrap, change function return type
    }

    fn read_from_quirc_code(encryption: &E, quirc_code: &Code) -> QrCode {
        let data = quirc_code.decode().unwrap().payload;
        let bytes = encryption.decypher(&data).unwrap();

        QrCode { data: bytes }
    }
}
