use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct CompactSize {
    pub value: u64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BitcoinError {
    InsufficientBytes,
    InvalidFormat,
}

impl CompactSize {
    pub fn new(value: u64) -> Self {
        // TODO: Construct a CompactSize from a u64 value
        Self { value }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Encode according to Bitcoin's CompactSize format:
        // [0x00–0xFC] => 1 byte
        // [0xFDxxxx] => 0xFD + u16 (2 bytes)
        // [0xFExxxxxxxx] => 0xFE + u32 (4 bytes)
        // [0xFFxxxxxxxxxxxxxxxx] => 0xFF + u64 (8 bytes)
        let v = self.value;
        if v <= 0xFC {
            vec![v as u8]
        } else if v <= 0xFFFF {
            let mut out = vec![0xFD];
            out.extend(&(v as u16).to_le_bytes());
            out
        } else if v <= 0xFFFF_FFFF {
            let mut out = vec![0xFE];
            out.extend(&(v as u32).to_le_bytes());
            out
        } else {
            let mut out = vec![0xFF];
            out.extend(&v.to_le_bytes());
            out
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // TODO: Decode CompactSize, returning value and number of bytes consumed.
        // First check if bytes is empty.
        // Check that enough bytes are available based on prefix.
         if bytes.is_empty() {
            return Err(BitcoinError::InsufficientBytes);
        }
        match bytes[0] {
    x @ 0x00..=0xFC => Ok((Self::new(x as u64), 1)),
    0xFD => {
        if bytes.len() < 3 {
            Err(BitcoinError::InsufficientBytes)
        } else {
            let val = u16::from_le_bytes([bytes[1], bytes[2]]) as u64;
            Ok((Self::new(val), 3))
        }
    }
    0xFE => {
        if bytes.len() < 5 {
            Err(BitcoinError::InsufficientBytes)
        } else {
            let val = u32::from_le_bytes(bytes[1..5].try_into().unwrap()) as u64;
            Ok((Self::new(val), 5))
        }
    }
    0xFF => {
        if bytes.len() < 9 {
            Err(BitcoinError::InsufficientBytes)
        } else {
            let val = u64::from_le_bytes(bytes[1..9].try_into().unwrap());
            Ok((Self::new(val), 9))
        }
    }
}
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Txid(pub [u8; 32]);

impl Serialize for Txid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // TODO: Serialize as a hex-encoded string (32 bytes => 64 hex characters)
         serializer.serialize_str(&hex::encode(self.0))
    }
}

impl<'de> Deserialize<'de> for Txid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // TODO: Parse hex string into 32-byte array
        // Use `hex::decode`, validate length = 32
        let s = String::deserialize(deserializer)?;
        let bytes = hex::decode(&s).map_err(serde::de::Error::custom)?;
        if bytes.len() != 32 {
            return Err(serde::de::Error::custom("Txid must be 32 bytes"));
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes);
        Ok(Txid(arr))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct OutPoint {
    pub txid: Txid,
    pub vout: u32,
}

impl OutPoint {
    pub fn new(txid: [u8; 32], vout: u32) -> Self {
        // TODO: Create an OutPoint from raw txid bytes and output index
         Self {
            txid: Txid(txid),
            vout,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Serialize as: txid (32 bytes) + vout (4 bytes, little-endian)
        let mut out = Vec::new();
        out.extend(&self.txid.0);
        out.extend(&self.vout.to_le_bytes());
        out
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // TODO: Deserialize 36 bytes: txid[0..32], vout[32..36]
        // Return error if insufficient bytes
        if bytes.len() < 36 {
            return Err(BitcoinError::InsufficientBytes);
        }
        let mut txid = [0u8; 32];
        txid.copy_from_slice(&bytes[0..32]);
        let vout = u32::from_le_bytes(bytes[32..36].try_into().unwrap());
        Ok((Self::new(txid, vout), 36))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Script {
    pub bytes: Vec<u8>,
}

impl Script {
    pub fn new(bytes: Vec<u8>) -> Self {
        // TODO: Simple constructor
         Self { bytes }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Prefix with CompactSize (length), then raw bytes
        let mut out = CompactSize::new(self.bytes.len() as u64).to_bytes();
        out.extend(&self.bytes);
        out
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // TODO: Parse CompactSize prefix, then read that many bytes
        // Return error if not enough bytes
        let (len_prefix, len_bytes) = CompactSize::from_bytes(bytes)?;
        let total_len = len_prefix.value as usize;
        if bytes.len() < len_bytes + total_len {
            return Err(BitcoinError::InsufficientBytes);
        }
        let data = bytes[len_bytes..len_bytes + total_len].to_vec();
        Ok((Self::new(data), len_bytes + total_len))
    }
}

impl Deref for Script {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        // TODO: Allow &Script to be used as &[u8]
         &self.bytes
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct TransactionInput {
    pub previous_output: OutPoint,
    pub script_sig: Script,
    pub sequence: u32,
}

impl TransactionInput {
    pub fn new(previous_output: OutPoint, script_sig: Script, sequence: u32) -> Self {
        // TODO: Basic constructor
        Self {
            previous_output,
            script_sig,
            sequence,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Serialize: OutPoint + Script (with CompactSize) + sequence (4 bytes LE)
        let mut out = self.previous_output.to_bytes();
        out.extend(self.script_sig.to_bytes());
        out.extend(&self.sequence.to_le_bytes());
        out
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // TODO: Deserialize in order:
        // - OutPoint (36 bytes)
        // - Script (with CompactSize)
        // - Sequence (4 bytes)
         let (prev_out, prev_len) = OutPoint::from_bytes(bytes)?;
        let (script, script_len) = Script::from_bytes(&bytes[prev_len..])?;
        if bytes.len() < prev_len + script_len + 4 {
            return Err(BitcoinError::InsufficientBytes);
        }
        let sequence =
            u32::from_le_bytes(bytes[prev_len + script_len..prev_len + script_len + 4].try_into().unwrap());
        Ok((
            Self::new(prev_out, script, sequence),
            prev_len + script_len + 4,
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct BitcoinTransaction {
    pub version: u32,
    pub inputs: Vec<TransactionInput>,
    pub lock_time: u32,
}

impl BitcoinTransaction {
    pub fn new(version: u32, inputs: Vec<TransactionInput>, lock_time: u32) -> Self {
        // TODO: Construct a transaction from parts
        Self {
            version,
            inputs,
            lock_time,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        // TODO: Format:
        // - version (4 bytes LE)
        // - CompactSize (number of inputs)
        // - each input serialized
        // - lock_time (4 bytes LE)
        let mut out = Vec::new();
        out.extend(&self.version.to_le_bytes());
        out.extend(CompactSize::new(self.inputs.len() as u64).to_bytes());
        for input in &self.inputs {
            out.extend(input.to_bytes());
        }
        out.extend(&self.lock_time.to_le_bytes());
        out
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Self, usize), BitcoinError> {
        // TODO: Read version, CompactSize for input count
        // Parse inputs one by one
        // Read final 4 bytes for lock_time
        if bytes.len() < 4 {
            return Err(BitcoinError::InsufficientBytes);
        }

        let version = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let (input_count, mut offset) = CompactSize::from_bytes(&bytes[4..])?;
        offset += 4; // adjust for version prefix

        let mut inputs = Vec::new();
        let mut cursor = offset;
        for _ in 0..input_count.value {
            let (input, used) = TransactionInput::from_bytes(&bytes[cursor..])?;
            inputs.push(input);
            cursor += used;
        }

        if bytes.len() < cursor + 4 {
            return Err(BitcoinError::InsufficientBytes);
        }

        let lock_time = u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap());
        Ok((
            Self::new(version, inputs, lock_time),
            cursor + 4,
        ))
    }
}

impl fmt::Display for BitcoinTransaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Format a user-friendly string showing version, inputs, lock_time
        // Display scriptSig length and bytes, and previous output info
        writeln!(f, "Version: {}", self.version)?;
        for input in &self.inputs {
            writeln!(f, "Previous Output Vout: {}", input.previous_output.vout)?;
            writeln!(f, "ScriptSig Length: {}", input.script_sig.bytes.len())?;
            writeln!(f, "ScriptSig: {}", hex::encode(&input.script_sig.bytes))?;
            writeln!(f, "Sequence: {}", input.sequence)?;
        }
        writeln!(f, "Lock Time: {}", self.lock_time)
    }
}
