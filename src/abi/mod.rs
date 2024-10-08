use ethers::prelude::abigen;

abigen!(

    IConveyorExecutor,
    r#"[
        function checkIn() external
        function lastCheckIn(address addr) external view returns (uint256)
    ]"#;

    IUniswapV2Factory,
    r#"[
        function getPair(address tokenA, address tokenB) external view returns (address pair)
        event PairCreated(address indexed token0, address indexed token1, address pair, uint256)
    ]"#;

    IUniswapV2Pair,
    r#"[
        function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)
        function token0() external view returns (address)
        function token1() external view returns (address)
        event Sync(uint112 reserve0, uint112 reserve1)
    ]"#;

    IUniswapV3Factory,
    r#"[
        function getPool(address tokenA, address tokenB, uint24 fee) external view returns (address pool)
        event PoolCreated(address indexed token0, address indexed token1, uint24 indexed fee, int24 tickSpacing, address pool)
    ]"#;

    IUniswapV3Pool,
    r#"[
        function token0() external view returns (address)
        function token1() external view returns (address)
        function liquidity() external view returns (uint128)
        function slot0() external view returns (uint160, int24, uint16, uint16, uint16, uint8, bool)
        function fee() external view returns (uint24)
        event Swap(address sender, address recipient, int256 amount0, int256 amount1, uint160 sqrtPriceX96, uint128 liquidity, int24 tick)
        ]"#;

    IUniswapV3Quoter,
    r#"[
        function quoteExactInputSingle(address tokenIn, address tokenOut,uint24 fee, uint256 amountIn, uint160 sqrtPriceLimitX96) external returns (uint256 amountOut)
        ]"#;

    IErc20,
    r#"[
        function balanceOf(address account) external view returns (uint256)
        function decimals() external view returns (uint8)
        function transfer(address to, uint256 amount) external returns (bool)

    ]"#;


);
