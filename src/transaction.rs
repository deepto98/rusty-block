use super::*;
pub struct Output {
    pub to_addr: Address,
    pub value: u64,
}

impl Hashable for Output {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.to_addr.as_bytes());
        bytes.extend(&u64_bytes(&self.value));

        bytes
    }
}

pub struct Transaction {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>,
}

impl Hashable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        // TODO : Learn flat_map in depth
        bytes.extend(
            self.inputs
                .iter()
                .flat_map(|input| input.bytes()) //Lambda function - flattens vectors
                .collect::<Vec<u8>>(),
        );

        bytes.extend(
            self.outputs
                .iter()
                .flat_map(|output| output.bytes()) //Lambda function - flattens vectors
                .collect::<Vec<u8>>(),
        );
        bytes
    }
}
