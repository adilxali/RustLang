// frontend/stores/laboratory.ts
import { defineStore } from 'pinia';
import { ref } from 'vue';
// import type { LabOrder, LabOrderTest, LabQualityControl } from '~/types';

export const useLaboratoryStore = defineStore('laboratory', () => {
    const orders = ref<any[]>([]);

    async function createLabOrder(order: { patientId: string; doctorId: string; appointmentId?: string }, tests: { testId: string }[]) {
        try {
            const response = await $fetch('/api/laboratory/orders', {
                method: 'POST',
                body: { order, tests: tests.map(t => ({ testId: t.testId, status: 'pending' })) },
                headers: { Authorization: `Bearer ${useAuthStore().token}` },
            });
            orders.value.push(response);
        } catch (error) {
            throw new Error('Failed to create lab order');
        }
    }

    async function updateTestResult(testId: string, update: { status: string; result?: string; notes?: string }) {
        try {
            const response = await $fetch(`/api/laboratory/tests/${testId}`, {
                method: 'PUT',
                body: update,
                headers: { Authorization: `Bearer ${useAuthStore().token}` },
            });
            orders.value = orders.value.map((order:any) => ({
                ...order,
                tests: order.tests?.map((test:any) => test.id === testId ? response : test),
            }));
        } catch (error) {
            throw new Error('Failed to update test result');
        }
    }

    async function recordQualityControl(qc: { testId: string; metricName: string; metricValue: number; recordedBy: string }) {
        try {
            await $fetch('/api/laboratory/quality-control', {
                method: 'POST',
                body: qc,
                headers: { Authorization: `Bearer ${useAuthStore().token}` },
            });
            useToast().add({ title: 'Quality control recorded' });
        } catch (error) {
            throw new Error('Failed to record quality control');
        }
    }

    async function fetchPatientOrders(patientId: string) {
        try {
            const response:any = await $fetch(`/api/laboratory/orders/patient/${patientId}`, {
                headers: { Authorization: `Bearer ${useAuthStore().token}` },
            });
            orders.value = response;
        } catch (error) {
            throw new Error('Failed to fetch lab orders');
        }
    }

    return { orders, createLabOrder, updateTestResult, recordQualityControl, fetchPatientOrders };
});