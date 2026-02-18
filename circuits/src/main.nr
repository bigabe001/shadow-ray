use dep::std;

fn main(
    x: u32, 
    y: u32, 
    salt: Field, 
    pub_commitment: pub Field
) {
    // 1. PROTOCOL 25 OPTIMIZATION: Use Poseidon instead of Pedersen
    // Poseidon is the "X-Ray" standard for high-performance ZK gaming.
    let hash = std::hash::poseidon::bn254::hash_3([x as Field, y as Field, salt]);
    
    // 2. PROVABLE VALIDITY: Ensure the private secret matches the public commitment
    assert(hash == pub_commitment);
    
    // 3. ESSENTIAL GAMEPLAY: Proves the unit is on the 10x10 "Shadow Ray" grid
    // This is the "Fairness" mechanic: you can't hide a unit outside the map!
    assert(x <= 10);
    assert(y <= 10);
}