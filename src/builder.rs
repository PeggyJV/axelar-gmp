use crate::search::SearchGMPRequest;

#[derive(Default)]
pub struct SearchGMPRequestBuilder {
    tx_hash: Option<String>,
    from_time: Option<u64>,
    to_time: Option<u64>,
    size: Option<u32>,
    source_chain: Option<String>,
    source_address: Option<String>,
    destination_chain: Option<String>,
    contract_address: Option<String>,
}

impl SearchGMPRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tx_hash(mut self, tx_hash: String) -> Self {
        self.tx_hash = Some(tx_hash);
        self
    }

    pub fn from_time(mut self, from_time: u64) -> Self {
        self.from_time = Some(from_time);
        self
    }

    pub fn to_time(mut self, to_time: u64) -> Self {
        self.to_time = Some(to_time);
        self
    }

    pub fn size(mut self, size: u32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn source_chain(mut self, source_chain: String) -> Self {
        self.source_chain = Some(source_chain);
        self
    }

    pub fn source_address(mut self, source_address: String) -> Self {
        self.source_address = Some(source_address);
        self
    }

    pub fn destination_chain(mut self, destination_chain: String) -> Self {
        self.destination_chain = Some(destination_chain);
        self
    }

    pub fn contract_address(mut self, contract_address: String) -> Self {
        self.contract_address = Some(contract_address);
        self
    }

    pub fn build(self) -> SearchGMPRequest {
        SearchGMPRequest {
            tx_hash: self.tx_hash,
            from_time: self.from_time,
            to_time: self.to_time,
            size: self.size,
            source_chain: self.source_chain,
            source_address: self.source_address,
            destination_chain: self.destination_chain,
            contract_address: self.contract_address,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_default() {
        let request = SearchGMPRequestBuilder::default().build();
        assert_eq!(request.tx_hash, None);
        assert_eq!(request.from_time, None);
        assert_eq!(request.to_time, None);
        assert_eq!(request.size, None);
        assert_eq!(request.source_chain, None);
        assert_eq!(request.source_address, None);
        assert_eq!(request.destination_chain, None);
        assert_eq!(request.contract_address, None);
    }

    #[test]
    fn test_builder_with_all_fields() {
        let request = SearchGMPRequestBuilder::default()
            .tx_hash("0x123".to_string())
            .from_time(1000)
            .to_time(2000)
            .size(100)
            .source_chain("ethereum".to_string())
            .source_address("0xabc".to_string())
            .destination_chain("polygon".to_string())
            .contract_address("0xdef".to_string())
            .build();

        assert_eq!(request.tx_hash, Some("0x123".to_string()));
        assert_eq!(request.from_time, Some(1000));
        assert_eq!(request.to_time, Some(2000));
        assert_eq!(request.size, Some(100));
        assert_eq!(request.source_chain, Some("ethereum".to_string()));
        assert_eq!(request.source_address, Some("0xabc".to_string()));
        assert_eq!(request.destination_chain, Some("polygon".to_string()));
        assert_eq!(request.contract_address, Some("0xdef".to_string()));
    }

    #[test]
    fn test_builder_with_some_fields() {
        let request = SearchGMPRequestBuilder::default()
            .from_time(1000)
            .size(50)
            .destination_chain("avalanche".to_string())
            .build();

        assert_eq!(request.tx_hash, None);
        assert_eq!(request.from_time, Some(1000));
        assert_eq!(request.to_time, None);
        assert_eq!(request.size, Some(50));
        assert_eq!(request.source_chain, None);
        assert_eq!(request.source_address, None);
        assert_eq!(request.destination_chain, Some("avalanche".to_string()));
        assert_eq!(request.contract_address, None);
    }
}
