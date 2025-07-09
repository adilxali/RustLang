// frontend/stores/inventory.ts
import { defineStore } from 'pinia';
import { ref } from 'vue';
// import type { InventoryItem, Prescription } from '~/types';

export const useInventoryStore = defineStore('inventory', () => {
    const items = ref<any[]>([]);
    const prescriptions = ref<any[]>([]);

    async function addInventoryItem(item: { name: string; description?: string; quantity: number; reorderPoint: number; expiryDate?: string; unitPrice: number }) {
        try {
            const response = await $fetch('/api/inventory/items', {
                method: 'POST',
                body: item,
                headers: { Authorization: `Bearer ${useAuthStore().token}` },
            });
            items.value.push(response);
        } catch (error) {
            throw new Error('Failed to add inventory item');
        }
    }

    async function processPrescription(prescription: { patientId: string; doctorId: string; appointmentId?: string }, prescriptionItems: { itemId: string; quantity: number; dosageInstructions?: string }[]) {
        try {
            const response = await $fetch('/api/inventory/prescriptions', {
                method: 'POST',
                body: { prescription, items: prescriptionItems },
                headers: { Authorization: `Bearer ${useAuthStore().token}` },
            });
            prescriptions.value.push(response);
        } catch (error) {
            throw new Error('Failed to process prescription');
        }
    }

    async function fetchLowStockItems() {
        try {
            const response:any = await $fetch('/api/inventory/low-stock', {
                headers: { Authorization: `Bearer ${useAuthStore().token}` },
            });
            items.value = response;
        } catch (error) {
            throw new Error('Failed to fetch low stock items');
        }
    }

    return { items, prescriptions, addInventoryItem, processPrescription, fetchLowStockItems };
});