// A Merkle Tree is a concept often used in Blockchains.
// It is a binary tree where each leaf node represents the hash of some interesting data
// and each internal node is the hash of the concatenated contents of its two children.
// Merkle Trees often record groups of transactions, and the roots are published widely to
// serve as summaries of all recognised transactions on a given date.

// By construction, the tree's root is a hash of all its leaves organised in a specific order.
// Since hash functions are hard to reverse, it is unfeasible to create a tree with a specific
// root if we don't know the inputs to it.
// We can prove a transaction happened before a certain date by showing it was a leaf of Merkle Tree
// that has already been published.
// Merkle Trees provide an efficient way to prove this inclusion in the tree.
// It is enough to show a path of neighbour-nodes from the leaf to the root.
// That is, a list that includes the sibling of the leaf node, then the sibling of its parent and so on
// until the root is reached.

// The code in this file represents a library that can create Merkle trees or return a proof of inclusion
// of a given leaf in a Merkle tree.
// It also includes tests to demonstrate how to use the library functions.

// EXERCISE:

// Your goal in this exercise is two-fold:
// 1. Imagine you receive this code in a Github Pull-Request submitted by one of your team mates.
// Write a code review for it with comments as you see fit.
//
// 2. Improve the code if you are able. Ensure it builds and runs, and that tests pass.

use std::time::Instant;

// ----------------- Tests -----------------------------------

#[test]
fn test_generate_root_1() {
    let mock_trees_roots = [
        "bc36789e7a1e281436464229828f817d6612f7b477d66591ff96a9e064bcc98a",
        "b2521d64679bc4720dabfbae7ce17947a5d373d987d3b0cc1e3042ba2054da4a",
        "d359d2743bb3a93ded4c902716931497ae3080f478c14e7af96344a92e9ddd51",
        "fecce4ac8ed6fc57f4d880d6af2b443418d564df8f5d52c6782e952564ed79eb",
        "11aeafa56c9b34805cc86b1c320c9331672c07e600f0a44317051cfa05a0c296",
        "ce24ba488022147ace7a2a962b481707002c079d7c7ca85b108f7489aaedabba",
        "49b36fbd8a6e3a5ea292f621a38d0afa8ac580c56090a9b0d93e0d06b37d1a89",
        "49c6f5244cba156c2170135c98a77f6fc9b812eb201aefcd3e32c38dfcec711a",
        "f54e6007f25df4d2c75a2ec526e4a635dac09b622497862f6062f9340f25ca81",
        "29da8f3f81c6c9dc74665e28dcbfc1645629746613cccbd76c3f8ccd6b1488ae",
    ];

    for x in 1..=mock_trees_roots.len() {
        let nodes = get_n_nodes(x as u32);
        println!("Running test {x}");
        assert_eq!(
            generate_tree_root(nodes).unwrap(),
            H256(to_array(
                hex::decode(mock_trees_roots[(x - 1) as usize])
                    .unwrap()
                    .as_slice()
            ))
        );
    }
}

#[test]
fn test_generate_root_2() {
    assert!(generate_tree_root(get_n_nodes(0)).is_err());
}

#[test]
fn test_generate_root_3() {
    // resistance test
    let now = Instant::now();
    let long_test = get_n_nodes(1000000);
    println!("Number of nodes: {}", long_test.len());
    println!("Creating leaves: {}", now.elapsed().as_millis());

    assert!(!generate_tree_root(long_test).is_err());
    println!("Building tree: {}", now.elapsed().as_millis());
}

