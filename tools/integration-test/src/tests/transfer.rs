use ibc_test_framework::ibc::denom::derive_ibc_denom;
use ibc_test_framework::prelude::*;
use ibc_test_framework::util::random::random_u64_range;

#[test]
fn test_ibc_transfer() -> Result<(), Error> {
    run_binary_channel_test(&IbcTransferTest)
}

#[cfg(feature = "manual")]
#[test]
fn test_slow_ibc_transfer() -> Result<(), Error> {
    // Purposely run the supervisor right after the chains are bootstrapped,
    // instead of the original behavior of running them after the channels
    // are bootstrapped. This will cause the IBC transfer to be relayed
    // very slowly.
    run_binary_node_test(&RunBinaryChainTest::new(&RunWithSupervisor::new(
        &RunBinaryConnectionTest::new(&RunBinaryChannelTest::new(&RepeatIbcTransferTest::<10>)),
    )))
}

/**
   Test that IBC token transfer can still work with a single
   chain that is connected to itself.
*/
#[test]
fn test_self_connected_ibc_transfer() -> Result<(), Error> {
    run_self_connected_binary_chain_test(&RunBinaryConnectionTest::new(&RunBinaryChannelTest::new(
        &RunWithSupervisor::new(&IbcTransferTest),
    )))
}

/**
   Run the IBC transfer test as an N-ary chain test case with SIZE=2.

   The work on N-ary chain is currently still work in progress, so we put
   this behind the "experimental" feature flag so that normal developers
   are not obligated to understand how this test works yet.
*/
#[test]
fn test_nary_ibc_transfer() -> Result<(), Error> {
    run_binary_as_nary_channel_test(&IbcTransferTest)
}

#[test]
fn test_self_connected_nary_ibc_transfer() -> Result<(), Error> {
    run_self_connected_nary_chain_test(&RunNaryConnectionTest::new(&RunNaryChannelTest::new(
        &RunBinaryAsNaryChannelTest::new(&IbcTransferTest),
    )))
}

pub struct IbcTransferTest;
pub struct RepeatIbcTransferTest<const COUNT: usize>;

impl TestOverrides for IbcTransferTest {}
impl<const COUNT: usize> TestOverrides for RepeatIbcTransferTest<COUNT> {}

impl<const COUNT: usize> BinaryChannelTest for RepeatIbcTransferTest<COUNT> {
    fn run<ChainA: ChainHandle, ChainB: ChainHandle>(
        &self,
        config: &TestConfig,
        relayer: RelayerDriver,
        chains: ConnectedChains<ChainA, ChainB>,
        channel: ConnectedChannel<ChainA, ChainB>,
    ) -> Result<(), Error> {
        for i in 0..COUNT {
            info!("Repeating IBC transfer test for the {}-th time", i);

            IbcTransferTest.run(config, relayer.clone(), chains.clone(), channel.clone())?;
        }

        Ok(())
    }
}

impl BinaryChannelTest for IbcTransferTest {
    fn run<ChainA: ChainHandle, ChainB: ChainHandle>(
        &self,
        _config: &TestConfig,
        _relayer: RelayerDriver,
        chains: ConnectedChains<ChainA, ChainB>,
        channel: ConnectedChannel<ChainA, ChainB>,
    ) -> Result<(), Error> {
        let denom_a = chains.node_a.denom();

        let wallet_a = chains.node_a.wallets().user1().cloned();
        let wallet_b = chains.node_b.wallets().user1().cloned();
        let wallet_c = chains.node_a.wallets().user2().cloned();

        let balance_a = chains
            .node_a
            .chain_driver()
            .query_balance(&wallet_a.address(), &denom_a)?;

        let a_to_b_amount = random_u64_range(1000, 5000);

        info!(
            "Sending IBC transfer from chain {} to chain {} with amount of {} {}",
            chains.chain_id_a(),
            chains.chain_id_b(),
            a_to_b_amount,
            denom_a
        );

        chains.node_a.chain_driver().transfer_token(
            &channel.port_a.as_ref(),
            &channel.channel_id_a.as_ref(),
            &wallet_a.address(),
            &wallet_b.address(),
            a_to_b_amount,
            &denom_a,
        )?;

        let denom_b = derive_ibc_denom(
            &channel.port_b.as_ref(),
            &channel.channel_id_b.as_ref(),
            &denom_a,
        )?;

        let balance_b = chains
            .node_b
            .chain_driver()
            .query_balance(&wallet_b.address(), &denom_b.as_ref())?;

        info!(
            "Waiting for user on chain B to receive IBC transferred amount of {} {}",
            a_to_b_amount, denom_b
        );

        chains.node_a.chain_driver().assert_eventual_wallet_amount(
            &wallet_a.address(),
            balance_a - a_to_b_amount,
            &denom_a,
        )?;

        chains.node_b.chain_driver().assert_eventual_wallet_amount(
            &wallet_b.address(),
            balance_b + a_to_b_amount,
            &denom_b.as_ref(),
        )?;

        info!(
            "successfully performed IBC transfer from chain {} to chain {}",
            chains.chain_id_a(),
            chains.chain_id_b(),
        );

        let balance_c = chains
            .node_a
            .chain_driver()
            .query_balance(&wallet_c.address(), &denom_a)?;

        let b_to_a_amount = random_u64_range(500, a_to_b_amount);

        info!(
            "Sending IBC transfer from chain {} to chain {} with amount of {} {}",
            chains.chain_id_b(),
            chains.chain_id_a(),
            b_to_a_amount,
            denom_b
        );

        chains.node_b.chain_driver().transfer_token(
            &channel.port_b.as_ref(),
            &channel.channel_id_b.as_ref(),
            &wallet_b.address(),
            &wallet_c.address(),
            b_to_a_amount,
            &denom_b.as_ref(),
        )?;

        info!(
            "Waiting for user on chain A to receive IBC transferred amount of {} {}",
            b_to_a_amount, denom_a
        );

        chains.node_b.chain_driver().assert_eventual_wallet_amount(
            &wallet_b.address(),
            balance_b + a_to_b_amount - b_to_a_amount,
            &denom_b.as_ref(),
        )?;

        chains.node_a.chain_driver().assert_eventual_wallet_amount(
            &wallet_c.address(),
            balance_c + b_to_a_amount,
            &denom_a,
        )?;

        info!(
            "successfully performed reverse IBC transfer from chain {} back to chain {}",
            chains.chain_id_b(),
            chains.chain_id_a(),
        );

        Ok(())
    }
}
