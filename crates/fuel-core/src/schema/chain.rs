use crate::{
    fuel_core_graphql_api::{
        api_service::ConsensusProvider,
        database::ReadView,
    },
    graphql_api::Config,
    query::{
        BlockQueryData,
        ChainQueryData,
    },
    schema::{
        block::Block,
        scalars::{
            Address,
            AssetId,
            U16,
            U32,
            U64,
            U8,
        },
    },
};
use async_graphql::{
    Context,
    Object,
    Union,
};
use fuel_core_types::{
    fuel_tx,
    fuel_tx::GasCostsValues,
};
use std::{
    ops::Deref,
    sync::Arc,
};

pub struct ChainInfo;
pub struct ConsensusParameters(Arc<fuel_tx::ConsensusParameters>);
pub struct TxParameters(fuel_tx::TxParameters);
pub struct PredicateParameters(fuel_tx::PredicateParameters);
pub struct ScriptParameters(fuel_tx::ScriptParameters);
pub struct ContractParameters(fuel_tx::ContractParameters);
pub struct FeeParameters(fuel_tx::FeeParameters);

pub struct GasCosts(fuel_tx::GasCosts);

pub struct Version(u8);

#[derive(Union)]
pub enum GasCostsVersion {
    V1(Version),
}

#[derive(Union)]
pub enum FeeParametersVersion {
    V1(Version),
}

#[derive(Union)]
pub enum ContractParametersVersion {
    V1(Version),
}

#[derive(Union)]
pub enum ScriptParametersVersion {
    V1(Version),
}

#[derive(Union)]
pub enum PredicateParametersVersion {
    V1(Version),
}

#[derive(Union)]
pub enum TxParametersVersion {
    V1(Version),
}

#[derive(Union)]
pub enum ConsensusParametersVersion {
    V1(Version),
}

#[derive(Union)]
pub enum DependentCost {
    LightOperation(LightOperation),
    HeavyOperation(HeavyOperation),
}

pub struct LightOperation {
    base: u64,
    units_per_gas: u64,
}

pub struct HeavyOperation {
    base: u64,
    gas_per_unit: u64,
}

impl From<fuel_tx::DependentCost> for DependentCost {
    fn from(value: fuel_tx::DependentCost) -> Self {
        match value {
            fuel_tx::DependentCost::LightOperation {
                base,
                units_per_gas,
            } => DependentCost::LightOperation(LightOperation {
                base,
                units_per_gas,
            }),
            fuel_tx::DependentCost::HeavyOperation { base, gas_per_unit } => {
                DependentCost::HeavyOperation(HeavyOperation { base, gas_per_unit })
            }
        }
    }
}

#[Object]
impl Version {
    async fn value(&self) -> U8 {
        self.0.into()
    }
}

#[Object]
impl ConsensusParameters {
    async fn version(&self) -> ConsensusParametersVersion {
        match self.0.as_ref() {
            fuel_tx::ConsensusParameters::V1(_) => {
                ConsensusParametersVersion::V1(Version(1))
            }
        }
    }

    async fn tx_params(&self, ctx: &Context<'_>) -> async_graphql::Result<TxParameters> {
        let params = ctx
            .data_unchecked::<ConsensusProvider>()
            .latest_consensus_params();

        Ok(TxParameters(params.tx_params().to_owned()))
    }

