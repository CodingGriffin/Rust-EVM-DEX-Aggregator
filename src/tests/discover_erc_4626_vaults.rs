use amms::discovery;
use ethers::providers::{Http, Provider};
use std::sync::Arc;

pub async fn try_discorver_erc_4626_vaults(rpc_endpoint: String) -> eyre::Result<()> {
    let provider = Arc::new(Provider::<Http>::try_from(rpc_endpoint)?);

    //discover vaults
    let vaults = discovery::erc_4626::discover_erc_4626_vaults(provider, 30000).await?;

    println!("Vaults: {:?}", vaults);

    Ok(())
}
