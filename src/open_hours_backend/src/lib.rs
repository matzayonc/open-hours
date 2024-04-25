use std::{cell::RefCell, collections::HashSet};

use candid::Principal;

const NUM_DAYS: usize = 7;
const NUM_TIME_SLOTS: usize = 32; // 8am to midnight in half-hour increments

type HoursMatrix<T> = [[T; NUM_TIME_SLOTS]; NUM_DAYS];
type Principals = HashSet<Principal>;
type Slot = u8;

thread_local! {
    static PARTICIPANTS: RefCell<Principals> = RefCell::default();
    static OPEN_HOURS: RefCell<HoursMatrix<Principals>> = RefCell::default(); // creates a grid analogous as on the front-end
    static OPEN_HOURS_COUNTS: RefCell<HoursMatrix<u16>> = RefCell::default();
}

#[ic_cdk::update]
fn set_open_hours(slots: Vec<Slot>) {
    let caller = ic_cdk::caller();

    if caller == Principal::anonymous() {
        ic_cdk::trap("Anonymous callers are not allowed to set open hours.");
    }

    OPEN_HOURS.with_borrow_mut(|o| {
        OPEN_HOURS_COUNTS.with_borrow_mut(|c| {
            for t in slots {
                let day = (t & 0b111) as usize; // 3 last bits are the day
                let hour = (t >> 3) as usize; // the rest is the hour

                o[day][hour].insert(caller);
                c[day][hour] = o[day][hour].len() as u16;
            }
        });
    });

    PARTICIPANTS.with_borrow_mut(|u| {
        u.insert(caller);
    });
}

#[ic_cdk::query]
fn get_open_hours_stats() -> (usize, HoursMatrix<u16>) {
    (
        PARTICIPANTS.with_borrow(|u| u.len()),
        OPEN_HOURS_COUNTS.with_borrow(|c| *c),
    )
}

#[ic_cdk::query]
fn get_open_for(slots: Vec<Slot>) -> Vec<(Slot, Vec<Principal>)> {
    OPEN_HOURS.with_borrow(|o| {
        slots
            .into_iter()
            .map(|t| {
                let day = (t & 0b111) as usize;
                let hour = (t >> 3) as usize;

                let principals = o[day][hour].iter().cloned().collect::<Vec<_>>();

                (t, principals)
            })
            .collect()
    })
}

ic_cdk::export_candid!();
