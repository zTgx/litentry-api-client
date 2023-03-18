use sp_core::Pair;
use sp_runtime::{MultiSignature, MultiSigner};
use substrate_api_client::{
    compose_extrinsic, CallIndex, PlainTip, SubstrateDefaultSignedExtra, UncheckedExtrinsicV4,
};
use sp_core::H256;
use crate::{
    primitives::{Assertion, MrEnclave},
    ApiClient, vc_management::VC_PALLET_NAME,
};

pub type VCRequestFn = (CallIndex, H256, Assertion);
pub type VCRequestXt<SignedExtra> = UncheckedExtrinsicV4<VCRequestFn, SignedExtra>;

pub trait VcManagementXtBuilder {
    fn build_extrinsic_request_vc(
        &self,
        shard: MrEnclave,
        assertion: Assertion,
    ) -> VCRequestXt<SubstrateDefaultSignedExtra<PlainTip>>;
}

impl<P> VcManagementXtBuilder for ApiClient<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
    MultiSigner: From<P::Public>,
{
    fn build_extrinsic_request_vc(
        &self,
        shard: MrEnclave,
        assertion: Assertion,
    ) -> VCRequestXt<SubstrateDefaultSignedExtra<PlainTip>> {
        compose_extrinsic!(
            self.api.clone(),
            VC_PALLET_NAME,
            "request_vc",
            H256::from(shard),
            assertion
        )
    }
}
