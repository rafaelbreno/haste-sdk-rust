use chrono::{DateTime, Utc};

pub struct PayoutDetail {
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub payee_handle: String,
    pub payer_handle: String,
    pub payment_amount: f64,
}

pub fn new_payout_detail(
    user_id: String,
    created_at: DateTime<Utc>,
    payee_handle: String,
    payer_handle: String,
    payment_amount: f64,
    ) -> PayoutDetail {
    PayoutDetail {
        user_id,
        created_at,
        payee_handle,
        payer_handle,
        payment_amount,
    }
}

pub struct PayoutEvent {
    pub event_id: String,
    pub details: PayoutDetail,
}

pub fn new_payout_event (
    event_id: String,
    details: PayoutDetail,
) -> PayoutEvent {
    PayoutEvent {
        event_id,
        details,
    }
}

pub struct Payout {
    pub starting_after: String,
    pub ending_before: String,
    pub events: Option<[PayoutEvent; 0]>,
}

pub fn new_payout (
    starting_after: String,
    ending_before: String,
    events: Option<[PayoutEvent; 0]>,
) -> Payout {
    Payout {
        starting_after,
        ending_before,
        events,
    }
}
