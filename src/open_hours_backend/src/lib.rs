use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

use candid::Principal;

const NUM_DAYS: usize = 7;
const NUM_TIME_SLOTS: usize = 32; // 8am to midnight in half-hour increments

thread_local! {
    static PARTICIPANTS: RefCell<Principals> = RefCell::default();
    static OPEN_HOURS: RefCell<OpenHours> = RefCell::default(); // creates a grid analogous as on the front-end
    static CHOSEN: RefCell<HashMap<Principal, HashSet<Slot>>> = RefCell::default();
}

type HoursMatrix<T> = [[T; NUM_TIME_SLOTS]; NUM_DAYS];
type Principals = HashSet<Principal>;
type Slot = u8;

#[derive(Default)]
struct OpenHours {
    open: HoursMatrix<Principals>,
    counts: HoursMatrix<u16>,
}

impl OpenHours {
    pub fn set(&mut self, slot: Slot, participant: Principal) {
        let day = (slot & 0b111) as usize; // 3 last bits are the day
        let hour = (slot >> 3) as usize; // the rest is the hour

        self.open[day][hour].insert(participant);
        self.counts[day][hour] = self.open[day][hour].len() as u16;
    }

    pub fn unset(&mut self, slot: Slot, participant: &Principal) {
        let day = (slot & 0b111) as usize; // 3 last bits are the day
        let hour = (slot >> 3) as usize; // the rest is the hour

        self.open[day][hour].remove(participant);
        self.counts[day][hour] = self.open[day][hour].len() as u16;
    }

    pub fn counts(&self) -> &HoursMatrix<u16> {
        &self.counts
    }

    pub fn get(&self, slot: Slot) -> &Principals {
        let day = (slot & 0b111) as usize; // 3 last bits are the day
        let hour = (slot >> 3) as usize; // the rest is the hour

        &self.open[day][hour]
    }
}

#[ic_cdk::update]
fn set_open_hours(slots: Vec<Slot>) {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        ic_cdk::trap("Anonymous callers are not allowed to set open hours.");
    }

    let slots = slots.iter().cloned().collect::<HashSet<_>>();

    CHOSEN.with_borrow_mut(|s| {
        let (set, unset) = if let Some(s) = s.get(&caller) {
            let set = slots.difference(&s).collect::<HashSet<_>>();
            let unset = s.difference(&slots).collect::<Vec<_>>();
            (set, unset)
        } else {
            (slots.iter().collect::<HashSet<_>>(), vec![])
        };

        OPEN_HOURS.with_borrow_mut(|o| {
            for t in unset {
                o.unset(*t, &caller);
            }

            for t in set {
                o.set(*t, caller);
            }
        });

        s.insert(caller, slots);
    });

    PARTICIPANTS.with_borrow_mut(|u| {
        u.insert(caller);
    });
}

#[ic_cdk::query]
fn open_counts() -> (usize, HoursMatrix<u16>) {
    (
        PARTICIPANTS.with_borrow(|u| u.len()),
        OPEN_HOURS.with_borrow(|o| o.counts().clone()),
    )
}

#[ic_cdk::query]
fn open_at(slots: Vec<Slot>) -> Vec<(Slot, Vec<Principal>)> {
    OPEN_HOURS.with_borrow(|o| {
        slots
            .into_iter()
            .map(|t| {
                let principals = o.get(t).iter().cloned().collect::<Vec<_>>();
                (t, principals)
            })
            .collect()
    })
}

#[ic_cdk::query]
fn open_for(principal: Option<Principal>) -> Vec<Slot> {
    let principal = match principal {
        Some(p) => p,
        None => ic_cdk::caller(),
    };

    if principal == Principal::anonymous() {
        ic_cdk::trap("Anonymous callers are not allowed to query open hours.");
    }

    CHOSEN.with_borrow(|s| {
        s.get(&principal)
            .unwrap_or(&HashSet::new())
            .iter()
            .cloned()
            .collect::<Vec<_>>()
    })
}

ic_cdk::export_candid!();
