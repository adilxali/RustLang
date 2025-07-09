// frontend/components/InvoiceForm.vue
<template>
  <UCard>
    <UForm :schema="schema" :state="state" @submit="handleSubmit">
      <UFormGroup label="Patient" name="patientId">
        <USelect v-model="state.patientId" :options="patientOptions" />
      </UFormGroup>
      <UFormGroup label="Appointment" name="appointmentId">
        <USelect v-model="state.appointmentId" :options="appointmentOptions" />
      </UFormGroup>
      <UFormGroup v-for="(item, index) in state.items" :key="index" :label="`Item ${index + 1}`" :name="`items[${index}]`">
        <UInput v-model="item.description" placeholder="Description" />
        <UInput v-model.number="item.amount" type="number" placeholder="Amount" />
        <UInput v-model.number="item.quantity" type="number" placeholder="Quantity" />
        <UButton color="error" size="xs" @click="removeItem(index)">Remove</UButton>
      </UFormGroup>
      <UButton @click="addItem">Add Item</UButton>
      <UButton type="submit">Create Invoice</UButton>
    </UForm>
  </UCard>
</template>

<script setup lang="ts">
import { z } from 'zod';
import { useBillingStore } from '~/stores/billing';

const billingStore = useBillingStore();
const state = ref({
  patientId: '',
  appointmentId: '',
  items: [{ description: '', amount: 0, quantity: 1 }],
});

const schema = z.object({
  patientId: z.string().uuid(),
  appointmentId: z.string().uuid().optional(),
  items: z.array(
      z.object({
        description: z.string().min(1),
        amount: z.number().positive(),
        quantity: z.number().positive(),
      })
  ).min(1),
});

// Mock data (replace with API calls)
const patientOptions = ref([{ value: 'uuid1', label: 'John Doe' }]);
const appointmentOptions = ref([{ value: 'uuid2', label: 'Appointment #123' }]);

function addItem() {
  state.value.items.push({ description: '', amount: 0, quantity: 1 });
}

function removeItem(index: number) {
  state.value.items.splice(index, 1);
}

async function handleSubmit() {
  try {
    await billingStore.createInvoice(
        { patientId: state.value.patientId, appointmentId: state.value.appointmentId },
        state.value.items
    );
    useToast().add({ title: 'Invoice created successfully' });
  } catch (error) {
    useToast().add({ title: 'Invoice creation failed', color: 'error' });
  }
}
</script>