// frontend/components/PrescriptionForm.vue
<template>
  <UCard>
    <UForm :schema="schema" :state="state" @submit="handleSubmit">
      <UFormGroup label="Patient" name="patientId">
        <USelect v-model="state.patientId" :options="patientOptions" />
      </UFormGroup>
      <UFormGroup label="Doctor" name="doctorId">
        <USelect v-model="state.doctorId" :options="doctorOptions" />
      </UFormGroup>
      <UFormGroup v-for="(item, index) in state.items" :key="index" :label="`Item ${index + 1}`" :name="`items[${index}]`">
        <USelect v-model="item.itemId" :options="inventoryOptions" placeholder="Select Item" />
        <UInput v-model.number="item.quantity" type="number" placeholder="Quantity" />
        <UInput v-model="item.dosageInstructions" placeholder="Dosage Instructions" />
        <UButton color="error" size="xs" @click="removeItem(index)">Remove</UButton>
      </UFormGroup>
      <UButton @click="addItem">Add Item</UButton>
      <UButton type="submit">Process Prescription</UButton>
    </UForm>
  </UCard>
</template>

<script setup lang="ts">
import { z } from 'zod';
import { useInventoryStore } from '~/stores/inventory';

const inventoryStore = useInventoryStore();
const state = ref({
  patientId: '',
  doctorId: '',
  appointmentId: '',
  items: [{ itemId: '', quantity: 1, dosageInstructions: '' }],
});

const schema = z.object({
  patientId: z.string().uuid(),
  doctorId: z.string().uuid(),
  appointmentId: z.string().uuid().optional(),
  items: z.array(
      z.object({
        itemId: z.string().uuid(),
        quantity: z.number().positive(),
        dosageInstructions: z.string().optional(),
      })
  ).min(1),
});

// Mock data (replace with API calls)
const patientOptions = ref([{ value: 'uuid1', label: 'John Doe' }]);
const doctorOptions = ref([{ value: 'uuid2', label: 'Dr. Smith' }]);
const inventoryOptions = ref([{ value: 'uuid3', label: 'Paracetamol' }]);

function addItem() {
  state.value.items.push({ itemId: '', quantity: 1, dosageInstructions: '' });
}

function removeItem(index: number) {
  state.value.items.splice(index, 1);
}

async function handleSubmit() {
  try {
    await inventoryStore.processPrescription(
        { patientId: state.value.patientId, doctorId: state.value.doctorId, appointmentId: state.value.appointmentId },
        state.value.items
    );
    useToast().add({ title: 'Prescription processed successfully' });
  } catch (error) {
    useToast().add({ title: 'Prescription processing failed', color: 'error' });
  }
}
</script>