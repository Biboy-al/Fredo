
use base64::{engine::general_purpose, Engine as _};
use rsa::{pkcs8::DecodePublicKey,  Pkcs1v15Encrypt, RsaPublicKey};
use rand::rngs::OsRng;
pub struct Encode{
    org_key: u8,
    key: u8
}

pub struct EncodeConnection{
    key: u8,
    pub_key: String
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

    pub fn new(key: u8, pub_key: String) -> EncodeConnection{

        EncodeConnection{
            key: key,
            pub_key: pub_key
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

    pub fn pub_key_enc(&self, plaintext: & str) ->String {
 
        let pub_key = RsaPublicKey::from_public_key_pem(&self.pub_key).unwrap();


        let padding = Pkcs1v15Encrypt;
        let encrypted = pub_key.encrypt(&mut OsRng , padding,  plaintext.as_bytes()).unwrap();

        general_purpose::STANDARD.encode(encrypted)
    }

    pub fn get_key(& self) -> u8{
        self.key
    }

} 