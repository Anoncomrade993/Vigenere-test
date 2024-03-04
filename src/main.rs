use vigenere::cipher::Algorithm;

fn main() {
  //TODO: Implemnt a random key generator for security reasons
  //it will be better if it has same length as the data to be  ciphered
    let key = "secret";
    let message = "Hello, world!";
    let encoded = Algorithm::encode(message, key);
    println!("Encoded: {}", encoded);
}
