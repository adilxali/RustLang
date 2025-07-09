// frontend/components/TestResultForm.vue
<template>
  <UCard>
    <UForm :schema="schema" :state="state" @submit="handleSubmit">
      <UFormGroup label="Test Status" name="status">
        <USelect v-model="state.status" :options="statusOptions" />
      </UFormGroup>
      <UFormGroup label="Result" name="result">
        <UInput v-model="state.result" />
      </UFormGroup>
      <UFormGroup label="Notes" name="notes">
        <UTextarea v-model="state.notes" />
      </UFormGroup>
      <UButton type="submit">Update Result</UButton>
    </UForm>
  </UCard>
</template>

<script setup lang="ts">
import { z } from 'zod';
import { useLaboratoryStore } from '~/stores/laboratory';

const props = defineProps<{ testId: string }>();
const laboratoryStore = useLaboratoryStore();
const state = ref<any>({
  status: 'pending',
  result: '',
  notes: '',
});

const schema = z.object({
  status: z.enum(['pending', 'completed', 'failed']),
  result: z.string().optional(),
  notes: z.string().optional(),
});

const statusOptions = ref([
  { value: 'pending', label: 'Pending' },
  { value: 'completed', label: 'Completed' },
  { value: 'failed', label: 'Failed' },
]);

async function handleSubmit() {
  try {
    await laboratoryStore.updateTestResult(props.testId, state.value);
    useToast().add({ title: 'Test result updated successfully' });
  } catch (error) {
    useToast().add({ title: 'Test result update failed', color: 'error' });
  }
}
</script>