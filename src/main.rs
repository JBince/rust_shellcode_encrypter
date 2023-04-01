use clap::Parser;
use libaes::Cipher;
use std::fs::File;
use std::io::Read;
use std::io::Write;
fn main() {
    //Parse command line arguments
    let args = Args::new();

    //Check that key is 32 bytes long
    let mut key = [0u8; 32];
    if args.key.len() < 32 || args.key.len() > 32 {
        println!("[!] Your key is {} bytes long. It must be 32 bytes long to accommodate AES-256. Using default key 'This is a key and it's 32 bytes!'", args.key.len());
        std::process::exit(1);
    } else {
        key.copy_from_slice(args.key.as_bytes());
    }

    //Set key & IV
    let iv = args.iv.as_bytes();
    let cipher = Cipher::new_256(&key);

    //Path to binary shellcode file
    let input_file = args.input_file;

    //Read into buffer
    let mut f = File::open(&input_file).unwrap();
    let metadata = std::fs::metadata(&input_file).unwrap();
    let mut buffer: Vec<u8> = vec![0; metadata.len() as usize];
    f.read(&mut buffer).unwrap();

    //Ensure buffer is a u8 array
    let shellcode = &buffer[..];

    //Encrypt the shellcode buffer
    let ciphertext = cipher.cbc_encrypt(iv, shellcode);
    let mut out_file = File::create(args.output_file).unwrap();

    //Write it to desired file
    out_file.write_all(&ciphertext).unwrap();
}

//Parse arguments
#[derive(Parser, Debug)]
struct Args {
    /// The key to use for encryption/decryption. Must be 32 bytes to accommodate AES-256
    #[clap(short, long, default_value = "This is a key and it's 32 bytes!")]
    key: String,

    ///Initialization vector used in AES-256 encryption
    #[clap(long, default_value = "This is 16 bytes!!")]
    iv: String,

    ///The binary shellcode file to be encrypted
    #[clap(short, long, default_value = "shellcode.bin")]
    input_file: String,

    ///The output file to write the encrypted shellcode to
    #[clap(short, long, default_value = "encrypted_shellcode.bin")]
    output_file: String,
}
impl Args {
    fn new() -> Self {
        Args::parse()
    }
}
