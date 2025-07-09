// frontend/components/AppointmentScheduler.vue
<template>
  <UCard>
    <UForm :schema="schema" :state="state" @submit="handleSubmit">
      <UFormGroup label="Patient" name="patientId">
        <USelect v-model="state.patientId" :options="patientOptions" />
      </UFormGroup>
      <UFormGroup label="Doctor" name="doctorId">
        <USelect v-model="state.doctorId" :options="doctorOptions" />
      </UFormGroup>
      <UFormGroup label="Start Time" name="startTime">
        <UInput v-model="state.startTime" type="datetime-local" />
      </UFormGroup>
      <UFormGroup label="Notes" name="notes">
        <UTextarea v-model="state.notes" />
      </UFormGroup>
      <UButton type="submit">Schedule Appointment</UButton>
    </UForm>
  </UCard>
</template>

<script setup lang="ts">
import { z } from 'zod';
import { useAppointmentStore } from '~/stores/appointments';

const appointmentStore = useAppointmentStore();
const state = ref({
  patientId: '',
  doctorId: '',
  startTime: '',
  notes: '',
});

const schema = z.object({
  patientId: z.string().uuid(),
  doctorId: z.string().uuid(),
  startTime: z.string().datetime(),
  notes: z.string().optional(),
});

// Mock data for select options (replace with API calls)
const patientOptions = ref([{ value: 'uuid1', label: 'John Doe' }]);
const doctorOptions = ref([{ value: 'uuid2', label: 'Dr. Smith' }]);

async function handleSubmit() {
  try {
    const start = new Date(state.value.startTime);
    const end = new Date(start.getTime() + 30 * 60000); // 30-minute default duration
    await appointmentStore.createAppointment({
      ...state.value,
      endTime: end.toISOString(),
    });
    useToast().add({ title: 'Appointment scheduled successfully' });
  } catch (error) {
    useToast().add({ title: 'Scheduling failed', color: 'error' });
  }
}
</script>