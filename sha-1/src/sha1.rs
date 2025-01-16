pub struct Sha1;

impl Sha1 {
    pub fn new() -> Self { // uppercase Self means return an instance of this datatype
        Sha1{}
    }

    /// Lowercase refers to a specific instance of the data type
    pub fn hash(&self, key:&str) -> [u8;20] {
        let temp_rtn: [u8; 20] = [2;20];
        // hash will be a string of 20 bytes 

        // Padding --> We want even chunks to work on
        let msg: Vec<u8> = self.pad_message(key);
        temp_rtn
    }

    fn pad_message(&self, input:&str) -> Vec<u8> {
        // borrowing &str --> not taking ownership of it, simply reading from it
        // borrowing &self --> not taking ownership of it, simply reading from it
        let mut bytes:Vec<u8> = input.as_bytes().to_vec(); // vectors allow us to add on additional items without having to resize on our own
        
        let original_bit_length: u64 = bytes.len() as u64 * 8; // 8 bits in a byte --> want to get the original input in bits0

        while (bytes.len() * 8) % 512 != 448 { // we want to be 64 bits short of 512
            bytes.push(0);
        }

        // borrowing &original_bit_length --> not taking ownership of it, simply reading from it
        bytes.extend_from_slice(&original_bit_length.to_be_bytes()); // bytes is a vector, then we add on to the end 

        return  // we 'll be basically taking 512 bit chunks
    }
}