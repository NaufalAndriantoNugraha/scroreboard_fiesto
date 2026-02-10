<template>
    <div class="background">
        <div class="main-container">
            <div class="information">
                <h1>Nomor Informasi Anda</h1>
                <p class="description">Di bawah ini merupakan nomor Informasi yang Anda dapat tukarkan untuk mendapatkan
                    kode token. Setelah mendapatkan kode token, silahkan masukkan kode token tersebut.</p>
                <div class="code">
                    <p class="number">{{ informationNumber }}</p>
                    <svg xmlns="http://www.w3.org/2000/svg" class="copy" width="4rem" @click="copyToClipboard"
                        height="1.5rem" viewBox="0 0 20 20">
                        <title xmlns="">copy-20-regular</title>
                        <path fill="currentColor"
                            d="M8 2a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2V4a2 2 0 0 0-2-2zM7 4a1 1 0 0 1 1-1h6a1 1 0 0 1 1 1v10a1 1 0 0 1-1 1H8a1 1 0 0 1-1-1zM4 6a2 2 0 0 1 1-1.732V14.5A2.5 2.5 0 0 0 7.5 17h6.232A2 2 0 0 1 12 18H7.5A3.5 3.5 0 0 1 4 14.5z" />
                    </svg>
                </div>
            </div>
            <div class="form">
                <input type="text" placeholder="Masukkan kode token..." v-model="tokenOrEncryptedCode"maxlength="150" ></input>
                <div class="button" @click="checkEncryptedCode()">Submit</div>
            </div>
        </div>
    </div>
    <div v-if="showSnackbar" class="snackbar">
        <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24" class="svg">
            <title xmlns="">info</title>
            <path fill="currentColor"
                d="M11.5 16.5h1V11h-1zm.5-6.923q.262 0 .439-.177t.176-.439t-.177-.438T12 8.346t-.438.177t-.177.439t.177.438t.438.177M12.003 21q-1.867 0-3.51-.708q-1.643-.709-2.859-1.924t-1.925-2.856T3 12.003t.709-3.51Q4.417 6.85 5.63 5.634t2.857-1.925T11.997 3t3.51.709q1.643.708 2.859 1.922t1.925 2.857t.709 3.509t-.708 3.51t-1.924 2.859t-2.856 1.925t-3.509.709" />
        </svg>
        <p>
            {{ snackbarMessage }}
        </p>
    </div>
</template>

<style>
@import url('https://fonts.googleapis.com/css2?family=Nunito+Sans:ital,opsz,wght@0,6..12,200..1000;1,6..12,200..1000&display=swap');

.background {
    background-color: #f2f2f2;
    height: 100vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    font-family: "Nunito Sans", sans-serif;
    background-image: radial-gradient(#d1d5db 1px, transparent 0);
    background-size: 24px 24px;
}

.main-container {
    background-color: white;
    max-width: 380px;
    overflow: hidden;
    border-radius: 0.5rem;
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 20px;
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);
}

.information {
    display: flex;
    flex-direction: column;
    gap: 10px;
}

.information h1 {
    font-weight: bold;
    font-size: 1.25rem;
    text-align: center;
}

.description {
    text-align: justify;
}

.code {
    display: flex;
    padding: 0.25rem 0.5rem;
    border: 1px #1c2024 dashed;
    border-radius: 0.25rem;
    gap: 5px;
    align-items: center;
}

.copy {
    cursor: pointer;
}

.number {
    text-align: center;
    color: #1c2024;
    user-select: text;
    overflow-x: auto;
    white-space: nowrap;
    scrollbar-width: thin;
    padding: 0.2rem;
}

.form {
    display: flex;
    flex-direction: column;
    align-items: stretch;
    gap: 10px;
}

input[type='text'] {
    border: 1px solid #1c2024;
    border-radius: 0.25rem;
    padding: 0.3rem;
    text-align: center;
}

.button {
    text-align: center;
    background-color: #1c2024;
    color: white;
    padding: 0.4rem;
    border-radius: 0.25rem;
    cursor: pointer;
    font-weight: bold;
}

.snackbar {
    position: fixed;
    bottom: 20px;
    left: 50%;
    transform: translateX(-50%);
    background-color: #dc2626;
    /* merah */
    color: white;
    padding: 0.6rem 1rem;
    border-radius: 0.4rem;
    box-shadow: 0 5px 15px rgba(0, 0, 0, 0.2);
    animation: fadeInOut 3s ease forwards;
    display: flex;
    align-items: center;
    gap: 10px;
}

@keyframes fadeInOut {
    0% {
        opacity: 0;
        transform: translate(-50%, 10px);
    }

    10% {
        opacity: 1;
        transform: translate(-50%, 0);
    }

    90% {
        opacity: 1;
        transform: translate(-50%, 0);
    }

    100% {
        opacity: 0;
        transform: translate(-50%, 10px);
    }
}
</style>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import { appWindow, WebviewWindow } from '@tauri-apps/api/window';
import { emit, listen } from '@tauri-apps/api/event';

interface License {
    session_code: string
}

const informationNumber = ref('');
const tokenOrEncryptedCode = ref('');

const showSnackbar = ref(false);
const snackbarMessage = ref('');


onMounted(async () => {
    try {
        const diskId: string = await invoke('get_disk_id_windows');
        informationNumber.value = diskId;
    } catch (error) {
        console.log(error);
    }
    await listen('license-validation-success', async () => {
        await appWindow.close();
    });
})

const copyToClipboard = async () => {
    if (informationNumber.value) {
        try {
            await navigator.clipboard.writeText(informationNumber.value);
        } catch (err) {
            console.error('Gagal menyalin: ', err);
        }
    }
}

const showErrorSnackbar = (message: string) => {
    showSnackbar.value = true;
    snackbarMessage.value = message;

    setTimeout(() => {
        showSnackbar.value = false;
    }, 5000);
};

const checkEncryptedCode = async () => {
    const inputRaw = tokenOrEncryptedCode.value.trim();
    if (!inputRaw) {
        showErrorSnackbar('Silakan masukkan kode token terlebih dahulu!');
        return;
    }

    if (inputRaw.length < 32) {
        showErrorSnackbar('Format token tidak valid!');
        return;
    }

    try {
        await invoke('update_license', {
            expiredDateCode: tokenOrEncryptedCode.value.toString().trim()
        });

        const splashScreenWindow = WebviewWindow.getByLabel('splashscreen');

        if (splashScreenWindow) {
            await splashScreenWindow.show();
            await splashScreenWindow.unminimize();
            await splashScreenWindow.setFocus();

            await emit('check-license-now');
            tokenOrEncryptedCode.value = "";
        }
    } catch (error: any) {
        console.error('System Error:', error);
        showErrorSnackbar(`System Error: ${error.toString()}`);
    }
}
</script>
