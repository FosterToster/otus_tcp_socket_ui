use shtp::SHTPClient;

pub struct ConnectedView{
    client: SHTPClient
}

impl ConnectedView {
    pub fn new(client: SHTPClient) -> Self {
        Self { client }
    }
}