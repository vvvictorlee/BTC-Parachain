/// Mocking the test environment
use crate::{Error, GenesisConfig, Module, Trait};
use frame_support::{
    assert_ok, impl_outer_event, impl_outer_origin, parameter_types,
    weights::{
        constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight},
        Weight,
    },
};
use pallet_balances as balances;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    Perbill,
};

use mocktopus::mocking::clear_mocks;

impl_outer_origin! {
    pub enum Origin for Test {}
}

mod test_events {
    pub use crate::Event;
}

impl_outer_event! {
    pub enum TestEvent for Test {
        frame_system<T>,
        test_events<T>,
        balances<T>,
        vault_registry<T>,
        collateral<T>,
        btc_relay,
        treasury<T>,
        exchange_rate_oracle<T>,
        security,
    }
}

// For testing the pallet, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of pallets we want to use.

pub type AccountId = u64;
pub type Balance = u64;
pub type BlockNumber = u64;

#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: Weight = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::one();
}

impl frame_system::Trait for Test {
    type AccountId = AccountId;
    type Call = ();
    type Lookup = IdentityLookup<Self::AccountId>;
    type Index = u64;
    type BlockNumber = BlockNumber;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type Header = Header;
    type Event = TestEvent;
    type Origin = Origin;
    type BlockHashCount = BlockHashCount;
    type MaximumBlockWeight = MaximumBlockWeight;
    type MaximumBlockLength = MaximumBlockLength;
    type AvailableBlockRatio = AvailableBlockRatio;
    type BlockExecutionWeight = BlockExecutionWeight;
    type DbWeight = RocksDbWeight;
    type ExtrinsicBaseWeight = ExtrinsicBaseWeight;
    type Version = ();
    type PalletInfo = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type AccountData = pallet_balances::AccountData<u64>;
    type BaseCallFilter = ();
    type MaximumExtrinsicWeight = MaximumBlockWeight;
    type SystemWeightInfo = ();
}

parameter_types! {
    pub const ExistentialDeposit: u64 = 1;
    pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Trait for Test {
    type MaxLocks = MaxLocks;
    type Balance = Balance;
    type Event = TestEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
}

impl vault_registry::Trait for Test {
    type Event = TestEvent;
    type RandomnessSource = pallet_randomness_collective_flip::Module<Test>;
    type WeightInfo = ();
}

impl collateral::Trait for Test {
    type Event = TestEvent;
    type DOT = Balances;
}

impl btc_relay::Trait for Test {
    type Event = TestEvent;
    type WeightInfo = ();
}

impl security::Trait for Test {
    type Event = TestEvent;
}

impl treasury::Trait for Test {
    type PolkaBTC = Balances;
    type Event = TestEvent;
}

parameter_types! {
    pub const MinimumPeriod: u64 = 5;
}

impl timestamp::Trait for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

impl exchange_rate_oracle::Trait for Test {
    type Event = TestEvent;
    type WeightInfo = ();
}

impl Trait for Test {
    type Event = TestEvent;
    type WeightInfo = ();
}

pub type TestError = Error<Test>;
pub type SecurityError = security::Error<Test>;
pub type VaultRegistryError = vault_registry::Error<Test>;

pub type System = frame_system::Module<Test>;
pub type Balances = pallet_balances::Module<Test>;
pub type Issue = Module<Test>;

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const CAROL: AccountId = 3;

pub const ALICE_BALANCE: u64 = 1_000_000;
pub const BOB_BALANCE: u64 = 1_000_000;
pub const CAROL_BALANCE: u64 = 1_000_000;

pub struct ExtBuilder;

impl ExtBuilder {
    pub fn build_with(conf: pallet_balances::GenesisConfig<Test>) -> sp_io::TestExternalities {
        let mut storage = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        conf.assimilate_storage(&mut storage).unwrap();

        GenesisConfig::<Test> {
            issue_griefing_collateral: 10,
            issue_period: 10,
        }
        .assimilate_storage(&mut storage)
        .unwrap();

        storage.into()
    }

    pub fn build() -> sp_io::TestExternalities {
        ExtBuilder::build_with(pallet_balances::GenesisConfig::<Test> {
            balances: vec![
                (ALICE, ALICE_BALANCE),
                (BOB, BOB_BALANCE),
                (CAROL, CAROL_BALANCE),
            ],
        })
    }
}

pub fn run_test<T>(test: T) -> ()
where
    T: FnOnce() -> (),
{
    clear_mocks();
    ExtBuilder::build().execute_with(|| {
        assert_ok!(<exchange_rate_oracle::Module<Test>>::_set_exchange_rate(1));
        System::set_block_number(1);
        test();
    });
}
