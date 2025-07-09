// @generated automatically by Diesel CLI.

diesel::table! {
    appointments (id) {
        id -> Uuid,
        patient_id -> Uuid,
        doctor_id -> Uuid,
        start_time -> Timestamptz,
        end_time -> Timestamptz,
        status -> Varchar,
        notes -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    billing_discounts (id) {
        id -> Uuid,
        invoice_id -> Uuid,
        description -> Varchar,
        amount -> Numeric,
        approved_by -> Nullable<Uuid>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    billing_invoices (id) {
        id -> Uuid,
        patient_id -> Uuid,
        appointment_id -> Nullable<Uuid>,
        total_amount -> Numeric,
        status -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    billing_items (id) {
        id -> Uuid,
        invoice_id -> Uuid,
        description -> Varchar,
        amount -> Numeric,
        quantity -> Int4,
    }
}

diesel::table! {
    billing_payments (id) {
        id -> Uuid,
        invoice_id -> Uuid,
        amount -> Numeric,
        payment_method -> Varchar,
        transaction_id -> Nullable<Varchar>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    inventory_items (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Nullable<Text>,
        quantity -> Int4,
        reorder_point -> Int4,
        expiry_date -> Nullable<Date>,
        unit_price -> Numeric,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    inventory_transactions (id) {
        id -> Uuid,
        item_id -> Uuid,
        transaction_type -> Varchar,
        quantity -> Int4,
        reason -> Nullable<Text>,
        created_at -> Timestamptz,
        created_by -> Uuid,
    }
}

diesel::table! {
    lab_order_tests (id) {
        id -> Uuid,
        order_id -> Uuid,
        test_id -> Uuid,
        status -> Varchar,
        result -> Nullable<Text>,
        notes -> Nullable<Text>,
        performed_by -> Nullable<Uuid>,
        performed_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    lab_orders (id) {
        id -> Uuid,
        patient_id -> Uuid,
        doctor_id -> Uuid,
        appointment_id -> Nullable<Uuid>,
        status -> Varchar,
        barcode -> Varchar,
        ordered_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    lab_quality_control (id) {
        id -> Uuid,
        test_id -> Uuid,
        metric_name -> Varchar,
        metric_value -> Nullable<Numeric>,
        recorded_at -> Timestamptz,
        recorded_by -> Uuid,
    }
}

diesel::table! {
    lab_tests (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Nullable<Text>,
        reference_range -> Nullable<Text>,
        unit -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    patients (id) {
        id -> Uuid,
        name -> Varchar,
        dob -> Date,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    prescription_items (id) {
        id -> Uuid,
        prescription_id -> Uuid,
        item_id -> Uuid,
        quantity -> Int4,
        dosage_instructions -> Nullable<Text>,
    }
}

diesel::table! {
    prescriptions (id) {
        id -> Uuid,
        patient_id -> Uuid,
        doctor_id -> Uuid,
        appointment_id -> Nullable<Uuid>,
        status -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password_hash -> Varchar,
        role -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(appointments -> patients (patient_id));
diesel::joinable!(appointments -> users (doctor_id));
diesel::joinable!(billing_discounts -> billing_invoices (invoice_id));
diesel::joinable!(billing_discounts -> users (approved_by));
diesel::joinable!(billing_invoices -> appointments (appointment_id));
diesel::joinable!(billing_invoices -> patients (patient_id));
diesel::joinable!(billing_items -> billing_invoices (invoice_id));
diesel::joinable!(billing_payments -> billing_invoices (invoice_id));
diesel::joinable!(inventory_transactions -> inventory_items (item_id));
diesel::joinable!(inventory_transactions -> users (created_by));
diesel::joinable!(lab_order_tests -> lab_orders (order_id));
diesel::joinable!(lab_order_tests -> lab_tests (test_id));
diesel::joinable!(lab_order_tests -> users (performed_by));
diesel::joinable!(lab_orders -> appointments (appointment_id));
diesel::joinable!(lab_orders -> patients (patient_id));
diesel::joinable!(lab_orders -> users (doctor_id));
diesel::joinable!(lab_quality_control -> lab_tests (test_id));
diesel::joinable!(lab_quality_control -> users (recorded_by));
diesel::joinable!(prescription_items -> inventory_items (item_id));
diesel::joinable!(prescription_items -> prescriptions (prescription_id));
diesel::joinable!(prescriptions -> appointments (appointment_id));
diesel::joinable!(prescriptions -> patients (patient_id));
diesel::joinable!(prescriptions -> users (doctor_id));

diesel::allow_tables_to_appear_in_same_query!(
    appointments,
    billing_discounts,
    billing_invoices,
    billing_items,
    billing_payments,
    inventory_items,
    inventory_transactions,
    lab_order_tests,
    lab_orders,
    lab_quality_control,
    lab_tests,
    patients,
    prescription_items,
    prescriptions,
    users,
);
