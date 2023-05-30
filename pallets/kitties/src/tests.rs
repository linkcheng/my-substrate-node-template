use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn test_create() {
	new_test_ext().execute_with(|| {
		let alice = 1;
		let kitty_id = 0;
		// assert_eq!(KittiesModule::get_next_id().unwrap(), kitty_id);
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(alice)));

		// success
		assert_eq!(KittiesModule::get_next_id().unwrap(), kitty_id + 1);
		assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(alice));
		assert_eq!(KittiesModule::kitty_parents(kitty_id), None);

		// event
		let kitty = KittiesModule::kitties(kitty_id).unwrap();
		System::assert_has_event(Event::KittyCreated { who: alice, kitty_id: 0, kitty }.into());
		assert_eq!(System::events().len(), 1);

		// InvalidKittyId
		crate::NextKittyId::<Test>::set(crate::KittyId::max_value());
		assert_noop!(KittiesModule::create(RuntimeOrigin::signed(alice)), Error::<Test>::InvalidKittyId);
	});
}

#[test]
fn test_breed() {
	new_test_ext().execute_with(|| {
		let alice = 1;
		let bob = 2;
		let charles = 3;
		let kitty_id1 = 1;
		let kitty_id2 = 2;
		let kitty_id3 = 3;
		crate::NextKittyId::<Test>::set(kitty_id1);

		// InvalidKittyId
		assert_noop!(
			KittiesModule::breed(RuntimeOrigin::signed(charles), kitty_id1, kitty_id2),
			Error::<Test>::InvalidKittyId
		);
		// SameKittyId
		assert_noop!(
			KittiesModule::breed(RuntimeOrigin::signed(charles), kitty_id1, kitty_id1),
			Error::<Test>::SameKittyId
		);

		let _ = KittiesModule::create(RuntimeOrigin::signed(alice));
		let _ = KittiesModule::create(RuntimeOrigin::signed(bob));
		assert_eq!(System::events().len(), 2); // 2 creates

		// success
		assert_ok!(KittiesModule::breed(RuntimeOrigin::signed(charles), kitty_id1, kitty_id2));
		assert_eq!(KittiesModule::get_next_id().unwrap(), kitty_id3 + 1);
		assert_eq!(KittiesModule::kitty_parents(kitty_id3).unwrap(), (kitty_id1, kitty_id2));

		// event
		let kitty = KittiesModule::kitties(kitty_id3).unwrap();
		System::assert_has_event(Event::KittyBred { who: charles, kitty_id: kitty_id3, kitty }.into());
		System::assert_last_event(Event::KittyBred { who: charles, kitty_id: kitty_id3, kitty }.into());
		assert_eq!(System::events().len(), 3); // 2 creates + 1 breed
	});
}

#[test]
fn transfer_should_work() {
	new_test_ext().execute_with(|| {
		let alice = 1;
		let bob = 2;
		let kitty_id = 1;
		crate::NextKittyId::<Test>::set(kitty_id);

		// Alice creates a kitty
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(alice)));
		// Alice transfers the kitty to Bob
		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(alice), kitty_id, bob));
		// Check that the kitty now belongs to Bob
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(bob));

		// event
		// [
		// 	EventRecord {
		// 		phase: Phase::Initialization,
		// 		event: RuntimeEvent::KittiesModule(
		// 			Event::KittyCreated {
		// 				who: 1,
		// 				kitty_id: 1,
		// 				kitty: Kitty([215, 75, 66, 60, 234, 156, 146, 62, 247, 65, 230, 205, 192, 2, 31, 70])
		// 			}),
		// 		topics: []
		// 	},
		// 	EventRecord {
		// 		phase: Phase::Initialization,
		// 		event: RuntimeEvent::KittiesModule(
		// 			Event::KittyTransferred {
		// 				who: 1,
		// 				kitty_id: 1,
		// 				dest: 2
		// 			}),
		// 		topics: []
		// 	}
		// ]
		System::assert_has_event(Event::KittyTransferred { who: alice, kitty_id, dest: bob }.into());
		System::assert_last_event(Event::KittyTransferred { who: alice, kitty_id, dest: bob }.into());
		assert_eq!(System::events().len(), 2); // 1 create + 1 transfer
	});
}
#[test]
fn transfer_should_fail_if_not_owner() {
	new_test_ext().execute_with(|| {
		let alice = 1;
		let bob = 2;
		let charlie = 3;
		let kitty_id = 1;
		crate::NextKittyId::<Test>::set(kitty_id);

		// Alice creates a kitty
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(alice)));
		// Bob tries to transfer the kitty to Charlie
		assert_noop!(
			KittiesModule::transfer(RuntimeOrigin::signed(bob), kitty_id, charlie),
			Error::<Test>::NotOwner
		);
		// Check that the kitty still belongs to Alice
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(alice));
	});
}
#[test]
fn transfer_should_fail_if_invalid_kitty_id() {
	new_test_ext().execute_with(|| {
		let alice = 1;
		let bob = 2;
		let kitty_id = 1;
		crate::NextKittyId::<Test>::set(kitty_id);

		// Alice creates a kitty
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(alice)));
		// Alice transfers the kitty to Bob
		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(alice), kitty_id, bob));
		// Bob tries to transfer a non-existent kitty to Alice
		assert_noop!(
			KittiesModule::transfer(RuntimeOrigin::signed(bob), 2, alice),
			Error::<Test>::InvalidKittyId
		);
	});
}
