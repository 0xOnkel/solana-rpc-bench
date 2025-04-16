use solana_client::client_error::ClientError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("internal test error")]
    InternalTestError,
    #[error("rpc client error")]
    ClientError(#[from] ClientError),
}
