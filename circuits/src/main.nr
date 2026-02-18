use dep::std;

fn main(
    x: u32, 
    y: u32, 
    salt: Field, 
    pub_commitment: pub Field
) {
    // Create a Pedersen commitment (the "Fingerprint" of your location)
    // The 'salt' prevents people from guessing your X/Y coordinates.
    let hash = std::hash::pedersen_hash([x as Field, y as Field, salt]);
    
    // Proves the coordinates you hold match the hash on the blockchain
    assert(hash == pub_commitment);
    
    // Boundary check: Proves the shadow is within the 100x100 game board
    assert(x <= 100);
    assert(y <= 100);
}