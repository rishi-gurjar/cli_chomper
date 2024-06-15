
## cli_chomper
- local, secure password storage, encrypted using AES-128, in Rust.
- encryption is fully abstracted for the user. (AES redone in Rust using [FIPS-197](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.197-upd1.pdf))
- CRUD operations in `src/main.rs` and AES implementation in `src/aes.rs`

#### INSTALLATION
after cloning, in root, run `cargo install --path .` to install to your $HOME path

#### OPTIONS
| Command | Description |
| :------ | :--- |
|`-n, --new <NEW>`   |     Create new password |
|`-u, --url <URL>`   |     Url for a new password |
|`-v, --view <VIEW>`   |   View all passwords [possible values: true, false] |
|`-d, --delete <DELETE>` | Delete a password | 
|`-h, --help`       |      Print help |
|`-V, --version`   |       Print version |

#### EXAMPLE USES
| Command | Description |
| :------ | :--- |
| `cli_chomper -n securepassword -u apple.com` | Creates password "securepassword" for the website "apple.com" |
| `cli_chomper -v true` | Displays all passwords |
| `cli_chomper -d apple.com` | Deletes the password for website "apple.com" |

---


#### Research notes - no salting done in this implementation since it is pure encoding

every plaintext -> ciphertext through the following function:

1. TRANSLATION: plaintext (String) -> UTF-8 Encoded binary
2. CONCATENATION: encoded plaintext (Int) + 64-bit binary key
3. ENCRYPTION: concatenated binary is concatenated with a salt and ran through a cipher, encrypted with a symmetric key (128, 192, or 256 bits). key is stored as plaintext on local device

salts: (one is chosen)
- climate: no2 + co concentrations at the lat and long of the user (https://openweathermap.org/api/air-pollution)
- timestamp: nanosecond-scale time of the password submission
- classic: conventional random number generator
- sessionid: CPU load/other sys stats

##### ENCRYPTION
Encrypted using a custom substitution–permutation network (derived off AES), with obfuscation, confusion, and diffusion through several layers, with the block size kept at 128 bits:

1. KeyExpansion
2. Initial round key addition ??
3. Layer Obfuscation

###### KeyExpansion
1. Key is split up into 4 byte (32bit) increments called "words" (128bit key = 4 words (32 bits each))
2. The first four words are the same as the original key, the remaining number of words are determined by the key length (table below)
3. The rest of the words are generated using a combination of previous words and transformations such as RotWord, SubWord, and XOR with round constants (Rcon) **FILL IN


| Key Length | # of round keys | # of words | # of total layers |
| :------ | :--- | :--- | :--- |
| 128-bit | 11 | 44 | 10 |
| 192-bit | 13 | 52 | 12 |
| 256-bit | 15 | 60 | 14 |

Layer Obfuscation:
1. substitution-boxes (a layers)
2. permutation-boxes (b layers)
3. final layer (determined by size of key)

a + b < total layers
