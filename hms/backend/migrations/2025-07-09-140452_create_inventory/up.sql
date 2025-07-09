-- Your SQL goes here
-- backend/migrations/2025-07-09-193400_create_inventory/up.sql
CREATE TABLE inventory_items (
                                 id UUID PRIMARY KEY,
                                 name VARCHAR NOT NULL,
                                 description TEXT,
                                 quantity INTEGER NOT NULL,
                                 reorder_point INTEGER NOT NULL,
                                 expiry_date DATE,
                                 unit_price DECIMAL(10,2) NOT NULL,
                                 created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                                 updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE inventory_transactions (
                                        id UUID PRIMARY KEY,
                                        item_id UUID NOT NULL REFERENCES inventory_items(id),
                                        transaction_type VARCHAR NOT NULL CHECK (transaction_type IN ('add', 'remove', 'adjust')),
                                        quantity INTEGER NOT NULL,
                                        reason TEXT,
                                        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                                        created_by UUID NOT NULL REFERENCES users(id)
);

CREATE TABLE prescriptions (
                               id UUID PRIMARY KEY,
                               patient_id UUID NOT NULL REFERENCES patients(id),
                               doctor_id UUID NOT NULL REFERENCES users(id),
                               appointment_id UUID REFERENCES appointments(id),
                               status VARCHAR NOT NULL CHECK (status IN ('pending', 'fulfilled', 'cancelled')),
                               created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                               updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE prescription_items (
                                    id UUID PRIMARY KEY,
                                    prescription_id UUID NOT NULL REFERENCES prescriptions(id),
                                    item_id UUID NOT NULL REFERENCES inventory_items(id),
                                    quantity INTEGER NOT NULL,
                                    dosage_instructions TEXT
);

CREATE INDEX idx_inventory_items_name ON inventory_items(name);
CREATE INDEX idx_inventory_transactions_item_id ON inventory_transactions(item_id);
CREATE INDEX idx_prescriptions_patient_id ON prescriptions(patient_id);
CREATE INDEX idx_prescription_items_prescription_id ON prescription_items(prescription_id);