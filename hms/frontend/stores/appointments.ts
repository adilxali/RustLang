// frontend/stores/appointments.ts
import { defineStore } from 'pinia';
import { ref } from 'vue';
// import type { Appointment } from '~/types';

export const useAppointmentStore = defineStore('appointments', () => {
    const appointments = ref<any[]>([]);

    async function createAppointment(appointment: {
        patientId: string;
        doctorId: string;
        startTime: string;
        endTime: string;
        notes?: string;
    }) {
        try {
            const response = await $fetch('/api/appointments', {
                method: 'POST',
                body: appointment,
                headers: { Authorization: `Bearer ${useAuthStore().token}` },
            });
            appointments.value.push(response);
        } catch (error) {
            throw new Error('Failed to create appointment');
        }
    }

    async function fetchDoctorAppointments(doctorId: string) {
        try {
            const response:any = await $fetch(`/api/appointments/doctor/${doctorId}`, {
                headers: { Authorization: `Bearer ${useAuthStore().token}` },
            });
            appointments.value = response;
        } catch (error) {
            throw new Error('Failed to fetch appointments');
        }
    }

    async function cancelAppointment(appointmentId: string) {
        try {
            const response = await $fetch(`/api/appointments/${appointmentId}/cancel`, {
                method: 'PUT',
                headers: { Authorization: `Bearer ${useAuthStore().token}` },
            });
            appointments.value = appointments.value.map((appt) =>
                appt.id === appointmentId ? response : appt
            );
        } catch (error) {
            throw new Error('Failed to cancel appointment');
        }
    }

    return { appointments, createAppointment, fetchDoctorAppointments, cancelAppointment };
});