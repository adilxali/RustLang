// frontend/components/AppointmentCalendar.vue
<template>
  <UCard>
    <div class="calendar-container">
      <!-- Integrate a calendar library like FullCalendar -->
      <div v-for="appointment in appointments" :key="appointment.id" class="appointment">
        {{ appointment.startTime }} - {{ appointment.patientName }}
        <UButton color="error" size="xs" @click="cancelAppointment(appointment.id)">
          Cancel
        </UButton>
      </div>
    </div>
  </UCard>
</template>

<script setup lang="ts">
import { useAppointmentStore } from '~/stores/appointments';

const appointmentStore = useAppointmentStore();
const { appointments } = storeToRefs(appointmentStore);

onMounted(() => {
  // Fetch appointments for current user (doctor)
  appointmentStore.fetchDoctorAppointments('uuid2'); // Replace with actual doctor ID
});

async function cancelAppointment(id: string) {
  try {
    await appointmentStore.cancelAppointment(id);
    useToast().add({ title: 'Appointment cancelled' });
  } catch (error) {
    useToast().add({ title: 'Cancellation failed', color: 'error' });
  }
}
</script>