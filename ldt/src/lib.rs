//! This crate contains a framework for low-degree tests (LDTs).

#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use core::marker::PhantomData;
use p3_challenger::Challenger;
use p3_commit::MMCS;
use p3_commit::PCS;
use p3_field::{ExtensionField, Field, TwoAdicField};
use p3_lde::TwoAdicLDE;
use p3_matrix::dense::RowMajorMatrix;

/// A batch low-degree test (LDT).
pub trait LDT<F: Field, M: MMCS<F>> {
    type Proof;
    type Error;

    /// Prove that each column of each matrix in `codewords` is a codeword.
    fn prove<Chal>(codewords: &[M::ProverData], challenger: &mut Chal) -> Self::Proof
    where
        Chal: Challenger<F>;

    fn verify<Chal>(
        codeword_commits: &[M::Commitment],
        proof: &Self::Proof,
        challenger: &mut Chal,
    ) -> Result<(), Self::Error>
    where
        Chal: Challenger<F>;
}

pub struct LDTBasedPCS<Val, Dom, LDE, M, L> {
    _phantom_val: PhantomData<Val>,
    _phantom_dom: PhantomData<Dom>,
    _phantom_lde: PhantomData<LDE>,
    _phantom_m: PhantomData<M>,
    _phantom_l: PhantomData<L>,
}

impl<Val, Dom, LDE, M, L> PCS<Val> for LDTBasedPCS<Val, Dom, LDE, M, L>
where
    Val: Field,
    Dom: ExtensionField<Val> + TwoAdicField,
    LDE: TwoAdicLDE<Val, Dom>,
    M: MMCS<Dom>,
    L: LDT<Dom, M>,
{
    type Commitment = M::Commitment;
    type ProverData = M::ProverData;
    type Proof = L::Proof;
    type Error = L::Error;

    fn commit_batches(
        _polynomials: Vec<RowMajorMatrix<Val>>,
    ) -> (Self::Commitment, Self::ProverData) {
        // (Streaming?) LDE + Merklize
        todo!()
    }

    fn get_committed_value(_prover_data: &Self::ProverData, _poly: usize, _value: usize) -> Val {
        todo!()
    }
}
