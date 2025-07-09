-- Your SQL goes here
-- backend/migrations/2025-07-09-170100_create_appointments/up.sql
CREATE TABLE appointments (
                              id UUID PRIMARY KEY,
                              patient_id UUID NOT NULL REFERENCES patients(id),
                              doctor_id UUID NOT NULL REFERENCES users(id),
                              start_time TIMESTAMPTZ NOT NULL,
                              end_time TIMESTAMPTZ NOT NULL,
                              status VARCHAR NOT NULL CHECK (status IN ('scheduled', 'completed', 'cancelled', 'no_show')),
                              notes TEXT,
                              created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                              updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_appointments_doctor_id ON appointments(doctor_id);
CREATE INDEX idx_appointments_patient_id ON appointments(patient_id);
CREATE INDEX idx_appointments_start_time ON appointments(start_time);