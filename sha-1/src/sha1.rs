// sha1.rs

// These constants are derived from the fractional parts of the square roots of the first five primes.
const H0: u32 = 0x67452301;
const H1: u32 = 0xEFCDAB89;
const H2: u32 = 0x98BADCFE;
const H3: u32 = 0x10325476;
const H4: u32 = 0xC3D2E1F0;

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
        let (mut h0, mut h1, mut h2, mut h3, mut h4): (u32,u32,u32,u32,u32) = (H0,H1,H2,H3,H4); // tuple of u32's --> these are accumulator values --> we'll be adding onto them as we go along
        let msg: Vec<u8> = self.pad_message(key);


        temp_rtn
    }

    fn pad_message(&self, input:&str) -> Vec<u8> {
        // borrowing &str --> not taking ownership of it, simply reading from it
        // borrowing &self --> not taking ownership of it, simply reading from it
        let mut bytes:Vec<u8> = input.as_bytes().to_vec(); // vectors allow us to add on additional items without having to resize on our own
        
        println!("ascii vals of bytes: {:?}",bytes);
        let original_bit_length: u64 = bytes.len() as u64 * 8; // 8 bits in a byte --> want to get the original input in bits0
        
        println!("original bit length of those bytes: {:?}",original_bit_length);
        bytes.push(0x80); // delimeter between old and new data
        // [bytes] 1000000 [padding] [bit length]                                     0x80 = 128 = 1000000 

        println!(" delimeter of 0x80 added to those bytes: {:?}",bytes);
        while (bytes.len() * 8) % 512 != 448 { // we want to be 64 bits short of 512
            bytes.push(0);
        }

        println!(" padding added to byte: {:?}",bytes);
        // borrowing &original_bit_length --> not taking ownership of it, simply reading from it
        bytes.extend_from_slice(&original_bit_length.to_be_bytes()); // bytes is a vector, then we add on to the end 
        println!(" original bit length of {original_bit_length} added to the bytes: {:?}",bytes);

        return  bytes // we 'll be basically taking 512 bit chunks

        // steps
        // 1. break string down into vector of bytes
        // 2. get bit length of string
        // 3. add delimeter
        // 3. loop through original bytes array and pad with zeros
        // 5. add the original bit length on to the end
    }
}