#[test]
fn test_generate_path_1() {
    let mocked_merkle_paths: Vec<(u8, usize, Vec<H256>)> = vec![
        // Mocked one node tree data [0]
        (1, 0, vec![]),
        // Mocked two nodes tree data [0, 1]
        (
            2,
            0,
            vec![H256(to_array(
                hex::decode("5fe7f977e71dba2ea1a68e21057beebb9be2ac30c6410aa38d4f3fbe41dcffd2")
                    .unwrap()
                    .as_slice(),
            ))],
        ),
        (
            2,
            1,
            vec![H256(to_array(
                hex::decode("bc36789e7a1e281436464229828f817d6612f7b477d66591ff96a9e064bcc98a")
                    .unwrap()
                    .as_slice(),
            ))],
        ),
        // Mocked three nodes tree data [0, 1, 2]
        (
            3,
            0,
            vec![
                H256(to_array(
                    hex::decode("f2ee15ea639b73fa3db9b34a245bdfa015c260c598b211bf05a1ecc4b3e3b4f2")
                        .unwrap()
                        .as_slice(),
                )),
                H256(to_array(
                    hex::decode("5fe7f977e71dba2ea1a68e21057beebb9be2ac30c6410aa38d4f3fbe41dcffd2")
                        .unwrap()
                        .as_slice(),
                )),
            ],
        ),
        (
            3,
            1,
            vec![
                H256(to_array(
                    hex::decode("f2ee15ea639b73fa3db9b34a245bdfa015c260c598b211bf05a1ecc4b3e3b4f2")
                        .unwrap()
                        .as_slice(),
                )),
                H256(to_array(
                    hex::decode("bc36789e7a1e281436464229828f817d6612f7b477d66591ff96a9e064bcc98a")
                        .unwrap()
                        .as_slice(),
                )),
            ],
        ),
        (
            3,
            2,
            vec![H256(to_array(
                hex::decode("b2521d64679bc4720dabfbae7ce17947a5d373d987d3b0cc1e3042ba2054da4a")
                    .unwrap()
                    .as_slice(),
            ))],
        ),
        // Mocked five nodes tree data [0, 1, 2, 3, 4]
        (
            5,
            0,
            vec![
                H256(to_array(
                    hex::decode("f343681465b9efe82c933c3e8748c70cb8aa06539c361de20f72eac04e766393")
                        .unwrap()
                        .as_slice(),
                )),
                H256(to_array(
                    hex::decode("c144ad52449a5832e51e7d4daca4c86a9aafc33d89ef15ff7908956d0edb977d")
                        .unwrap()
                        .as_slice(),
                )),
                H256(to_array(
                    hex::decode("5fe7f977e71dba2ea1a68e21057beebb9be2ac30c6410aa38d4f3fbe41dcffd2")
                        .unwrap()
                        .as_slice(),
                )),
            ],
        ),
        (
            5,
            1,
            vec![
                H256(to_array(
                    hex::decode("f343681465b9efe82c933c3e8748c70cb8aa06539c361de20f72eac04e766393")
                        .unwrap()
                        .as_slice(),
                )),
                H256(to_array(
                    hex::decode("c144ad52449a5832e51e7d4daca4c86a9aafc33d89ef15ff7908956d0edb977d")
                        .unwrap()
                        .as_slice(),
                )),
                H256(to_array(
                    hex::decode("bc36789e7a1e281436464229828f817d6612f7b477d66591ff96a9e064bcc98a")
                        .unwrap()
                        .as_slice(),
                )),
            ],
        ),
        (
            5,
            2,
            vec![
                H256(to_array(
                    hex::decode("f343681465b9efe82c933c3e8748c70cb8aa06539c361de20f72eac04e766393")
                        .unwrap()
                        .as_slice(),
                )),
                H256(to_array(
                    hex::decode("b2521d64679bc4720dabfbae7ce17947a5d373d987d3b0cc1e3042ba2054da4a")
                        .unwrap()
                        .as_slice(),
                )),
                H256(to_array(
                    hex::decode("69c322e3248a5dfc29d73c5b0553b0185a35cd5bb6386747517ef7e53b15e287")
                        .unwrap()
                        .as_slice(),
                )),
            ],
        ),
        (
            5,
            3,
            vec![
                H256(to_array(
                    hex::decode("f343681465b9efe82c933c3e8748c70cb8aa06539c361de20f72eac04e766393")
                        .unwrap()
                        .as_slice(),
                )),
                H256(to_array(
                    hex::decode("b2521d64679bc4720dabfbae7ce17947a5d373d987d3b0cc1e3042ba2054da4a")
                        .unwrap()
                        .as_slice(),
                )),
                H256(to_array(
                    hex::decode("f2ee15ea639b73fa3db9b34a245bdfa015c260c598b211bf05a1ecc4b3e3b4f2")
                        .unwrap()
                        .as_slice(),
                )),
            ],
        ),
        (
            5,
            4,
            vec![H256(to_array(
                hex::decode("fecce4ac8ed6fc57f4d880d6af2b443418d564df8f5d52c6782e952564ed79eb")
                    .unwrap()
                    .as_slice(),
            ))],
        ),
    ];

    for (n, i, paht) in mocked_merkle_paths {
        let nodes = get_n_nodes(n as u32);
        assert_eq!(
            gnereate_merkle_path(&nodes[i].clone(), nodes).unwrap(),
            paht
        );
    }
}

