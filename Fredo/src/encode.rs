
use base64::{engine::general_purpose, Engine as _};
pub struct Encode{
    org_key: u8,
    key: u8
}

pub struct EncodeConnection{
    key: u8
}


impl Encode{

    pub fn new(key: u8) -> Encode{

        Encode{
            org_key: key,
            key: key
        }
    }

    pub fn encrypt(&mut self, plain_text: & str ) ->  String{
               let mut encode_string = Vec::new();
        
        for byte in plain_text.as_bytes(){
            
            if *byte != 0 {
                let encoded_byte = *byte ^ self.key;
                encode_string.push(encoded_byte);
                self.key = self.key.rotate_right(1);

            }else{
                encode_string.push(0);

            }
            
        }
        general_purpose::STANDARD.encode(&encode_string)
        
    }

    pub fn decrypt(&mut self, base_cipher: & str ) ->  String{

        let cipher = general_purpose::STANDARD.decode(&base_cipher).expect("Cannot encode");

        let mut encode_string = Vec::new();
        
        for byte in cipher{
            
            if byte != 0 {
                let encoded_byte = byte ^ self.key;
                encode_string.push(encoded_byte);
                self.key = self.key.rotate_right(1);

            }else{
                encode_string.push(0);

            }
            
        }

        encode_string.iter().map(|&b| b as char).collect()    
        
    }

    pub fn reset_key(&mut self){

        self.key = self.org_key;

    }


} 


impl EncodeConnection{

    pub fn new(key: u8) -> EncodeConnection{

        EncodeConnection{
            key: key
        }
    }

    pub fn encrypt(&self, plain_text: & str ) ->  String{
               let mut encode_string = Vec::new();
        
        for byte in plain_text.as_bytes(){
            
            if *byte != 0 {
                let encoded_byte = *byte ^ self.key;
                encode_string.push(encoded_byte);

            }else{
                encode_string.push(0);

            }
            
        }
        general_purpose::STANDARD.encode(&encode_string)
        
    }

    pub fn decrypt(& self, base_cipher: & str ) ->  String{

        let cipher = general_purpose::STANDARD.decode(&base_cipher).expect("Cannot encode");

        let mut encode_string = Vec::new();
        
        for byte in cipher{
            
            if byte != 0 {
                let encoded_byte = byte ^ self.key;
                encode_string.push(encoded_byte);

            }else{
                encode_string.push(0);

            }
            
        }

        encode_string.iter().map(|&b| b as char).collect()    
        
    }

    pub fn get_key(& self) -> u8{
        self.key
    }

} 