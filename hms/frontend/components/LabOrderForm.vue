// frontend/components/LabOrderForm.vue
<template>
  <UCard>
    <UForm :schema="schema" :state="state" @submit="handleSubmit">
      <UFormGroup label="Patient" name="patientId">
        <USelect v-model="state.patientId" :options="patientOptions" />
      </UFormGroup>
      <UFormGroup label="Doctor" name="doctorId">
        <USelect v-model="state.doctorId" :options="doctorOptions" />
      </UFormGroup>
      <UFormGroup v-for="(test, index) in state.tests" :key="index" :label="`Test ${index + 1}`" :name="`tests[${index}]`">
        <USelect v-model="test.testId" :options="testOptions" placeholder="Select Test" />
        <UButton color="error" size="xs" @click="removeTest(index)">Remove</UButton>
      </UFormGroup>
      <UButton @click="addTest">Add Test</UButton>
      <UButton type="submit">Create Lab Order</UButton>
    </UForm>
  </UCard>
</template>

<script setup lang="ts">
import { z } from 'zod';
import { useLaboratoryStore } from '~/stores/laboratory';

const laboratoryStore = useLaboratoryStore();
const state = ref({
  patientId: '',
  doctorId: '',
  appointmentId: '',
  tests: [{ testId: '' }],
});

const schema = z.object({
  patientId: z.string().uuid(),
  doctorId: z.string().uuid(),
  appointmentId: z.string().uuid().optional(),
  tests: z.array(z.object({ testId: z.string().uuid() })).min(1),
});

// Mock data (replace with API calls)
const patientOptions = ref([{ value: 'uuid1', label: 'John Doe' }]);
const doctorOptions = ref([{ value: 'uuid2', label: 'Dr. Smith' }]);
const testOptions = ref([{ value: 'uuid3', label: 'Blood Test' }]);

function addTest() {
  state.value.tests.push({ testId: '' });
}

function removeTest(index: number) {
  state.value.tests.splice(index, 1);
}

async function handleSubmit() {
  try {
    await laboratoryStore.createLabOrder(
        { patientId: state.value.patientId, doctorId: state.value.doctorId, appointmentId: state.value.appointmentId },
        state.value.tests
    );
    useToast().add({ title: 'Lab order created successfully' });
  } catch (error) {
    useToast().add({ title: 'Lab order creation failed', color: 'error' });
  }
}
</script>