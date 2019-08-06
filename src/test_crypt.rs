// test_crypt.rs ----------------------------------------------------------------------------------------------------------------

#![allow(unused_variables)]
#![allow(dead_code)] 

//------------------------------------------------------------------------------------------------------------------------------

pub struct DHEndpoint {
    public_key1: u64,
    public_key2: u64,
    private_key: u64, 
}

impl DHEndpoint  {
    pub fn new( pub_key1: u64, pub_key2: u64, priv_key: u64) -> Self {
        Self {
            public_key1: pub_key1,
            public_key2: pub_key2,
            private_key: priv_key
        }
    }

    pub fn  generate_partial_key( &self) -> u64 {
        let prod = self.public_key1 * self.private_key;
        let partial_key = prod % self.public_key2;
        partial_key
    }

    pub fn  generate_full_key( &self, partial_key_r: u64) -> u64 {
        let     prod  = partial_key_r *self.private_key;
        let     full_key = prod % self.public_key2;
        full_key
    }
}
 
pub struct DHChanel<'a>   {
    endppoint: &'a DHEndpoint, 
    full_key: u64,
}

impl<'a> DHChanel<'a>   {
    pub fn new( ep: &'a DHEndpoint,  partial_key_r: u64) -> Self {
        Self {
            endppoint: ep,
            full_key: ep.generate_full_key( partial_key_r),            
        }
    }
}

pub fn test_crypt() {

    let     pub_key1 = 97;
    let     priv_key1 = 37;
    
    let     pub_key2 = 61;
    let     priv_key2 = 31;
    
    let ep1 = DHEndpoint::new( pub_key1, pub_key2, priv_key1);
    let ep2 = DHEndpoint::new( pub_key2, pub_key1, priv_key2);
    

}

/*
class DH_Endpoint(object):
    def __init__(self, public_key1, public_key2, private_key):
        self.public_key1 = public_key1
        self.public_key2 = public_key2
        self.private_key = private_key
        self.full_key = None
        
    def generate_partial_key(self):
        partial_key = self.public_key1**self.private_key
        partial_key = partial_key%self.public_key2
        return partial_key
    
    def generate_full_key(self, partial_key_r):
        full_key = partial_key_r**self.private_key
        full_key = full_key%self.public_key2
        self.full_key = full_key
        return full_key
    
    def encrypt_message(self, message):
        encrypted_message = ""
        key = self.full_key
        for c in message:
            encrypted_message += chr(ord(c)+key)
        return encrypted_message
    
    def decrypt_message(self, encrypted_message):
        decrypted_message = ""
        key = self.full_key
        for c in encrypted_message:
            decrypted_message += chr(ord(c)-key)
        return decrypted_message

*/

//------------------------------------------------------------------------------------------------------------------------------
