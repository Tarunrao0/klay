use anchor_lang::error_code;

#[error_code]

pub enum ParameterError {
    #[msg("Contract price cannot be 0")]
    InvalidContractPrice,

    #[msg("Expiration date cannot be 0")]
    InvalidExpirationDate,

    #[msg("Asset name cannot be empty")]
    InvalidAssetName
}