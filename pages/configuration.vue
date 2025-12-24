<template>
    <div class="fixed w-screen h-screen">
        <div class="flex flex-col justify-center items-center h-screen w-screen bg-blue-300">
            <div v-show="is_notif_shown"
                class="absolute top-8 py-8 shadow-lg text-2xl font-bold text-center w-full transition ease-in-out delay-150 "
                :class="{ 'opacity-100': is_notif_shown, 'opacity-0': !is_notif_shown, 'bg-green-300': notif_status == 'success', 'bg-red-300': notif_status != 'success' }">
                <span class="text-2xl font-bold text-center w-full">Success</span>
            </div>
            <div class="flex flex-col rounded bg-blue-300 w-3/4 p-2">
                <span class="text-4xl font-bold text-center w-full mb-4">Configuration</span>
                <div class="flex w-full mb-2 items-end gap-2">
                    <div class="flex-1">
<label class="block text-gray-700 text-sm font-bold mb-2">
                           Online WebSocket Address 
                        </label>
                        <input type="text" class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" placeholder="Alamat web socket online" v-model="onlineWSIp"></input>
                    </div>
                    <button class="bg-green-500 hover:bg-green-600 active:bg-green-900 text-white font-bold py-2 px-4 rounded h-fit" @click="connectOnline">Save</button>
                </div>
                <hr class="my-1 h-px border-0 bg-blue-600">
                <div class="flex w-full mb-2">
                    <!-- <div class="w-1/3 pr-2">
                        <label class="block text-gray-700 text-sm font-bold mb-2" for="rabbitmq_host">
                            RabbbitMQ Host
                        </label>
                        <input v-model="config.rabbitmq_host"
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            id="rabbitmq_host" type="text">
                    </div> -->
                    <!-- <div class="w-1/3">
                        <label class="block text-gray-700 text-sm font-bold mb-2" for="rabbitmq_username">
                            RabbbitMQ Username
                        </label>
                        <input v-model="config.rabbitmq_username"
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            id="rabbitmq_username" type="text">
                    </div> -->
                    <!-- <div class="w-1/3 pl-2">
                        <label class="block text-gray-700 text-sm font-bold mb-2" for="rabbitmq_password">
                            RabbitMQ Password
                        </label>
                        <input v-model="config.rabbitmq_password"
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            id="rabbitmq_password" type="password">
                    </div> -->
                </div>
                <div class="flex w-full mb-2">
                    <div class="w-1/2 pr-2">
                        <label class="block text-gray-700 text-sm font-bold mb-2" for="event_id">
                            Event ID
                        </label>
                        <input v-model="config.event_id"
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            id="event_id" type="text">
                    </div>
                    <div class="w-1/2 pl-2">
                        <label class="block text-gray-700 text-sm font-bold mb-2" for="field_id">
                            Field ID
                        </label>
                        <input v-model="config.field_id"
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            id="field_id" type="text">
                    </div>
                </div>
                <div class="mb-2">
                    <label class="block text-gray-700 text-sm font-bold mb-2" for="scorer_url">
                        Scorer URL
                    </label>
                    <div class="flex justify-center items-center">
                        <input v-model="config.scorer_url"
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline"
                            id="scorer_url" type="text">
                        <button @click="preview(config.scorer_url)"
                            class="bg-blue-500 hover:bg-blue-700 text-white font-bold p-2 mb-2 rounded">Preview</button>
                    </div>
                </div>
                <div class="mb-2">
                    <label class="block text-gray-700 text-sm font-bold mb-2" for="dark_statistic_url">
                        Dark Statistic URL
                    </label>
                    <div class="flex">
                        <input v-model="config.dark_statistic_url"
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline"
                            id="dark_statistic_url" type="text">
                        <button @click="preview(config.dark_statistic_url)"
                            class="bg-blue-500 hover:bg-blue-700 text-white font-bold p-2 mb-2 rounded">Preview</button>
                    </div>
                </div>
                <div class="mb-2">
                    <label class="block text-gray-700 text-sm font-bold mb-2" for="light_statistic_url">
                        Light Statistic URL
                    </label>
                    <div class="flex">
                        <input v-model="config.light_statistic_url"
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline"
                            id="light_statistic_url" type="text">
                        <button @click="preview(config.light_statistic_url)"
                            class="bg-blue-500 hover:bg-blue-700 text-white font-bold p-2 mb-2 rounded">Preview</button>
                    </div>
                </div>
                <div class="mb-2">
                    <label class="block text-gray-700 text-sm font-bold mb-2" for="man_of_the_match_url">
                        Man of the Match URL
                    </label>
                    <div class="flex">
                        <input v-model="config.man_of_the_match_url"
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline"
                            id="man_of_the_match_url" type="text">
                        <button @click="preview(config.man_of_the_match_url)"
                            class="bg-blue-500 hover:bg-blue-700 text-white font-bold p-2 mb-2 rounded">Preview</button>
                    </div>
                </div>
                <div class="mb-2">
                    <label class="block text-gray-700 text-sm font-bold mb-2" for="top_player_url">
                        Top Player URL
                    </label>
                    <div class="flex">
                        <input v-model="config.top_player_url"
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline"
                            id="top_player_url" type="text">
                        <button @click="preview(config.top_player_url)"
                            class="bg-blue-500 hover:bg-blue-700 text-white font-bold p-2 mb-2 rounded">Preview</button>
                    </div>
                </div>
                <div class="flex items-center justify-between mt-6">
                    <button @click="saveConfig"
                        class="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                        type="button">
                        Save
                    </button>
                    <button @click="openApplication"
                        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                        type="button">
                        Done
                    </button>
                </div>
            </div>

            <div v-if="show_preview" class="fixed inset-0 flex items-center justify-center">
                <div class="absolute inset-0 bg-black bg-opacity-50"></div> <!-- Backdrop -->

                <div class="relative bg-slate-200 shadow-lg w-full max-w-2xl mx-4 my-8">
                    <span class="absolute top-0 right-0 cursor-pointer p-4" @click="show_preview = false">x</span>
                    <div class="p-4">
                        <span class="text-4xl text-center block">Preview</span>
                        <iframe class="w-full" :src="preview_url" frameborder="0"></iframe>
                    </div>
                </div>
            </div>

        </div>
    </div>
