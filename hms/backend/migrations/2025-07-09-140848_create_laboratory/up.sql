-- Your SQL goes here
-- backend/migrations/2025-07-09-200100_create_laboratory/up.sql
CREATE TABLE lab_tests (
                           id UUID PRIMARY KEY,
                           name VARCHAR NOT NULL,
                           description TEXT,
                           reference_range TEXT,
                           unit VARCHAR,
                           created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                           updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE lab_orders (
                            id UUID PRIMARY KEY,
                            patient_id UUID NOT NULL REFERENCES patients(id),
                            doctor_id UUID NOT NULL REFERENCES users(id),
                            appointment_id UUID REFERENCES appointments(id),
                            status VARCHAR NOT NULL CHECK (status IN ('ordered', 'in_progress', 'completed', 'cancelled')),
                            barcode VARCHAR NOT NULL UNIQUE,
                            ordered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE lab_order_tests (
                                 id UUID PRIMARY KEY,
                                 order_id UUID NOT NULL REFERENCES lab_orders(id),
                                 test_id UUID NOT NULL REFERENCES lab_tests(id),
                                 status VARCHAR NOT NULL CHECK (status IN ('pending', 'completed', 'failed')),
                                 result TEXT,
                                 notes TEXT,
                                 performed_by UUID REFERENCES users(id),
                                 performed_at TIMESTAMPTZ
);

CREATE TABLE lab_quality_control (
                                     id UUID PRIMARY KEY,
                                     test_id UUID NOT NULL REFERENCES lab_tests(id),
                                     metric_name VARCHAR NOT NULL,
                                     metric_value DECIMAL(10,2),
                                     recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                                     recorded_by UUID NOT NULL REFERENCES users(id)
);

CREATE INDEX idx_lab_orders_patient_id ON lab_orders(patient_id);
CREATE INDEX idx_lab_orders_barcode ON lab_orders(barcode);
CREATE INDEX idx_lab_order_tests_order_id ON lab_order_tests(order_id);
CREATE INDEX idx_lab_quality_control_test_id ON lab_quality_control(test_id);