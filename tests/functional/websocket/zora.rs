use {
    super::check_if_rpc_is_responding_correctly_for_supported_chain,
    crate::context::ServerContext,
    test_context::test_context,
};

#[test_context(ServerContext)]
#[tokio::test]
async fn zora_websocket_provider(ctx: &mut ServerContext) {
    // Zora mainnet
    check_if_rpc_is_responding_correctly_for_supported_chain(ctx, "eip155:7777777", "0x76adf1")
        .await;
}
