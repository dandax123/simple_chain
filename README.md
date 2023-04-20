### Simple Merkletree and Chain implementation

Merkletree: Simple datastructure to build hash references of data (uses SHA256 for hashing)  
Block: Contains a merkletree, the `prev_block_hash`, nonce and current block hash.  
Chain: Mine new blocks and add to the vector of blocks
