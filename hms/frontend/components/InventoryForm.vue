// frontend/components/InventoryForm.vue
<template>
  <UCard>
    <UForm :schema="schema" :state="state" @submit="handleSubmit">
      <UFormGroup label="Item Name" name="name">
        <UInput v-model="state.name" />
      </UFormGroup>
      <UFormGroup label="Quantity" name="quantity">
        <UInput v-model.number="state.quantity" type="number" />
      </UFormGroup>
      <UFormGroup label="Reorder Point" name="reorderPoint">
        <UInput v-model.number="state.reorderPoint" type="number" />
      </UFormGroup>
      <UFormGroup label="Expiry Date" name="expiryDate">
        <UInput v-model="state.expiryDate" type="date" />
      </UFormGroup>
      <UFormGroup label="Unit Price" name="unitPrice">
        <UInput v-model.number="state.unitPrice" type="number" step="0.01" />
      </UFormGroup>
      <UButton type="submit">Add Item</UButton>
    </UForm>
  </UCard>
</template>

<script setup lang="ts">
import { z } from 'zod';
import { useInventoryStore } from '~/stores/inventory';

const inventoryStore = useInventoryStore();
const state = ref({
  name: '',
  description: '',
  quantity: 0,
  reorderPoint: 0,
  expiryDate: '',
  unitPrice: 0,
});

const schema = z.object({
  name: z.string().min(1),
  description: z.string().optional(),
  quantity: z.number().nonnegative(),
  reorderPoint: z.number().nonnegative(),
  expiryDate: z.string().optional(),
  unitPrice: z.number().positive(),
});

async function handleSubmit() {
  try {
    await inventoryStore.addInventoryItem(state.value);
    useToast().add({ title: 'Inventory item added successfully' });
  } catch (error) {
    useToast().add({ title: 'Failed to add item', color: 'error' });
  }
}
</script>