    async fn predicate_params(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<PredicateParameters> {
        let params = ctx
            .data_unchecked::<ConsensusProvider>()
            .latest_consensus_params();

        Ok(PredicateParameters(params.predicate_params().to_owned()))
    }

    async fn script_params(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<ScriptParameters> {
        let params = ctx
            .data_unchecked::<ConsensusProvider>()
            .latest_consensus_params();

        Ok(ScriptParameters(params.script_params().to_owned()))
    }

    async fn contract_params(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<ContractParameters> {
        let params = ctx
            .data_unchecked::<ConsensusProvider>()
            .latest_consensus_params();

        Ok(ContractParameters(params.contract_params().to_owned()))
    }

    async fn fee_params(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<FeeParameters> {
        let params = ctx
            .data_unchecked::<ConsensusProvider>()
            .latest_consensus_params();

        Ok(FeeParameters(params.fee_params().to_owned()))
    }

    async fn base_asset_id(&self, ctx: &Context<'_>) -> async_graphql::Result<AssetId> {
        let params = ctx
            .data_unchecked::<ConsensusProvider>()
            .latest_consensus_params();

        Ok(AssetId(*params.base_asset_id()))
    }

    async fn block_gas_limit(&self, ctx: &Context<'_>) -> async_graphql::Result<U64> {
        let params = ctx
            .data_unchecked::<ConsensusProvider>()
            .latest_consensus_params();

        Ok(params.block_gas_limit().into())
    }

    async fn chain_id(&self) -> U64 {
        (*self.0.chain_id()).into()
    }

    async fn gas_costs(&self, ctx: &Context<'_>) -> async_graphql::Result<GasCosts> {
        let params = ctx
            .data_unchecked::<ConsensusProvider>()
            .latest_consensus_params();

        Ok(GasCosts(params.gas_costs().clone()))
    }

    async fn privileged_address(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Address> {
        let params = ctx
            .data_unchecked::<ConsensusProvider>()
            .latest_consensus_params();

        Ok(Address(*params.privileged_address()))
    }
}

#[Object]
impl TxParameters {
    async fn version(&self) -> TxParametersVersion {
        match self.0 {
            fuel_tx::TxParameters::V1(_) => TxParametersVersion::V1(Version(1)),
        }
    }

    async fn max_inputs(&self) -> U16 {
        self.0.max_inputs().into()
    }

    async fn max_outputs(&self) -> U16 {
        self.0.max_outputs().into()
    }

    async fn max_witnesses(&self) -> U32 {
        self.0.max_witnesses().into()
    }

    async fn max_gas_per_tx(&self) -> U64 {
        self.0.max_gas_per_tx().into()
    }

    async fn max_size(&self) -> U64 {
        self.0.max_size().into()
    }

    async fn max_bytecode_subsections(&self) -> U16 {
        self.0.max_bytecode_subsections().into()
    }
}

#[Object]
impl PredicateParameters {
    async fn version(&self) -> PredicateParametersVersion {
        match self.0 {
            fuel_tx::PredicateParameters::V1(_) => {
                PredicateParametersVersion::V1(Version(1))
            }
        }
    }

    async fn max_predicate_length(&self) -> U64 {
        self.0.max_predicate_length().into()
    }

    async fn max_predicate_data_length(&self) -> U64 {
        self.0.max_predicate_data_length().into()
    }

    async fn max_gas_per_predicate(&self) -> U64 {
        self.0.max_gas_per_predicate().into()
    }

    async fn max_message_data_length(&self) -> U64 {
        self.0.max_message_data_length().into()
    }
}

#[Object]
impl ScriptParameters {
    async fn version(&self) -> ScriptParametersVersion {
        match self.0 {
            fuel_tx::ScriptParameters::V1(_) => ScriptParametersVersion::V1(Version(1)),
        }
    }

    async fn max_script_length(&self) -> U64 {
        self.0.max_script_length().into()
    }

    async fn max_script_data_length(&self) -> U64 {
        self.0.max_script_data_length().into()
    }
}

#[Object]
impl ContractParameters {
    async fn version(&self) -> ContractParametersVersion {
        match self.0 {
            fuel_tx::ContractParameters::V1(_) => {
                ContractParametersVersion::V1(Version(1))
            }
        }
    }

    async fn contract_max_size(&self) -> U64 {
        self.0.contract_max_size().into()
    }

    async fn max_storage_slots(&self) -> U64 {
        self.0.max_storage_slots().into()
    }
}

#[Object]
impl FeeParameters {
    async fn version(&self) -> FeeParametersVersion {
        match self.0 {
            fuel_tx::FeeParameters::V1(_) => FeeParametersVersion::V1(Version(1)),
        }
    }

    async fn gas_price_factor(&self) -> U64 {
        self.0.gas_price_factor().into()
    }

    async fn gas_per_byte(&self) -> U64 {
        self.0.gas_per_byte().into()
    }
}

#[Object]
impl GasCosts {
    async fn version(&self) -> GasCostsVersion {
        match self.0.deref() {
            GasCostsValues::V1(_) => GasCostsVersion::V1(Version(1)),
        }
    }

    async fn add(&self) -> U64 {
        self.0.add().into()
    }

    async fn addi(&self) -> U64 {
        self.0.addi().into()
    }

    async fn aloc(&self) -> U64 {
        self.0.aloc().into()
    }

    async fn and(&self) -> U64 {
        self.0.and().into()
    }

    async fn andi(&self) -> U64 {
        self.0.andi().into()
    }

    async fn bal(&self) -> U64 {
        self.0.bal().into()
    }

    async fn bhei(&self) -> U64 {
        self.0.bhei().into()
    }

    async fn bhsh(&self) -> U64 {
        self.0.bhsh().into()
    }

    async fn burn(&self) -> U64 {
        self.0.burn().into()
    }

    async fn cb(&self) -> U64 {
        self.0.cb().into()
    }

    async fn cfei(&self) -> U64 {
        self.0.cfei().into()
    }

    async fn cfsi(&self) -> U64 {
        self.0.cfsi().into()
    }

    async fn div(&self) -> U64 {
        self.0.div().into()
    }

    async fn divi(&self) -> U64 {
        self.0.divi().into()
    }

    async fn ecr1(&self) -> U64 {
        self.0.ecr1().into()
    }

    async fn eck1(&self) -> U64 {
        self.0.eck1().into()
    }

    async fn ed19(&self) -> U64 {
        self.0.ed19().into()
    }

    async fn eq(&self) -> U64 {
        self.0.eq_().into()
    }

    async fn exp(&self) -> U64 {
        self.0.exp().into()
    }

    async fn expi(&self) -> U64 {
        self.0.expi().into()
    }

    async fn flag(&self) -> U64 {
        self.0.flag().into()
    }

    async fn gm(&self) -> U64 {
        self.0.gm().into()
    }

    async fn gt(&self) -> U64 {
        self.0.gt().into()
    }

    async fn gtf(&self) -> U64 {
        self.0.gtf().into()
    }

    async fn ji(&self) -> U64 {
        self.0.ji().into()
    }

    async fn jmp(&self) -> U64 {
        self.0.jmp().into()
    }

    async fn jne(&self) -> U64 {
        self.0.jne().into()
    }

    async fn jnei(&self) -> U64 {
        self.0.jnei().into()
    }

    async fn jnzi(&self) -> U64 {
        self.0.jnzi().into()
    }

    async fn jmpf(&self) -> U64 {
        self.0.jmpf().into()
    }

    async fn jmpb(&self) -> U64 {
        self.0.jmpb().into()
    }

    async fn jnzf(&self) -> U64 {
        self.0.jnzf().into()
    }

    async fn jnzb(&self) -> U64 {
        self.0.jnzb().into()
    }

    async fn jnef(&self) -> U64 {
        self.0.jnef().into()
    }

    async fn jneb(&self) -> U64 {
        self.0.jneb().into()
    }

    async fn lb(&self) -> U64 {
        self.0.lb().into()
    }

    async fn log(&self) -> U64 {
        self.0.log().into()
    }

    async fn lt(&self) -> U64 {
        self.0.lt().into()
    }

    async fn lw(&self) -> U64 {
        self.0.lw().into()
    }

    async fn mint(&self) -> U64 {
        self.0.mint().into()
    }

    async fn mlog(&self) -> U64 {
        self.0.mlog().into()
    }

    async fn mod_op(&self) -> U64 {
        self.0.mod_op().into()
    }

    async fn modi(&self) -> U64 {
        self.0.modi().into()
    }

    async fn move_op(&self) -> U64 {
        self.0.move_op().into()
    }

    async fn movi(&self) -> U64 {
        self.0.movi().into()
    }

    async fn mroo(&self) -> U64 {
        self.0.mroo().into()
    }

    async fn mul(&self) -> U64 {
        self.0.mul().into()
    }

    async fn muli(&self) -> U64 {
        self.0.muli().into()
    }

    async fn mldv(&self) -> U64 {
        self.0.mldv().into()
    }

    async fn noop(&self) -> U64 {
        self.0.noop().into()
    }

    async fn not(&self) -> U64 {
        self.0.not().into()
    }

    async fn or(&self) -> U64 {
        self.0.or().into()
    }

    async fn ori(&self) -> U64 {
        self.0.ori().into()
    }

    async fn poph(&self) -> U64 {
        self.0.poph().into()
    }

    async fn popl(&self) -> U64 {
        self.0.popl().into()
    }

    async fn pshh(&self) -> U64 {
        self.0.pshh().into()
    }

    async fn pshl(&self) -> U64 {
        self.0.pshl().into()
    }

    async fn ret(&self) -> U64 {
        self.0.ret().into()
    }

    async fn rvrt(&self) -> U64 {
        self.0.rvrt().into()
    }

    async fn sb(&self) -> U64 {
        self.0.sb().into()
    }

    async fn sll(&self) -> U64 {
        self.0.sll().into()
    }

    async fn slli(&self) -> U64 {
        self.0.slli().into()
    }

    async fn srl(&self) -> U64 {
        self.0.srl().into()
    }

    async fn srli(&self) -> U64 {
        self.0.srli().into()
    }

    async fn srw(&self) -> U64 {
        self.0.srw().into()
    }

    async fn sub(&self) -> U64 {
        self.0.sub().into()
    }

    async fn subi(&self) -> U64 {
        self.0.subi().into()
    }

    async fn sw(&self) -> U64 {
        self.0.sw().into()
    }

    async fn sww(&self) -> U64 {
        self.0.sww().into()
    }

    async fn time(&self) -> U64 {
        self.0.time().into()
    }

    async fn tr(&self) -> U64 {
        self.0.tr().into()
    }

    async fn tro(&self) -> U64 {
        self.0.tro().into()
    }

    async fn wdcm(&self) -> U64 {
        self.0.wdcm().into()
    }

    async fn wqcm(&self) -> U64 {
        self.0.wqcm().into()
    }

    async fn wdop(&self) -> U64 {
        self.0.wdop().into()
    }

    async fn wqop(&self) -> U64 {
        self.0.wqop().into()
    }

    async fn wdml(&self) -> U64 {
        self.0.wdml().into()
    }

    async fn wqml(&self) -> U64 {
        self.0.wqml().into()
    }

    async fn wddv(&self) -> U64 {
        self.0.wddv().into()
    }

    async fn wqdv(&self) -> U64 {
        self.0.wqdv().into()
    }

    async fn wdmd(&self) -> U64 {
        self.0.wdmd().into()
    }

    async fn wqmd(&self) -> U64 {
        self.0.wqmd().into()
    }

    async fn wdam(&self) -> U64 {
        self.0.wdam().into()
    }

    async fn wqam(&self) -> U64 {
        self.0.wqam().into()
    }

    async fn wdmm(&self) -> U64 {
        self.0.wdmm().into()
    }

    async fn wqmm(&self) -> U64 {
        self.0.wqmm().into()
    }

    async fn xor(&self) -> U64 {
        self.0.xor().into()
    }

    async fn xori(&self) -> U64 {
        self.0.xori().into()
    }

    async fn call(&self) -> DependentCost {
        self.0.call().into()
    }

    async fn ccp(&self) -> DependentCost {
        self.0.ccp().into()
    }

    async fn croo(&self) -> DependentCost {
        self.0.croo().into()
    }

    async fn csiz(&self) -> DependentCost {
        self.0.csiz().into()
    }

    async fn k256(&self) -> DependentCost {
        self.0.k256().into()
    }

    async fn ldc(&self) -> DependentCost {
        self.0.ldc().into()
    }

    async fn logd(&self) -> DependentCost {
        self.0.logd().into()
    }

    async fn mcl(&self) -> DependentCost {
        self.0.mcl().into()
    }

    async fn mcli(&self) -> DependentCost {
        self.0.mcli().into()
    }

    async fn mcp(&self) -> DependentCost {
        self.0.mcp().into()
    }

    async fn mcpi(&self) -> DependentCost {
        self.0.mcpi().into()
    }

    async fn meq(&self) -> DependentCost {
        self.0.meq().into()
    }

    async fn retd(&self) -> DependentCost {
        self.0.retd().into()
    }

    async fn s256(&self) -> DependentCost {
        self.0.s256().into()
    }

    async fn scwq(&self) -> DependentCost {
        self.0.scwq().into()
    }

    async fn smo(&self) -> DependentCost {
        self.0.smo().into()
    }

    async fn srwq(&self) -> DependentCost {
        self.0.srwq().into()
    }

    async fn swwq(&self) -> DependentCost {
        self.0.swwq().into()
    }

    // Non-opcode prices

    async fn contract_root(&self) -> DependentCost {
        self.0.contract_root().into()
    }

    async fn state_root(&self) -> DependentCost {
        self.0.state_root().into()
    }

    async fn vm_initialization(&self) -> DependentCost {
        self.0.vm_initialization().into()
    }

    async fn new_storage_per_byte(&self) -> U64 {
        self.0.new_storage_per_byte().into()
    }
}

#[Object]
impl LightOperation {
    async fn base(&self) -> U64 {
        self.base.into()
    }

    async fn units_per_gas(&self) -> U64 {
        self.units_per_gas.into()
    }
}

#[Object]
impl HeavyOperation {
    async fn base(&self) -> U64 {
        self.base.into()
    }

    async fn gas_per_unit(&self) -> U64 {
        self.gas_per_unit.into()
    }
}

#[Object]
impl ChainInfo {
    async fn name(&self, ctx: &Context<'_>) -> async_graphql::Result<String> {
        let config: &Config = ctx.data_unchecked();
        Ok(config.chain_name.clone())
    }

    async fn latest_block(&self, ctx: &Context<'_>) -> async_graphql::Result<Block> {
        let query: &ReadView = ctx.data_unchecked();

        let latest_block = query.latest_block()?.into();
        Ok(latest_block)
    }

    async fn da_height(&self, ctx: &Context<'_>) -> U64 {
        let query: &ReadView = ctx.data_unchecked();

        let height = query
            .da_height()
            .expect("The blockchain always should have genesis block");

        height.0.into()
    }

    async fn consensus_parameters(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<ConsensusParameters> {
        let params = ctx
            .data_unchecked::<ConsensusProvider>()
            .latest_consensus_params();

        Ok(ConsensusParameters(params))
    }

    async fn gas_costs(&self, ctx: &Context<'_>) -> async_graphql::Result<GasCosts> {
        let params = ctx
            .data_unchecked::<ConsensusProvider>()
            .latest_consensus_params();

        Ok(GasCosts(params.gas_costs().clone()))
    }
}

#[derive(Default)]
pub struct ChainQuery;

#[Object]
impl ChainQuery {
    async fn chain(&self) -> ChainInfo {
        ChainInfo
    }
}
