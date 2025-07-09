// frontend/components/PaymentForm.vue
<template>
  <UCard>
    <UForm :schema="schema" :state="state" @submit="handleSubmit">
      <UFormGroup label="Invoice" name="invoiceId">
        <USelect v-model="state.invoiceId" :options="invoiceOptions" />
      </UFormGroup>
      <UFormGroup label="Amount" name="amount">
        <UInput v-model.number="state.amount" type="number" />
      </UFormGroup>
      <UFormGroup label="Payment Method" name="paymentMethod">
        <USelect v-model="state.paymentMethod" :options="paymentMethodOptions" />
      </UFormGroup>
      <UButton type="submit">Process Payment</UButton>
    </UForm>
  </UCard>
</template>

<script setup lang="ts">
import { z } from 'zod';
import { useBillingStore } from '~/stores/billing';

const billingStore = useBillingStore();
const state = ref({
  invoiceId: '',
  amount: 0,
  paymentMethod: '',
});

const schema = z.object({
  invoiceId: z.string().uuid(),
  amount: z.number().positive(),
  paymentMethod: z.string().min(1),
});

// Mock data (replace with API calls)
const invoiceOptions = ref([{ value: 'uuid3', label: 'Invoice #001' }]);
const paymentMethodOptions = ref([
  { value: 'credit_card', label: 'Credit Card' },
  { value: 'cash', label: 'Cash' },
  { value: 'insurance', label: 'Insurance' },
]);

async function handleSubmit() {
  try {
    await billingStore.processPayment({
      ...state.value,
      transactionId: `TXN-${Date.now()}`,
    });
    useToast().add({ title: 'Payment processed successfully' });
  } catch (error) {
    useToast().add({ title: 'Payment processing failed', color: 'error' });
  }
}
</script>