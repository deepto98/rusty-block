use blockchainlib::*;

fn main() {
    let mut block = Block::new(0, now(), vec![0; 32], 0, "Genesis Block".to_owned());
    println!("{:?}", &block);

    // How the hash  works - Block implements Hashable.bytes. that is used in Hashable.hash() to generate the hash for the block
    let h = block.hash();

    println!("{:?}", &h);

    //Set generated hash as hash of current block
    block.hash=h;

    println!("{:?}", &block);
}
