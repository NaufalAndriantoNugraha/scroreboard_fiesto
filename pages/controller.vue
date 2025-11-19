<template>
    <div class="fixed w-screen h-screen bg-slate-400">
        <div data-tauri-drag-region class="flex items-center justify-end h-[5%] w-full bg-slate-300 hover:cursor-move">
            <span data-tauri-drag-region class="w-full text-center text-2xl font-bold">Controller</span>
            <button class="bg-blue-500 hover:bg-blue-600 active:bg-blue-900 text-white font-bold py-2 px-4 rounded"
                @click="toggleFullscreen">o</button>
            <button class="bg-red-500 hover:bg-red-600 active:bg-red-900 text-white font-bold py-2 px-4 rounded"
                @click="closeApp">x</button>
        </div>
        <div class="flex flex-col items-center overflow-y-scroll overscroll-x-hidden p-8 h-full w-full">
            <div class="flex items-center justify-between w-full">
                <TeamController defaultname="Terang" teamname="teamA" :info="teamA"
                    class="flex flex-col items-center justify-center h-full w-1/3" />
                <div class="flex flex-col items-center justify-center h-full w-1/3">
                    <span class="text-3xl font-bold mb-3">CONTROLLER</span>
                    <div class="flex items-center justify-center py-2 mb-2">
                        <button @click="emitEvent('quarter_step_event', { step: 'down' })"
                            class="bg-red-500 hover:bg-red-600 active:bg-red-900 text-white font-bold py-2 px-4 rounded">
                            -
                        </button>
                        <span class="text-3xl font-bold mx-2 font-martianMono">Q{{ quarter }}</span>
                        <button @click="emitEvent('quarter_step_event', { step: 'up' })"
                            class="bg-blue-500 hover:bg-blue-600 active:bg-blue-900 text-white font-bold py-2 px-4 rounded">
                            +
                        </button>
                    </div>
                    <button v-if="!isRunning"
                        class="bg-blue-500 hover:bg-blue-600 active:bg-blue-900 text-white font-bold py-2 px-4 mb-2 rounded"
                        @click="startTimer">
                        Start Timer
                    </button>
                    <button v-else
                        class="bg-red-500 hover:bg-red-600 active:bg-red-900 text-white font-bold py-2 px-4 mb-2 rounded"
                        @click="stopTimer">
                        Stop Timer
                    </button>
                    <button v-if="!isTimeout"
                        class="bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 mb-2 rounded"
                        @click="startTimerTimeout">
                        Start Timeout
                    </button>
                    <button v-else class="bg-red-500 hover:bg-red-600 text-white font-bold py-2 px-4 mb-2 rounded"
                        @click="stopTimerTimeout">
                        Stop Timeout
                    </button>

                    <div class="flex flex-col items-center justify-center mt-12">
                        <span v-if="isTimeout" class="text-3xl font-bold font-martianMono mb-8">
                            {{ formatedTimeout }}
                        </span>
                        <div class="flex items-center justify-around w-full mb-2">
                            <button @click="() => { emitSumEvent('change_time_event', { value: 60 }); }"
                                class="bg-blue-500 hover:bg-blue-600 active:bg-blue-900 text-white font-bold p-2 rounded w-12 h-12">
                                +
                            </button>
                            <!-- <button @click="() => { emitSumEvent('change_time_event', { value: 10 }); }"
                                class="bg-blue-500 hover:bg-blue-600 active:bg-blue-900 text-white font-bold p-2 rounded w-12 h-12">
                                +10
                            </button> -->
                            <button @click="() => { emitSumEvent('change_time_event', { value: 1 }); }"
                                class="bg-blue-500 hover:bg-blue-600 active:bg-blue-900 text-white font-bold p-2 rounded w-12 h-12">
                                +
                            </button>
                        </div>
                        <span class="text-3xl font-bold font-martianMono">
                            {{ formatedTime }}
                        </span>
                        <div class="flex items-center justify-around w-full mt-2">
                            <button @click="() => { emitSumEvent('change_time_event', { value: -60 }); }"
                                class="bg-red-500 hover:bg-red-600 active:bg-red-900 text-white font-bold p-2 rounded w-12 h-12">
                                -
                            </button>
                            <!-- <button @click="() => { emitSumEvent('change_time_event', { value: -10 }); }"
                                class="bg-red-500 hover:bg-red-600 active:bg-red-900 text-white font-bold p-2 rounded w-12 h-12">
                                -10
                            </button> -->
                            <button @click="() => { emitSumEvent('change_time_event', { value: -1 }); }"
                                class="bg-red-500 hover:bg-red-600 active:bg-red-900 text-white font-bold p-2 rounded w-12 h-12">
                                -
                            </button>
                        </div>
                    </div>
                </div>
                <TeamController defaultname="Gelap" teamname="teamB" :info="teamB"
                    class="flex flex-col items-center justify-center h-full w-1/3" />
            </div>
            <div class="flex items-center justify-around">
                <button @click="triggerEvent('3point')"
                    class="bg-blue-500 hover:bg-blue-600 active:bg-blue-900 text-white font-bold py-2 px-4 m-2 rounded">3
                    Point</button>
                <button @click="triggerEvent('and_one')"
                    class="bg-blue-500 hover:bg-blue-600 active:bg-blue-900 text-white font-bold py-2 px-4 m-2 rounded">And
                    One</button>
            </div>
            <div class="flex items-center justify-around">
                <button @click="toggleBanner('scorer_url')"
                    class="bg-amber-500 hover:bg-amber-600 active:bg-amber-900 text-white font-bold py-2 px-4 m-2 rounded">Toggle
                    Scorer</button>
                <button @click="toggleBanner('dark_statistic_url')"
                    class="bg-amber-500 hover:bg-amber-600 active:bg-amber-900 text-white font-bold py-2 px-4 m-2 rounded">Toggle
                    Statistic
                    (Dark)</button>
                <button @click="toggleBanner('light_statistic_url')"
                    class="bg-amber-500 hover:bg-amber-600 active:bg-amber-900 text-white font-bold py-2 px-4 m-2 rounded">Toggle
                    Statistic
                    (Light)</button>
                <button @click="toggleBanner('man_of_the_match_url')"
                    class="bg-amber-500 hover:bg-amber-600 active:bg-amber-900 text-white font-bold py-2 px-4 m-2 rounded">Toggle
                    MOTM</button>
                <button @click="toggleBanner('top_player_url')"
                    class="bg-amber-500 hover:bg-amber-600 active:bg-amber-900 text-white font-bold py-2 px-4 m-2 rounded">Toggle
                    Top
                    Player</button>
                <button 
                    @click="resetTimer" 
                    class="bg-amber-500 hover:bg-amber-600 active:bg-amber-900 text-white font-bold py-2 px-4 m-2 rounded"
                    >Reset Timer
                </button>
                <button
                    @click="resetGame" 
                    class="bg-amber-500 hover:bg-amber-600 active:bg-amber-900 text-white font-bold py-2 px-4 m-2 rounded"
                >Reset Game
            </button>
            </div>
            <div class="flex justify-center items-center w-full">
              <button @click="openAdDirectory"
                class="bg-blue-500 hover:bg-blue-600 active:bg-blue-900 text-white font-bold py-2 px-4 m-2 rounded"
              >Open Ad Directory</button>
            </div>
            <div class="flex justify-center items-center w-full">
              <button v-for="dir in adDirectories"
                @click="playAdDirectory(dir)"
                class="bg-amber-500 hover:bg-amber-600 active:bg-amber-900 text-white font-bold py-2 px-4 m-2 rounded"
                >{{ dir.name }}</button>
            </div>
            <div class="flex justify-between items-center mx-10 w-full px-8">
                <Icon @click="openConfig" name="mynaui:config"
                    class="text-5xl hover:cursor-pointer bg-blue-500 hover:bg-blue-600 active:bg-blue-900 text-white font-bold p-2 m-2 rounded" />
                <div class="flex items-center">
                    <input type="text" v-model="alarmDuration"
                        class="border border-gray-400 hover:border-gray-500 py-2 px-4 m-2 w-12 rounded text-center">
                    <label for="alarmDuration" class="text-lg font-bold">Second</label>
                    <button @click="triggerAlarm" :disabled="!isSerialConnected">
                        <Icon
                            class="text-5xl hover:cursor-pointer bg-blue-500 hover:bg-blue-600 active:bg-blue-900 text-white font-bold p-2 m-2 rounded"
                            name="material-symbols:sound-detection-loud-sound" />
                    </button>
                    <select
                        class="appearance-none bg-white border border-gray-400 hover:border-gray-500 py-2 px-4 m-2 rounded focus:outline-none focus:shadow-outline"
                        v-model="selectedPort" :disabled="isSerialConnected">
                        <option value="">PORT?</option>
                        <option v-for="port in serialPorts" :value="port">{{ port }}</option>
                    </select>
                    <button @click="fetchSerialPorts"
                        class="bg-blue-500 hover:bg-blue-600 active:bg-blue-900 text-white font-bold py-2 px-4 m-2 rounded">Refresh</button>
                    <button v-if="!isSerialConnected" @click="serialConnect"
                        class="bg-blue-500 hover:bg-blue-600 active:bg-blue-900 text-white font-bold py-2 px-4 m-2 rounded">Connect</button>
                    <button v-else @click="serialDisconnect"
                        class="bg-red-500 hover:bg-red-600 active:bg-red-900 text-white font-bold py-2 px-4 m-2 rounded">Disconnect</button>
                </div>
            </div>
            <div v-show="isNotifShown"
                class="absolute top-8 py-8 shadow-lg text-2xl font-bold text-center w-full transition ease-in-out delay-150 "
                :class="{ 'opacity-100': isNotifShown, 'opacity-0': !isNotifShown, 'bg-green-300': notificationStatus == 'success', 'bg-red-300': notificationStatus != 'success' }">
                <span class="text-2xl font-bold text-center w-full">{{ notificationMessage }}</span>
            </div>
        </div>
    </div>
