use super::*;
use std::collections::HashSet;

/// The Output structure
#[derive(Clone)]
pub struct Output {
    pub to_addr: String,
    pub value: u64,
}

/// Implemets the Hash trait in order to hash the Output struct
impl Hash for Output {
    fn bytes (&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(self.to_addr.as_bytes());
        bytes.extend(&u64_bytes(&self.value));

        bytes
    }
}

/// The Transaction structure
pub struct Transaction {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>,
}

impl Transaction {
    /// Gets the sum of all the inputs
    pub fn input_value (&self) -> u64 {
        self.inputs
            .iter()
            .map(|input| input.value)
            .sum()
    }

    /// Gets the sum of all the outputs
    pub fn output_value (&self) -> u64 {
        self.outputs
            .iter()
            .map(|output| output.value)
            .sum()
    }

    /// Collects the hash of all the inputs into a HashSet with Vec<u8>
    pub fn input_hashes (&self) -> HashSet<Vec<u8>> {
        self.inputs
            .iter()
            .map(|input| input.hash())
            .collect::<HashSet<Vec<u8>>>()
    }

    /// Collects the hash of all the outputs into a HashSet with Vec<u8>
    pub fn output_hashes (&self) -> HashSet<Vec<u8>> {
        self.outputs
            .iter()
            .map(|output| output.hash())
            .collect::<HashSet<Vec<u8>>>()
    }

    /// Checks if the transaction is a coinbase transaction
    pub fn is_coinbase (&self) -> bool {
        self.inputs.len() == 0
    }
}

impl Hash for Transaction {
    /// Converts the transaction into bytes
    fn bytes (&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(
            self.inputs
                .iter()
                .flat_map(|input| input.bytes())
                .collect::<Vec<u8>>()
        );

        bytes.extend(
            self.outputs
                .iter()
                .flat_map(|output| output.bytes())
                .collect::<Vec<u8>>()
        );

        bytes
    }
}