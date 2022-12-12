# Review Report of the Rust Program for the Merkle Tree Demo

## Audit

- Dependency and unsafe check \
  Has no vulnerabilities for packages and unsafe codes
- Unused structs and variables \
  `Duration` ([here](https://github.com/black-wyvern-dev/merkle_tree_audit/blob/47477f619f1da23e8b706555160a73edfd8adba9/main.rs#L29)) :- seems import to measurement the test running duration but not implement anywhere. \
  `to_array(_size: u8, bytes: &[u8])` ([here](https://github.com/black-wyvern-dev/merkle_tree_audit/blob/47477f619f1da23e8b706555160a73edfd8adba9/main.rs#L372)) :- the `_size` param is not used too. Seems define this param to customize the result array length. \
  But I don't think define such length dynamically because we are using the SHA 256 algorithm only.

- Respect naming conventions \
  Suggest to name the `to_array`([here](https://github.com/black-wyvern-dev/merkle_tree_audit/blob/47477f619f1da23e8b706555160a73edfd8adba9/main.rs#L372)) func more understandable and definitely. \
  All variables and methods should be `snak_cased` names. \
  `let longTest` ([here](https://github.com/black-wyvern-dev/merkle_tree_audit/blob/47477f619f1da23e8b706555160a73edfd8adba9/main.rs#L72)) should be named as `long_test` in `test_generate_root_3` test. \
  f.e. `to_u8_array`, `bytes_to_u8_array`...
- Need to comment the functionality of all the functions include for testing too \
  f.e. `get_n_nodes`, `to_array`
- Arithmetic Overflow \
  Has no operations may occur overflow

## Improve ([commit](https://github.com/black-wyvern-dev/merkle_tree_audit/commit/ed8fb06401e8cd5cd0cac04b75419bbec161e544#diff-9177877afee1a3106b0dd6843541f808678413a3a09e942ec8a13c877db1e3f1R29))

- Remove unused elements
![image](https://user-images.githubusercontent.com/80482169/188129801-d8a55946-ca09-4442-b0c0-469d453fd4ca.png)
- Prettier code format
- Create empty main function for safety build ([here](https://github.com/black-wyvern-dev/merkle_tree_audit/blob/b2b26edd03a047c1fb4f86d50083e01bef37a903/main.rs#L451))
- Fix bugs for tests
- Made utils for double used code snippets
![image](https://user-images.githubusercontent.com/80482169/188130367-8552c7d7-adc0-4c1b-bd26-edc49ec7cce0.png)

- Leave simple comments for the util functions \
[encode_to_hashed_nodes](https://github.com/black-wyvern-dev/merkle_tree_audit/blob/b2b26edd03a047c1fb4f86d50083e01bef37a903/main.rs#L434) \
[decode_to_raw_nodes](https://github.com/black-wyvern-dev/merkle_tree_audit/blob/b2b26edd03a047c1fb4f86d50083e01bef37a903/main.rs#L443)