</template>

<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { emit, listen } from '@tauri-apps/api/event';
import { connectDatabaseEmulator } from 'firebase/database';

export default {
    data() {
        return {
            config: {
                rabbitmq_host: '',
                rabbitmq_username: '',
                rabbitmq_password: '',
                event_id: '',
                field_id: '',
                scorer_url: '',
                dark_statistic_url: '',
                light_statistic_url: '',
                man_of_the_match_url: '',
                top_player_url: ''
            },
            preview_url: '',
            show_preview: false,
            is_notif_shown: false,
            notif_status: '',
            onlineWSIp: '' as string,
        }
    },
    methods: {
        async saveConfig() {
            let result = await invoke('save_config', {
                rabbitmqHost: this.config.rabbitmq_host,
                rabbitmqUsername: this.config.rabbitmq_username,
                rabbitmqPassword: this.config.rabbitmq_password,
                eventId: this.config.event_id,
                fieldId: this.config.field_id,
                scorerUrl: this.config.scorer_url,
                darkStatisticUrl: this.config.dark_statistic_url,
                lightStatisticUrl: this.config.light_statistic_url,
                manOfTheMatchUrl: this.config.man_of_the_match_url,
                topPlayerUrl: this.config.top_player_url
            });

            if (!result) {
                this.showNotif('success', 'Config saved');
            } else {
                this.showNotif('failed', 'Failed to save config');
            }
        },
        async openApplication() {
            await invoke('save_config', {
                rabbitmqHost: this.config.rabbitmq_host,
                rabbitmqUsername: this.config.rabbitmq_username,
                rabbitmqPassword: this.config.rabbitmq_password,
                eventId: this.config.event_id,
                fieldId: this.config.field_id,
                scorerUrl: this.config.scorer_url,
                darkStatisticUrl: this.config.dark_statistic_url,
                lightStatisticUrl: this.config.light_statistic_url,
                manOfTheMatchUrl: this.config.man_of_the_match_url,
                topPlayerUrl: this.config.top_player_url
            });
            await emit('update_config_event', this.config);
            invoke('end_config');
        },
        async preview(url: string) {
            this.preview_url = url;
            this.show_preview = true;
        },
        async showNotif(status: string, message: string) {
            this.notif_status = status;
            this.is_notif_shown = true;
            setTimeout(() => {
                this.is_notif_shown = false;
            }, 2000);
        },
        async connectOnline() {
            if (!this.onlineWSIp.valueOf()) return;
            emit("ws:online-ip", this.onlineWSIp.valueOf());
            console.log("Kirim IP Online:", this.onlineWSIp.valueOf());
        },
    },
    async mounted() {
        this.config = await invoke('get_config');
    },
    
}
</script>

<style></style>