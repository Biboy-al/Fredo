
use base64::{engine::general_purpose, Engine as _};
use rsa::{pkcs8::DecodePublicKey,  Pkcs1v15Encrypt, RsaPublicKey};
use rand::rngs::OsRng;

//Struct to encode file
//The encoder uses XOR perserving 
//shifting the bit the right
pub struct EncodeFile{
    org_key: u8,
    key: u8
}

//struct to encode connection data
//the encode uses a normal XOR perserving
//Uses public key encryption for registery
pub struct EncodeConnection{
    key: u8,
    pub_key: String
}


//Implementation of file encoder
impl EncodeFile{

    //constructor of encdor
    pub fn new(key: u8) -> EncodeFile{

        EncodeFile{
            org_key: key,
            key: key
        }
    }

    //encrypt the file using XOR, encoding it to base64
    //XOR is used for simple and cheap encryption
    //Encoded in base64 to make it writeable or printable
    pub fn encrypt(&mut self, plain_text: & str ) ->  String{
               let mut encode_string = Vec::new();
        
        //byte by byte of string perform encrypt it
        for byte in plain_text.as_bytes(){
            
            //make sure byte is not null
            if *byte != 0 {
                let encoded_byte = *byte ^ self.key;
                encode_string.push(encoded_byte);
                //after encoding one shift it to right
                self.key = self.key.rotate_right(1);

            }else{
                encode_string.push(0);

            }
            
        }

        //encode it to base 64
        general_purpose::STANDARD.encode(&encode_string)
        
    }

    //function to decrypt files
    //function is a complete inverse to the encryption
    pub fn decrypt(&mut self, base_cipher: & str ) ->  String{

        //decode from base 64 getting the bytes
        let cipher = general_purpose::STANDARD.decode(&base_cipher).expect("Cannot encode");

        let mut encode_string = Vec::new();
        
        //byte by byte decoding it
        for byte in cipher{
            
            if byte != 0 {
                let encoded_byte = byte ^ self.key;
                encode_string.push(encoded_byte);
                //shift to the right
                self.key = self.key.rotate_right(1);

            }else{
                encode_string.push(0);

            }
            
        }

        //collect it into a string, this is the plain text
        encode_string.iter().map(|&b| b as char).collect()    
        
    }

    //key resteing function
    //this makes it so that for each encryption they key does not rest
    //and so that that decrpytion has the same starting key
    pub fn reset_key(&mut self){

        self.key = self.org_key;

    }


} 

//the implementation of connection encoder
impl EncodeConnection{

    //constructor
    pub fn new(key: u8, pub_key: String) -> EncodeConnection{

        EncodeConnection{
            key: key,
            pub_key: pub_key
        }
    }

    //function to encrypt data over the internet
    //Makes it hard for analysis to read file without the key
    //encoding it to base 64
    pub fn encrypt(&self, plain_text: & str ) ->  String{

        let mut encode_string = Vec::new();
        
        //byte by byte of string perform encrypt it
        for byte in plain_text.as_bytes(){
            
            if *byte != 0 {
                let encoded_byte = *byte ^ self.key;
                encode_string.push(encoded_byte);

            }else{
                encode_string.push(0);

            }
            
        }

        //encode it to base 64
        general_purpose::STANDARD.encode(&encode_string)
        
    }

      //function to decrypt data over the internet
    //function is a complete inverse to the encryption
    pub fn decrypt(& self, base_cipher: & str ) ->  String{

        //decode the cipher text from base 64
        let cipher = general_purpose::STANDARD.decode(&base_cipher).expect("Cannot encode");

        let mut encode_string = Vec::new();
        
        //byte by byte of string perform encrypt it
        for byte in cipher{
            
            if byte != 0 {
                let encoded_byte = byte ^ self.key;
                encode_string.push(encoded_byte);

            }else{
                encode_string.push(0);

            }
            
        }

        //return plaintext
        encode_string.iter().map(|&b| b as char).collect()    
        
    }


    //function to encrypt the data using the c2 server's public key
    //this is used during registration to send a shared key
    //this ensures authentication of server is only the server can decrpyt it
    //This makes the server harder to spoof
    pub fn pub_key_enc(&self, plaintext: & str) ->String {
        
        //load the public key
        let pub_key = RsaPublicKey::from_public_key_pem(&self.pub_key).unwrap();

        //sets up the data needed to encrypt the file
        let padding = Pkcs1v15Encrypt;  
        let encrypted = pub_key.encrypt(&mut OsRng , padding,  plaintext.as_bytes()).unwrap();

        //encode the encrypted file into base64
        general_purpose::STANDARD.encode(encrypted)
    }

    //getter function to get xor key
    pub fn get_key(& self) -> u8{
        self.key
    }

} 