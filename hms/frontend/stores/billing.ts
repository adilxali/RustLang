// frontend/stores/billing.ts
import { defineStore } from 'pinia';
import { ref } from 'vue';
// import type { Invoice, BillingItem, Payment, Discount } from '~/types';

export const useBillingStore = defineStore('billing', () => {
    const invoices = ref<any[]>([]);

    async function createInvoice(invoice: { patientId: string; appointmentId?: string }, items: any[]) {
        try {
            const response = await $fetch('/api/billing/invoices', {
                method: 'POST',
                body: { invoice, items },
                headers: { Authorization: `Bearer ${useAuthStore().token}` },
            });
            invoices.value.push(response);
        } catch (error) {
            throw new Error('Failed to create invoice');
        }
    }

    async function processPayment(payment: { invoiceId: string; amount: number; paymentMethod: string; transactionId?: string }) {
        try {
            const response:any = await $fetch('/api/billing/payments', {
                method: 'POST',
                body: payment,
                headers: { Authorization: `Bearer ${useAuthStore().token}` },
            });
            invoices.value = invoices.value.map((inv:any) => (inv.id === response.id ? response : inv));
        } catch (error) {
            throw new Error('Failed to process payment');
        }
    }

    async function applyDiscount(discount: { invoiceId: string; description: string; amount: number; approvedBy?: string }) {
        try {
            const response:any = await $fetch('/api/billing/discounts', {
                method: 'POST',
                body: discount,
                headers: { Authorization: `Bearer ${useAuthStore().token}` },
            });
            invoices.value = invoices.value.map((inv:any) => (inv.id === response.id ? response : inv));
        } catch (error) {
            throw new Error('Failed to apply discount');
        }
    }

    async function fetchPatientInvoices(patientId: string) {
        try {
            const response:any = await $fetch(`/api/billing/invoices/patient/${patientId}`, {
                headers: { Authorization: `Bearer ${useAuthStore().token}` },
            });
            invoices.value = response;
        } catch (error) {
            throw new Error('Failed to fetch invoices');
        }
    }

    return { invoices, createInvoice, processPayment, applyDiscount, fetchPatientInvoices };
});