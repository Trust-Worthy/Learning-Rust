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
    pub fn hash(&mut self, key:&str) -> [u8;20] { // changed self 
        // hash will be a string of 20 bytes 

        // Padding --> We want even chunks to work on
        let (mut h0, mut h1, mut h2, mut h3, mut h4): (u32,u32,u32,u32,u32) = (H0,H1,H2,H3,H4); // tuple of u32's --> these are accumulator values --> we'll be adding onto them as we go along
        // --> at the end when we put all of these h values together to get the final 20 byte hash

        let msg: Vec<u8> = self.pad_message(key);
        
        for chunk in msg.chunks(64) { // because we did the padding, we know that we can get an even & exact num of 64 bit chunks
        
            let schedule = self.build_schedule(chunk);

            let (mut a, mut b, mut c, mut d, mut e) = (H0, H1, H2, H3, H4);
            for i in 0..80 { // doing this loop 80 times 
                let (f, k) = match i {
                     0..=19 => ((b & c) | ((!b) & d), 0x5A827999),
                     20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                     40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                     _ => (b ^ c ^ d, 0xCA62C1D6),
                 };

                 // diffusion across different portions of the hash
                 // needed the mut keyword on a - e because we're changing the values here 
                 let temp = a
                    .rotate_left(5) // rotate operation operation that isn't the same as bit shift again! 
                    .wrapping_add(f)
                    .wrapping_add(e)
                    .wrapping_add(k)
                    .wrapping_add(schedule[i]);
                e = d;
                d = c;
                c = b.rotate_left(30);
                b = a;
                a = temp;
            }
            // add the compressed chunk to the current hash value.
            h0 = h0.wrapping_add(a);
            h1 = h1.wrapping_add(b);
            h2 = h2.wrapping_add(c);
            h3 = h3.wrapping_add(d);
            h4 = h4.wrapping_add(e);

                // * you never want one portion of a variable to impact the end has more than any other variable
        }

        let mut hash = [0u8;20]; // final hash to be returned

        hash[0..4].copy_from_slice(&h0.to_be_bytes());
        hash[4..8].copy_from_slice(&h1.to_be_bytes());
        hash[8..12].copy_from_slice(&h2.to_be_bytes());
        hash[12..16].copy_from_slice(&h3.to_be_bytes());
        hash[16..20].copy_from_slice(&h4.to_be_bytes());


        return hash
    }

    fn build_schedule(&mut self, chunk: &[u8]) -> [u32;80] {
        let mut schedule: [u32;80] = [0u32; 80]; // going to be filling this up with the first 16 u32 


        for (i,block) in chunk.chunks(4).enumerate() { // take a 4 byte integer out at a time and shove it into the array
            schedule[i] = u32::from_be_bytes(block.try_into().unwrap());
        }

        // use the first 16 bytes to create teh 17th byte 
        for i in 16..80  { // 0 - 15 are the first 16 integers
            // ohh I get it. 17 up to 80 integers are all determined based on the previous one 
            schedule[i] = schedule[i -3] ^ schedule[i - 8] ^ schedule[i - 14] ^ schedule[i - 16]; // this does a bitwise xor
            schedule[i] = schedule[i].rotate_left(1) // this is different than bitshifting <<
            // rotating takes the left most bit and puts it in the rightmost spot and shifts everything over
            // ex 1011
            // goes to 0111
        }
        return schedule
    }

    fn pad_message(&self, input:&str) -> Vec<u8> {
        // borrowing &str --> not taking ownership of it, simply reading from it
        // borrowing &self --> not taking ownership of it, simply reading from it
        let mut bytes:Vec<u8> = input.as_bytes().to_vec(); // vectors allow us to add on additional items without having to resize on our own
        
        //println!("ascii vals of bytes: {:?}",bytes);
        let original_bit_length: u64 = bytes.len() as u64 * 8; // 8 bits in a byte --> want to get the original input in bits0
        
        //println!("original bit length of those bytes: {:?}",original_bit_length);
        bytes.push(0x80); // delimeter between old and new data
        // [bytes] 1000000 [padding] [bit length]                                     0x80 = 128 = 1000000 

        //println!(" delimeter of 0x80 added to those bytes: {:?}",bytes);
        while (bytes.len() * 8) % 512 != 448 { // we want to be 64 bits short of 512
            bytes.push(0);
        }

        //println!(" padding added to byte: {:?}",bytes);
        // borrowing &original_bit_length --> not taking ownership of it, simply reading from it
        bytes.extend_from_slice(&original_bit_length.to_be_bytes()); // bytes is a vector, then we add on to the end 
        //println!(" original bit length of {original_bit_length} added to the bytes: {:?}",bytes);

        return  bytes // we 'll be basically taking 512 bit chunks

        // steps
        // 1. break string down into vector of bytes
        // 2. get bit length of string
        // 3. add delimeter
        // 3. loop through original bytes array and pad with zeros
        // 5. add the original bit length on to the end
    }
}