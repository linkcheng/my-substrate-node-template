#[cfg(test)]
mod tests {
	use crate::{mock::*, Error, Proofs};
	use frame_support::{assert_noop, assert_ok, BoundedVec};
	use sp_core::ConstU32;

	fn generate_bounded_claim() -> BoundedVec<u8, ConstU32<10>> {
		BoundedVec::try_from(vec![0, 1, 2]).unwrap()
	}
	#[test]
	fn create_claim_should_work() {
		new_test_ext().execute_with(|| {
			let claim = generate_bounded_claim();
			assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));
			assert_eq!(Proofs::<Test>::get(&claim).unwrap().0, 1);
		});
	}
	#[test]
	fn create_claim_should_fail_when_claim_already_exist() {
		new_test_ext().execute_with(|| {
			let claim = generate_bounded_claim();
			assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));
			assert_noop!(
				PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()),
				Error::<Test>::ProofAlreadyExist
			);
		});
	}

	#[test]
	fn revoke_claim_should_work() {
		new_test_ext().execute_with(|| {
			let claim = generate_bounded_claim();
			let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
			assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()));
		});
	}

	#[test]
	fn revoke_claim_should_fail_when_claim_not_exist() {
		new_test_ext().execute_with(|| {
			let claim = generate_bounded_claim();
			assert_noop!(
				PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()),
				Error::<Test>::ClaimNotExist
			);
		});
	}

	#[test]
	fn revoke_claim_should_fail_when_not_claim_owner() {
		new_test_ext().execute_with(|| {
			let claim = generate_bounded_claim();
			let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
			assert_noop!(
				PoeModule::revoke_claim(RuntimeOrigin::signed(2), claim.clone()),
				Error::<Test>::NotClaimOwner
			);
		});
	}

	#[test]
	fn transfer_claim_should_work() {
		new_test_ext().execute_with(|| {
			let claim = generate_bounded_claim();
			let sender = 1;
			let dest = 2;
			let _ = PoeModule::create_claim(RuntimeOrigin::signed(sender), claim.clone());

			assert_ok!(PoeModule::transfer_claim(
				RuntimeOrigin::signed(sender),
				claim.clone(),
				dest
			));
			assert_eq!(
				Proofs::<Test>::get(&claim),
				Some((dest, frame_system::Pallet::<Test>::block_number()))
			);
		});
	}

	#[test]
	fn transfer_claim_should_fail_if_claim_does_not_exist() {
		new_test_ext().execute_with(|| {
			let claim = generate_bounded_claim();
			let sender = 1;
			let dest = 2;

			assert_noop!(
				PoeModule::transfer_claim(RuntimeOrigin::signed(sender), claim.clone(), dest),
				Error::<Test>::ClaimNotExist
			);
		});
	}

	#[test]
	fn transer_claim_should_fail_if_not_claim_owner() {
		new_test_ext().execute_with(|| {
			let claim = generate_bounded_claim();
			let sender1 = 1;
			let sender2: u64 = 2;
			let dest = 3;
			let _ = PoeModule::create_claim(RuntimeOrigin::signed(sender1), claim.clone());

			assert_noop!(
				PoeModule::transfer_claim(RuntimeOrigin::signed(sender2), claim.clone(), dest),
				Error::<Test>::NotClaimOwner
			);
		});
	}
}
