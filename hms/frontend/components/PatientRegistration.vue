// frontend/components/PatientRegistration.vue
<template>
  <UCard>
    <UForm :schema="schema" :state="state" @submit="handleSubmit">
      <UFormGroup label="Full Name" name="name">
        <UInput v-model="state.name" />
      </UFormGroup>
      <UFormGroup label="Date of Birth" name="dob">
        <UInput v-model="state.dob" type="date" />
      </UFormGroup>
      <UButton type="submit">Register Patient</UButton>
    </UForm>
  </UCard>
</template>

<script setup lang="ts">
import { z } from 'zod';
import { useAuthStore } from '~/stores/auth';

const authStore = useAuthStore();
const state = ref({
  name: '',
  dob: '',
});

const schema = z.object({
  name: z.string().min(2),
  dob: z.string().regex(/^\d{4}-\d{2}-\d{2}$/),
});

async function handleSubmit() {
  try {
    await $fetch('/api/patients', {
      method: 'POST',
      body: state.value,
      headers: { Authorization: `Bearer ${authStore.token}` },
    });
    useToast().add({ title: 'Patient registered successfully' });
  } catch (error) {
    useToast().add({ title: 'Registration failed', color: 'error' });
  }
}
</script>