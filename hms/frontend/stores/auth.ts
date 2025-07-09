// frontend/stores/auth.ts
import { defineStore } from 'pinia';
import { ref } from 'vue';

export const useAuthStore = defineStore('auth', () => {
    const token = ref<string | null>(null);
    const user = ref<any | null>(null);

    async function login(credentials: { email: string; password: string }) {
        try {
            const response:any = await $fetch('/api/auth/login', {
                method: 'POST',
                body: credentials,
            });
            token.value = response.token;
            user.value = response.user;
        } catch (error) {
            throw new Error('Authentication failed');
        }
    }

    async function logout() {
        token.value = null;
        user.value = null;
    }

    return { token, user, login, logout };
});