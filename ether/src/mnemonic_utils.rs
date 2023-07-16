use ethers::{
    signers::{coins_bip39::English, MnemonicBuilder, LocalWallet},
};
use eyre::Result;

pub fn from_mnemonic(mnemonic: &str, index: u32) -> Result<LocalWallet> {
    let wallet = MnemonicBuilder::<English>::default()
        .phrase(mnemonic)
        .index(index)?
        .build()?;

    Ok(wallet)
}


#[cfg(test)]
mod test {
    use ethers::{
        types::H160,
        signers::Signer,
    };
    use crate::mnemonic_utils::from_mnemonic;

    #[tokio::test]
    async fn test() {
        let wallet = from_mnemonic("fatal brass black survey crucial assist timber pattern execute sister illegal trade friend rival main", 0).unwrap();
        assert_eq!("0x7c47f87b1cbfef6e6a2159851ec16541b8f17536".parse::<H160>().unwrap(), wallet.address());
    }
}