#[test]
fn test_generate_path_2() {
    assert!(gnereate_merkle_path(&vec![0], get_n_nodes(0)).is_err());
}

#[test]
fn test_generate_path_3() {
    assert!(gnereate_merkle_path(&vec![], get_n_nodes(1)).is_err());
}

// ----------------- Library code ----------------------------

use jsonrpc_core::{Error, ErrorCode};
use serde::{Deserialize, Serialize};
use sp_core::{hashing::keccak_256, H256};

#[derive(Clone, Serialize, Deserialize, Debug)]
// Contains all the data required to prove that `encoded_leaf` is part of a merkle tree.
pub struct MerklePathData {
    // abi encoded leaf which can be decoded by Ethereum
    pub encoded_leaf: EncodedLeafData,
    // Merkle path to prove the inclusion of the `encoded_leaf` in a merkle tree
    pub merkle_path: Vec<H256>,
}

// Vector of bytes that represents abi encoded leaf data
pub type EncodedLeafData = Vec<u8>;

// Generates a merkle tree and returns the root hash
pub fn generate_tree_root(leaves_data: Vec<Vec<u8>>) -> Result<H256, Error> {
    let mut nodes_hashes: Vec<H256> = encode_to_hashed_nodes(leaves_data);

    let result = recurse(&mut nodes_hashes);

    if result.len() != 1 {
        return Err(Error {
            code: ErrorCode::ServerError(1),
            message: "Error generating merkle root".into(),
            data: None,
        });
    }

    return Ok(result[0]);
}

// Keys:
//  N: number of nodes
//  i: index of sum
// Sum_i=1^log(N) O(N/2^(i+1)) = O(N)
fn recurse(nodes: &mut Vec<H256>) -> Vec<H256> {
    let mut processed_nodes = process_nodes_in_pairs(nodes);

    if processed_nodes.len() > 1 {
        processed_nodes = recurse(&mut processed_nodes);
    }

    return processed_nodes;
}

// Keys: N - number of nodes
// O(N/2)
fn process_nodes_in_pairs(nodes: &mut Vec<H256>) -> Vec<H256> {
    let mut processed_nodes: Vec<H256> = vec![];
    for index in 0..nodes.len() / 2 {
        let left_node = nodes[2 * index];
        let right_node = nodes[2 * index + 1];

        let temp: Vec<H256>;
        if left_node <= right_node {
            temp = vec![left_node, right_node];
        } else {
            temp = vec![right_node, left_node];
        }

        let node = decode_to_raw_nodes(temp);
        processed_nodes.push(H256::from_slice(&keccak_256(&node)));
    }

    if nodes.len() % 2 == 1 {
        processed_nodes.push(*nodes.last().unwrap());
    }

    return processed_nodes;
}

