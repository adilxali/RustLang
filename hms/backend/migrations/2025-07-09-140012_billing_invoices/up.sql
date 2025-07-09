-- Your SQL goes here
-- backend/migrations/2025-07-09-180100_create_billing/up.sql
CREATE TABLE billing_invoices (
                                  id UUID PRIMARY KEY,
                                  patient_id UUID NOT NULL REFERENCES patients(id),
                                  appointment_id UUID REFERENCES appointments(id),
                                  total_amount DECIMAL(10,2) NOT NULL,
                                  status VARCHAR NOT NULL CHECK (status IN ('pending', 'paid', 'partially_paid', 'cancelled')),
                                  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                                  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE billing_items (
                               id UUID PRIMARY KEY,
                               invoice_id UUID NOT NULL REFERENCES billing_invoices(id),
                               description VARCHAR NOT NULL,
                               amount DECIMAL(10,2) NOT NULL,
                               quantity INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE billing_payments (
                                  id UUID PRIMARY KEY,
                                  invoice_id UUID NOT NULL REFERENCES billing_invoices(id),
                                  amount DECIMAL(10,2) NOT NULL,
                                  payment_method VARCHAR NOT NULL,
                                  transaction_id VARCHAR,
                                  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE billing_discounts (
                                   id UUID PRIMARY KEY,
                                   invoice_id UUID NOT NULL REFERENCES billing_invoices(id),
                                   description VARCHAR NOT NULL,
                                   amount DECIMAL(10,2) NOT NULL,
                                   approved_by UUID REFERENCES users(id),
                                   created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_billing_invoices_patient_id ON billing_invoices(patient_id);
CREATE INDEX idx_billing_invoices_appointment_id ON billing_invoices(appointment_id);
CREATE INDEX idx_billing_payments_invoice_id ON billing_payments(invoice_id);