</template>

<script lang="ts">
import { emit, listen } from '@tauri-apps/api/event';
import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
import type { TeamInfo } from '~/types/TeamInfo';
import { getCurrent } from '@tauri-apps/api/window';
import { readDir, type FileEntry } from '@tauri-apps/api/fs';
import { ref } from 'vue'

import { open } from '@tauri-apps/api/dialog';
import { appDataDir } from '@tauri-apps/api/path';


type PreviewUrl = {
    'scorer_url': string,
    'dark_statistic_url': string,
    'light_statistic_url': string,
    'man_of_the_match_url': string,
    'top_player_url': string
}

export default {
    setup() {
      const adDirectories = ref<FileEntry[]>([]);

      async function openAdDirectory() {
        const dir = await open({
          directory: true,
          multiple: false,
          
          defaultPath: await appDataDir()
        })

        if (dir === null) return;
        
        adDirectories.value = (await readDir(dir as string)).filter(e => e.children !== undefined);
      }

      async function playAdDirectory(dir: FileEntry) {
        // invoke event to rust backend 
      }

      return {
        adDirectories,
        openAdDirectory
      }
    },
    data() {
        return {
            time: 0 as number,
            lastTimeUpdate: null as number | null,
            isRunning: false as boolean,
            timeout: 0 as number,
            lastTimeoutUpdate: null as number | null,
            isTimeout: false as boolean,
            minimumUpdatePeriod: 200 as number,
            teamA: {
                name: 'Terang',
                picture: '',
                score: 0,
                foul: 0,
                timeout: 0
            } as TeamInfo,
            teamB: {
                name: 'Gelap',
                picture: '',
                score: 0,
                foul: 0,
                timeout: 0
            } as TeamInfo,
            quarter: 1 as number,
            previewUrl: '' as string,
            isBannerShown: false as boolean,
            showingBanner: '' as string,
            listUrl: {
            } as PreviewUrl,
            alarmDuration: '5' as string,
            serialPorts: [] as string[],
            selectedPort: '' as string,
            isSerialConnecting: false as boolean,
            isSerialConnected: false as boolean,
            isNotifShown: false as boolean,
            notificationStatus: '' as 'success' | 'failed',
            notificationMessage: '' as string
        }
    },
    mounted() {
        this.fetchSerialPorts();

        listen('timer_event', (event: any) => {
            this.time = event.payload.value;
            this.isRunning = true;
            this.lastTimeUpdate = Date.now();
        });

        listen('timeout_event', (event: any) => {
            // console.log(event.payload.value)
            this.timeout = event.payload.value;
            this.isTimeout = true;
            this.lastTimeoutUpdate = Date.now();
        });

        listen('team_a_event', (event: any) => {
            this.teamA = event.payload.teamA
            if (this.teamA.name == '') {
                this.teamA.name = 'Gelap';
            }
        });

        listen('team_b_event', (event: any) => {
            this.teamB = event.payload.teamB
            if (this.teamB.name == '') {
                this.teamB.name = 'Terang';
            }
        });

        listen('quarter_event', (event: any) => {
            console.log(event.payload.quarter)
            invoke('update_quarter', { quarter: `${event.payload.quarter}` })
            this.quarter = event.payload.quarter;
        });

        listen('timer_stop_event', (event: any) => {
            this.isRunning = false;
            this.triggerAlarm();
        });

        listen('update_config_event', (event: any) => {
            this.getConfig();
        });

        this.getConfig();

        // Periodically check if data is still coming
        setInterval(() => {
            if (this.lastTimeUpdate && (Date.now() - this.lastTimeUpdate > this.minimumUpdatePeriod)) {
                this.isRunning = false;
                this.lastTimeUpdate = null;
            }
            if (this.lastTimeoutUpdate && (Date.now() - this.lastTimeoutUpdate > this.minimumUpdatePeriod)) {
                this.isTimeout = false;
                this.lastTimeoutUpdate = null;
            }
        }, this.minimumUpdatePeriod);
    },
    computed: {
        formatedTime() {
            const milliseconds = (this.time % 1000) / 10;
            const seconds = Math.floor(this.time / 1000) % 60;
            const minutes = Math.floor(this.time / (1000 * 60)) % 60;

            const strMinutes = String((minutes < 10) ? "0" + minutes.toFixed(0) : minutes.toFixed(0));
            const strSeconds = String((seconds < 10) ? "0" + seconds.toFixed(0) : seconds.toFixed(0));
            const strMilliseconds = String((milliseconds < 10) ? "0" + milliseconds.toFixed(0) : milliseconds.toFixed(0));

            return strMinutes + ":" + strSeconds + "." + strMilliseconds;
        },
        formatedTimeout() {
            const milliseconds = (this.timeout % 1000) / 10;
            const seconds = Math.floor(this.timeout / 1000) % 60;
            const minutes = Math.floor(this.timeout / (1000 * 60)) % 60;

            const strMinutes = String((minutes < 10) ? "0" + minutes.toFixed(0) : minutes.toFixed(0));
            const strSeconds = String((seconds < 10) ? "0" + seconds.toFixed(0) : seconds.toFixed(0));
            const strMilliseconds = String((milliseconds < 10) ? "0" + milliseconds.toFixed(0) : milliseconds.toFixed(0));

            return strSeconds;
        },
        windowName() {
            const window = getCurrent();
            const windowName = window.label;
            return windowName;
        }
    },
    methods: {
        async getConfig() {
            invoke('get_config').then((state: any) => {
                this.listUrl = state;
            })
        },
        async startTimer() {
            if (this.time > 0) {
                emit('start_timer_event', { initialTime: this.time })
            } else {
                emit('start_timer_event', { initialTime: 600000 })
            }
        },
        async openConfig() {
            invoke('open_config');
        },
        async triggerAlarm() {
            console.log('trigger alarm');
            invoke('trigger_alarm', { portName: this.selectedPort, duration: this.alarmDuration });
        },
        async stopTimer() {
            emit('stop_timer_event');
        },
        async startTimerTimeout() {
            emit('start_timeout_event', { initialTime: 60000 });
        },
        async stopTimerTimeout() {
            emit('stop_timeout_event');
        },
        async toggleBanner(key: "scorer_url" | "dark_statistic_url" | "light_statistic_url" | "man_of_the_match_url" | "top_player_url") {
            if (this.showingBanner == key) {
                emit('hide_banner', {});
                this.showingBanner = '';
            } else {
                emit('show_banner', { url: this.listUrl[key] });
                this.showingBanner = key;

            }
        },
        async triggerEvent(event_name: string) {
            emit(`${event_name}_event`, {});
        },
        async toggleFullscreen() {
            invoke('toggle_fullscreen');
        },
        // async updateQuarter() {
        //     emit('quarter_event', { quarter: this.quarter });
        // },
        async fetchSerialPorts() {
            try {
                const ports = await invoke('list_serial_ports') as string[];
                this.serialPorts = ports;
            } catch (error) {
                console.error('Failed to fetch serial ports:', error);
            }
        },
        async serialConnect() {
            this.isSerialConnecting = true;
            let status = await invoke('connect_serial_port', { portName: this.selectedPort })
                .then(response => {
                    // Success notification
                    this.isSerialConnected = true;
                    this.showNotif('success', 'Serial port connected');
                })
                .catch(error => {
                    // Error notification
                    this.showNotif('failed', 'Failed to connect serial port');
                })
                .finally(() => {
                    this.isSerialConnecting = false;
                });
            console.log(status);
        },
        async serialDisconnect() {
            let status = await invoke('disconnect_serial_port')
                .then(response => {
                    // Success notification
                    console.log(response);
                    this.isSerialConnected = false;
                    this.showNotif('success', 'Serial port disconnected');
                })
                .catch(error => {
                    // Error notification
                    console.log(error);
                    this.showNotif('failed', 'Failed to disconnect serial port');
                });
            console.log(status);
        },
        async emitSumEvent(event: string, data: any) {
            console.log(data);
            this.time += (data.value * 1000);
            emit(event, data);
        },
        async emitEvent(event: string, data: any) {
            emit(event, data);
        },
        async showNotif(status: 'success' | 'failed', message: string) {
            this.notificationStatus = status;
            this.notificationMessage = message;
            this.isNotifShown = true;
            setTimeout(() => {
                this.isNotifShown = false;
            }, 2000);
        },
        async closeApp() {
            invoke('close_all_processes');
        },

        async resetTimer() {
            this.time = 0;
            emit('reset_timer_event');
        },

        async resetGame() {
            this.emitEvent('foul_step_event', {step: "reset"});
            this.emitEvent('score_step_event', {step: "reset"});
            this.emitEvent('timeout_step_event', {step: "reset"});
            this.emitEvent('quarter_step_event', {step: 'reset'});
            this.resetTimer();
        }        
    }
}
</script>

<style></style>