<template>
    <div class="bg-gray-100 flex justify-center items-center h-screen">
        <div class="w-400 h-800 flex flex-col justify-center items-center bg-white shadow-lg rounded-lg">
            <div class="animate-spin rounded-full h-32 w-32 border-t-4 border-b-4 border-blue-500"></div>
            <p class="mt-4 text-xl font-semibold text-gray-700">Loading...</p>
        </div>
    </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { onMounted } from 'vue';
import { WebviewWindow, appWindow } from '@tauri-apps/api/window';
import { emit, listen } from '@tauri-apps/api/event';

interface License {
    session_code: string,
}

function isLicenseExpired(expiredDate: string): boolean {
    const today = new Date();
    today.setHours(0, 0, 0, 0);

    const parts = expiredDate.trim().split('-').map(Number);
    if (parts.some(isNaN)) {
        return true;
    }

    const exp = new Date(parts[0], parts[1] - 1, parts[2]);
    exp.setHours(0, 0, 0, 0);

    return today.getTime() >= exp.getTime();
}

async function checkLicenseLogic() {
    try {
        await invoke('init_license');

        const diskId: string = await invoke('get_disk_id_windows');
        const json = await invoke<License>('read_license');

        const expiredDate: string = await invoke('get_date_from_hash', {
            token: json.session_code,
            infoNumber: diskId,
        });

        const isTokenEmpty = !json.session_code || json.session_code.trim() === '';
        const isInvalidFormat = expiredDate === 'Invalid' || !expiredDate.includes('-');

        if (isTokenEmpty || isInvalidFormat || isLicenseExpired(expiredDate)) {
            const licenseWindow = WebviewWindow.getByLabel('license_screen');
            await licenseWindow?.show();
            await appWindow.hide();
            return;
        }

        if (expiredDate === 'Invalid' || isLicenseExpired(expiredDate)) {
            const licenseWindow = WebviewWindow.getByLabel('license_screen');
            await licenseWindow?.show();
            await appWindow.hide();
        } else {
            await emit('license-validation-success');
            const configurationWindow = WebviewWindow.getByLabel('configurationpage');
            await configurationWindow?.show();
            await appWindow.close();
        }
    } catch (error) {
        const licenseWindow = WebviewWindow.getByLabel('license_screen')

        if (licenseWindow) {
            await licenseWindow.show();
            await licenseWindow.unminimize();
            await licenseWindow.setFocus();
        }

        await appWindow.close();
    }
}

onMounted(async () => {
    setTimeout(async () => {
        await checkLicenseLogic();
    }, 1000);

    await listen('check-license-now', async () => {
        await checkLicenseLogic();
    });
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