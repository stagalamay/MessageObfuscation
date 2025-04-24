# Message Obfuscation Using Esolang and Multi-layered Encoding

## Objective  
This project implements a **message obfuscation system** that encodes readable text into a highly obfuscated format using a custom esolang and multiple transformation layers.

### Steps involved:
1. Message Translation to Esolang (`diropql`)
2. Multi-layered Obfuscation (BWT → MTF → RLE → Base85)
3. Metadata Handling for Correct Reversal
4. Message Recovery (Full Deobfuscation)

---

## Technology  
**Language:** Rust  
**Encoding techniques used:**
- Burrows-Wheeler Transform (BWT)
- Move-To-Front Encoding (MTF)
- Run-Length Encoding (RLE)
- Base85 Encoding  
**Obfuscation Language:** `diropql` (custom esolang)

---

## Step 1: Message Translation via `diropql`

Text input is first compiled into a minimalist esolang that mimics Brainf\*\*\* behavior, using the alphabet `diropql`. Each character in the message is encoded into `diropql` instructions that increment memory cells and print characters.

---

## Step 2: Message Obfuscation

The `diropql` output is then passed through a series of encoding layers:
1. **BWT (Burrows-Wheeler Transform):** Reorders the string for improved compression/structure
2. **MTF (Move-To-Front):** Re-encodes data relative to a fixed alphabet (`diropql`)
3. **RLE (Run-Length Encoding):** Compresses repeated elements
4. **Base85:** Converts bytes into printable ASCII for storage/transmission

Metadata (e.g., BWT index, program length, and padding) is prepended and the full output is base85 encoded to finalize the obfuscation.

---

## Step 3: Message Deobfuscation

The reverse process:
1. Extract metadata from the base85-decoded blob
2. Decode through RLE → MTF → BWT (using saved index)
3. Interpret the resulting `diropql` to recover the original message

---

## Example

```rust
let message = "hello";
let encoded = write_diropqlz(&message.to_string());
let decoded = read_diropqlz(&encoded);
assert_eq!(decoded, message);
