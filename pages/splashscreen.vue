<template>
    <div class="bg-gray-100 flex justify-center items-center h-screen">
        <div class="w-400 h-800 flex flex-col justify-center items-center bg-white shadow-lg rounded-lg">
            <div class="animate-spin rounded-full h-32 w-32 border-t-4 border-b-4 border-blue-500"></div>
            <p class="mt-4 text-xl font-semibold text-gray-700">Loading...</p>
        </div>
    </div>
</template>

<script setup lang="ts">
import { app, invoke } from '@tauri-apps/api';
import { onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { WebviewWindow, appWindow } from '@tauri-apps/api/window';

const router = useRouter();

interface License {
    information_number: string;
    expired_date: string,
}

function isLicenseExpired(expiredDate: string): boolean {
    const today = new Date();
    today.setHours(0, 0, 0, 0);

    const parts = expiredDate.split('-').map(Number);
    const exp = new Date(parts[0], parts[1] - 1, parts[2]);
    exp.setHours(0, 0, 0, 0);

    return today.getTime() >= exp.getTime();
}

onMounted(async () => {
    try {
        console.log('=============== Start!!!!')
        await invoke('init_license');
        const json = await invoke<License>('read_license');

        if (!json.expired_date || isLicenseExpired(json.expired_date.toString())) {
            const licenseWindow = WebviewWindow.getByLabel('license_screen');
            await licenseWindow?.show();
        } else {
            const configurationWindow = WebviewWindow.getByLabel('configurationpage');
            await configurationWindow?.show();

        }
        await appWindow.close();
    } catch (error) {
        const licenseWindow = WebviewWindow.getByLabel('license_screen')
        await licenseWindow?.show()
        await appWindow.close();
    }
});
</script>

<style scoped>
@keyframes spin {
    0% {
        transform: rotate(0deg);
    }

    100% {
        transform: rotate(360deg);
    }
}
</style>