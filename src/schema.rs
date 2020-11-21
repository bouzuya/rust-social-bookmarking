table! {
    credential (id) {
        id -> Int4,
        user_id -> Int4,
        mail_address -> Text,
        password -> Text,
        status -> Int4,
    }
}

table! {
    credential_password_reset (credential_id) {
        credential_id -> Int4,
        secret -> Text,
        expired_at -> Timestamp,
    }
}

table! {
    credential_verification (credential_id) {
        credential_id -> Int4,
        secret -> Text,
        expired_at -> Timestamp,
    }
}

table! {
    credential_verified (credential_id) {
        credential_id -> Int4,
        verified_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        key -> Text,
    }
}

joinable!(credential -> users (user_id));
joinable!(credential_password_reset -> credential (credential_id));
joinable!(credential_verification -> credential (credential_id));
joinable!(credential_verified -> credential (credential_id));

allow_tables_to_appear_in_same_query!(
    credential,
    credential_password_reset,
    credential_verification,
    credential_verified,
    users,
);
