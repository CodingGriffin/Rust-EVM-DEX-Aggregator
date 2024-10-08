use amms::amm::{
    uniswap_v2::{UniswapV2Pool, IUNISWAPV2PAIR_ABI},
    uniswap_v3::{UniswapV3Pool, IUNISWAPV3POOL_ABI},
};
use ethers::{
    abi::Token,
    providers::{Http, Provider},
    types::{H160, U256},
};
use hex::FromHex;
use std::{str::FromStr, sync::Arc};

pub async fn try_swap_calldata(rpc_endpoint: String) -> eyre::Result<()> {
    let middleware = Arc::new(Provider::<Http>::try_from(rpc_endpoint)?);

    // // Initialize the pool
    // let pool_address = H160::from_str("0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc")?;
    // let pool = UniswapV2Pool::new_from_address(pool_address, 300, middleware.clone()).await?;

    // Generate the swap calldata
    let to_address = H160::from_str("83e082f589ba40d198d924c5a31788c1fc414e00")?;
    // let swap_calldata = pool.swap_calldata(U256::from(10000), U256::zero(), to_address, vec![]);
    let amount_0_out = U256::from(10000);
    let amount_1_out = U256::zero();
    let to = to_address;
    let calldata = vec![];

    let input_tokens = vec![
        Token::Uint(amount_0_out),
        Token::Uint(amount_1_out),
        Token::Address(to),
        Token::Bytes(calldata),
    ];

    let swap_calldata = IUNISWAPV2PAIR_ABI
        .function("swap")?
        .encode_input(&input_tokens);

    match swap_calldata {
        Ok(calldata) => {
            let hex_string = hex::encode(calldata.clone());

            println!("Swap calldata: {:?}", hex_string);
            let decoded_data = IUNISWAPV2PAIR_ABI
                .function("swap")?
                .decode_input(&calldata.clone()[4..]);
            println!("Decoded data: {:?}", decoded_data);
        }
        Err(e) => {}
    }

    let swap_calldata_mockup = "022c0d9f0000000000000000000000000000000000000000000000017dd6957f952f019800000000000000000000000000000000000000000000000000000000000000000000000000000000000000002c11a5a75048d0d1e1598aec4b67a11f1e85129300000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000040000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2000000000000000000000000000000000000000000000000000000000000012c";
    println!("Swap calldata: {:?}", swap_calldata_mockup);
    match Vec::from_hex(&swap_calldata_mockup) {
        Ok(bytes) => {
            let decoded_data = IUNISWAPV2PAIR_ABI
                .function("swap")?
                .decode_input(&bytes.clone()[4..]);
            println!("Decoded swap data: {:?}", decoded_data);
        }
        Err(e) => {}
    }

    let v3_swap_calldata_mockup = "128acb080000000000000000000000003ad638fb85a16c3401a9bfc9b961382904daa8350000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000217925387e3e0000000000000000000000000000fffd8963efd1fc6a506488495d951d5263988d2500000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000020000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2";
    println!("V3 swap calldata: {:?}", v3_swap_calldata_mockup);
    match Vec::from_hex(&v3_swap_calldata_mockup) {
        Ok(bytes) => {
            let decoded_data = IUNISWAPV3POOL_ABI
                .function("swap")?
                .decode_input(&bytes.clone()[4..]);
            println!("Decoded swap data: {:?}", decoded_data);
        }
        Err(e) => {}
    }

    let transfer_calldata_mockup = "a9059cbb00000000000000000000000031372afe90e7900bd4bf682fe9ba143c5206afe4000000000000000000000000000000000000000000000000000000000e7cf83d";
    println!("Transfer calldata: {:?}", transfer_calldata_mockup);
    match Vec::from_hex(&transfer_calldata_mockup) {
        Ok(bytes) => {
            // let decoded_data = IUNISWAPV2PAIR_ABI
            //     .function("transfer")?
            //     .decode_input(&bytes.clone()[4..]);
            let amount = U256::from_str_radix(
                &transfer_calldata_mockup[transfer_calldata_mockup.len() - 64..],
                16,
            )
            .unwrap();
            println!("Decoded transfer data: {:?}", amount);
        }
        Err(e) => {}
    }

    Ok(())
}
