table! {
    bookmark (id) {
        id -> Int4,
        key -> Text,
        user_id -> Int4,
        url -> Text,
        comment -> Text,
        title -> Text,
    }
}

table! {
    credential (id) {
        id -> Int4,
        user_id -> Int4,
        mail_address -> Text,
        password -> Text,
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
    user (id) {
        id -> Int4,
        key -> Text,
    }
}

joinable!(bookmark -> user (user_id));
joinable!(credential -> user (user_id));
joinable!(credential_password_reset -> credential (credential_id));
joinable!(credential_verification -> credential (credential_id));
joinable!(credential_verified -> credential (credential_id));

allow_tables_to_appear_in_same_query!(
    bookmark,
    credential,
    credential_password_reset,
    credential_verification,
    credential_verified,
    user,
);