fn get_n_nodes(n: u32) -> Vec<Vec<u8>> {
    let mut nodes: Vec<Vec<u8>> = vec![];
    for number in 0..n {
        nodes.push(vec![(number % 256) as u8]);
    }
    return nodes;
}

fn to_array(bytes: &[u8]) -> [u8; 32] {
    let mut array = [0; 32];
    let bytes = &bytes[..];
    array.copy_from_slice(bytes);
    array
}

// Generates a merkle tree using `leaves_data` and returns the path from the specified `leaf_data` to the root
pub fn gnereate_merkle_path(leaf: &Vec<u8>, leaves_data: Vec<Vec<u8>>) -> Result<Vec<H256>, Error> {
    let mut merkle_path: Vec<H256> = vec![];

    if leaf.is_empty() {
        return Err(Error {
            code: ErrorCode::ServerError(2),
            message: "Error generating merkle path".into(),
            data: None,
        });
    }

    if leaves_data.is_empty() {
        return Err(Error {
            code: ErrorCode::ServerError(3),
            message: "Error generating merkle path".into(),
            data: None,
        });
    }

    let mut node_hash_in_leaf_branch = H256::from_slice(&keccak_256(leaf));
    let nodes_hashes = encode_to_hashed_nodes(leaves_data);

    recruse_for_path(
        &mut node_hash_in_leaf_branch,
        &nodes_hashes,
        &mut merkle_path,
    );

    return Ok(merkle_path);
}

fn recruse_for_path(
    node_hash_in_leaf_branch: &mut H256,
    nodes: &Vec<H256>,
    merkle_path: &mut Vec<H256>,
) -> Vec<H256> {
    let mut processed_nodes =
        process_nodes_in_pairs_for_path(node_hash_in_leaf_branch, nodes, merkle_path);

    if processed_nodes.len() > 1 {
        processed_nodes = recruse_for_path(node_hash_in_leaf_branch, &processed_nodes, merkle_path);
    }

    return processed_nodes;
}

fn process_nodes_in_pairs_for_path(
    node_hash_in_leaf_branch: &mut H256,
    nodes: &Vec<H256>,
    merkle_path: &mut Vec<H256>,
) -> Vec<H256> {
    let mut processed_nodes: Vec<H256> = vec![];
    for index in 0..nodes.len() / 2 {
        let left_node = nodes[2 * index];
        let right_node = nodes[2 * index + 1];

        let temp: Vec<H256>;
        if left_node <= right_node {
            temp = vec![left_node, right_node];
        } else {
            temp = vec![right_node, left_node];
        }

        let node = decode_to_raw_nodes(temp);
        let node_hash = H256::from_slice(&keccak_256(&node));
        if *node_hash_in_leaf_branch == left_node || *node_hash_in_leaf_branch == right_node {
            if *node_hash_in_leaf_branch == left_node {
                merkle_path.insert(0, right_node);
            } else {
                merkle_path.insert(0, left_node);
            }
            *node_hash_in_leaf_branch = node_hash;
        }
        processed_nodes.push(node_hash);
    }

    if nodes.len() % 2 == 1 {
        processed_nodes.push(*nodes.last().unwrap());
    }

    return processed_nodes;
}

// Convert raw nodes data to hashed nodes data
pub fn encode_to_hashed_nodes(nodes_data: Vec<Vec<u8>>) -> Vec<H256> {
    nodes_data
        .into_iter()
        .map(|data| H256::from_slice(&keccak_256(&data)))
        .collect::<Vec<H256>>()
}

// Convert hashed nodes data to raw nodes data
pub fn decode_to_raw_nodes(hashed_nodes: Vec<H256>) -> Vec<u8> {
    hashed_nodes
        .into_iter()
        .map(|h| h.to_fixed_bytes().to_vec())
        .flatten()
        .collect::<Vec<u8>>()
}

fn main() {}
