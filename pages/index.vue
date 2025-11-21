<template>
  <div
    class="fixed container-body font-basementGrotesque bg-[url('~/assets/img/mock.jpg')]"
  >
    <div
      data-tauri-drag-region
      class="flex items-center justify-end h-[5%] w-full bg-slate-300 hover:cursor-move"
      id="handle"
    >
      <span data-tauri-drag-region class="w-full text-center text-2xl font-bold"
        >Controller</span
      >
      <button
        class="bg-blue-500 hover:bg-blue-600 active:bg-blue-900 text-white font-bold py-2 px-4 rounded"
        @click="toggleFullscreen"
      >
        o
      </button>
    </div>
    <div class="bg-iframe">
      <!-- <iframe width="560" height="315" src="https://www.youtube.com/embed/Ps-0f0K6izM?si=mj9tm8_keiaPgwZC&autoplay=1&controls=0&loop=1&showinfo=0" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe> -->
      <video width="320" height="240" autoplay loop muted id="bgVideo">
        <source src="../assets/video/bg-video.mp4" type="video/mp4" />
        Your browser does not support the video tag.
      </video>
    </div>
    <div class="wrapper-score">
      <img src="../assets//img/light-slim.png" alt="" class="light-left" />
      <img src="../assets//img/light-slim.png" alt="" class="light-right" />
      <div class="header-score">
        <div class="bg-header"></div>
        <div class="logo-header">
          <img src="../assets/img/logo.svg" />
        </div>
        <div class="label-header font-roboto-condensed">
          <div class="left-label label-score">
            <div>
              {{ teamA.name != '' ? teamA.name : 'Terang' }}
            </div>
          </div>
          <div class="right-label label-score">
            <div>{{ teamB.name != '' ? teamB.name : 'Gelap' }}</div>
          </div>
        </div>
      </div>
      <div class="body-score">
        <!-- <img src="../assets//img/light-big.png" alt="" class="light-big" /> -->
        <div class="score-block">
          <div class="left-score score-count">
            <div>{{ teamA.score }}</div>
            <img
              src="../assets//img/light-slim.png"
              alt=""
              class="light-score"
            />
          </div>
          <div class="right-score score-count">
            <div>{{ teamB.score }}</div>
            <img
              src="../assets//img/light-slim.png"
              alt=""
              class="light-score"
            />
          </div>
        </div>
      </div>
      <div class="footer-score">
        <div class="foul-team left-foul">
          <div>
            <h3>TEAM FOUL</h3>
            <div class="foul-count">{{ teamA.foul }}</div>
          </div>
          <div class="timout-count">
            <div
              v-for="n in 3"
              :class="n <= teamA.timeout ? 'active' : ''"
              class="list-timout"
            ></div>
          </div>
        </div>
        <div class="timer font-noto">
          <div v-if="!isTimeout" class="timer quarter font-basementGrotesque">
            <div class="label-quarter">QUARTER</div>
            {{ quarter }}
          </div>
          <div v-else class="timer quarter font-basementGrotesque">
            <div class="label-quarter">TIMEOUT</div>
            {{ formatedTimeout }}
          </div>
          {{ formatedTime }}
        </div>
        <div class="foul-team right-foul">
          <div class="timout-count">
            <div
              v-for="n in 3"
              :class="n <= teamB.timeout ? 'active' : ''"
              class="list-timout"
            ></div>
          </div>
          <div>
            <h3>TEAM FOUL</h3>
            <div class="foul-count">{{ teamB.foul }}</div>
          </div>
        </div>
      </div>
    </div>
    <!-- <div class="flex items-center justify-center w-screen h-screen bg-gray-700">
            <div class="flex items-center justify-center h-screen w-2/5">
                <TeamInfo name="Team 1" :info="teamA" />
            </div>
            <div class="flex flex-col items-center justify-center w-1/5">
                <div class="flex items-center justify-center bg-slate-300 rounded-lg px-8 py-4">
                    <span v-show="quarterName !== ''" class="text-4xl font-bold">
                        {{ quarterName }}
                    </span>
                </div>
                <span class="text-8xl font-bold">
                    {{ formatedTime }}
                </span>
                <span v-if="isTimeout" class="text-6xl font-bold text-red-400">
                    {{ formatedTimeout }}
                </span>
            </div>
            <div class="flex items-center justify-center h-screen w-2/5">
                <TeamInfo name="Team 2" :info="teamB" />
            </div>
        </div> -->
    <div
      :class="{
        'layer-active opacity-100 layer-popup': isBannerShown,
        'opacity-0 layer-popup': !isBannerShown,
      }"
      class="fixed inset-0 w-screen h-screen flex items-center justify-center bg-opacity-0"
    >
      <div class="relativeshadow-lg w-screen">
        <span class="absolute cursor-pointer p-4" @click="isBannerShown = false"
          >x</span
        >
        <div>
          <iframe
            class="w-screen h-screen"
            :src="previewUrl"
            frameborder="0"
          ></iframe>
        </div>
      </div>
    </div>

    <div
      v-show="is3Point"
      class="flex justify-center items-center h-screen bg-black"
      style="z-index: 100"
    >
      <video
        ref="threePointPlayer"
        class="w-full h-full"
        controls
        @click="toggleFullscreen"
      >
        <source src="../assets/video/3point.mp4" type="video/mp4" />
        Your browser does not support the video tag.
      </video>
    </div>

    <div
      v-show="isAndOne"
      class="flex justify-center items-center h-screen bg-black"
      style="z-index: 100"
    >
      <video
        ref="andOnePlayer"
        class="w-full h-full"
        controls
        @click="toggleFullscreen"
      >
        <source src="../assets/video/and_one.mp4" type="video/mp4" />
        Your browser does not support the video tag.
      </video>
    </div>
  </div>
  <!-- Ad Video Player Overlay -->
<div
    v-show="isVideoPlaying"
    class="fixed inset-0 flex justify-center items-center bg-black z-50"
>
    <video
        ref="adVideoPlayer"
        class="w-full h-full"
        @ended="stopVideo"
        @click="stopVideo"
    >
        Your browser does not support the video tag.
    </video>
    <button
        @click="stopVideo"
        class="absolute top-4 left-4 font-bold py-2 px-4 rounded text-transparent"
    >
        Close (X)
    </button>
</div>
</template>

<script lang="ts">
import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
import { emit, listen } from '@tauri-apps/api/event';
import type { TeamInfo } from '~/types/TeamInfo';
import { doc, updateDoc } from 'firebase/firestore';
import { event } from '@tauri-apps/api';
import { readDir } from '@tauri-apps/api/fs';

let ws: WebSocket | null = null;

function initWS() {
  ws = new WebSocket("ws://192.168.18.251:8070");

  ws.onopen = () => console.log("WS Connected");
  ws.onclose = () => console.log("WS Disconnected");
}

function sendWS(data: any) {
  if (ws && ws.readyState === WebSocket.OPEN) {
    ws.send(JSON.stringify(data));
  }
}

export default {
  data() {
    return {
      time: 0 as number,
      timeout: 0 as number,
      timer: null as any,
      timerTimeout: null as any,
      timerUpdateCounter: 0 as number,
      timeoutUpdateCounter: 0 as number,
      isRunning: false as boolean,
      isTimeout: false as boolean,
      isBannerShown: false as boolean,
      previewUrl: '' as string,
      is3Point: false as boolean,
      isAndOne: false as boolean,
      quarter: 0 as number,
      isVideoPlaying: false as boolean,
      currentVideoSrc: '' as string,
      currentVideoIndex: 0 as number,
      videoPlaylist: [] as string[],
      teamA: {
        name: '',
        picture: '',
        score: 0,
        foul: 0,
        timeout: 0,
      } as TeamInfo,
      teamB: {
        name: '',
        picture: '',
        score: 0,
        foul: 0,
        timeout: 0,
      } as TeamInfo,
    };
  },
  async mounted() {
    initWS();

    await listen('start_timer_event', (event: any) => {
      this.startTimer(event.payload.initialTime);
      this.broadcastState();
    });
    await listen('reset_timer_event', (event: any) => {
      this.resetTimer();
      this.broadcastState();
    });
    await listen('stop_timer_event', (event: any) => {
      this.stopTimer();
      this.broadcastState();
    });
    await listen('start_timeout_event', (event: any) => {
      this.startTimeout(event.payload.team, event.payload.initialTime);
      this.broadcastState();
    });
    await listen('stop_timeout_event', (event: any) => {
      this.stopTimeout();
      this.broadcastState();
    });
    await listen('show_banner', (event: any) => {
      this.showBanner(event.payload.url);
    });
    await listen('hide_banner', (event: any) => {
      this.hideBanner();
    });
    await listen('quarter_event', (event: any) => {
      this.quarter = event.payload.quarter;
      invoke('update_quarter', { quarter: this.quarter });
      this.broadcastState();
    });

    await listen('quarter_step_event', (event: any) => {
      switch (event.payload.step) {
        case 'up':
          this.quarter += 1;
          this.broadcastState();
          break;

        case 'down':
          this.quarter = this.quarter - 1 < 0 ? 0 : this.quarter - 1;
          this.broadcastState();
          break;

        case 'reset':
          this.quarter = 1;
          this.broadcastState();
          break;

        default:
          break;
      }
    });

    await listen('change_time_event', (event: any) => {
      this.time = this.time + event.payload.value * 1000;
      this.broadcastState();
    });

    await listen('team_name_event', (event: any) => {
      switch (event.payload.team) {
        case 'teamA':
          this.teamA.name = event.payload.name;
          this.broadcastState();
          break;

        case 'teamB':
          this.teamB.name = event.payload.name;
          this.broadcastState();
          break;

        default:
          break;
      }
    });

    await listen('3point_event', (event: any) => {
      this.is3Point = true;
      const videoElement = this.$refs.threePointPlayer as HTMLVideoElement;
      if (videoElement) {
        videoElement.play().catch((error) => {
          console.error('Error attempting to play video:', error);
        });
        setTimeout(() => {
          this.is3Point = false;
        }, 3000);
      }
    });

    await listen('and_one_event', (event: any) => {
      this.isAndOne = true;
      const videoElement = this.$refs.andOnePlayer as HTMLVideoElement;
      if (videoElement) {
        videoElement.play().catch((error) => {
          console.error('Error attempting to play video:', error);
        });
        setTimeout(() => {
          this.isAndOne = false;
        }, 3000);
      }
    });

    await listen('score_step_event', (event: any) => {
      switch (event.payload.step) {
        case 'up':
          switch (event.payload.team) {
            case 'teamA':
              this.teamA.score += 1;
              this.broadcastState();
              break;

            case 'teamB':
              this.teamB.score += 1;
              this.broadcastState();
              break;

            default:
              break;
          }
          break;

        case 'down':
          switch (event.payload.team) {
            case 'teamA':
              this.teamA.score =
                this.teamA.score - 1 < 0 ? 0 : this.teamA.score - 1;

                this.broadcastState();
              break;

            case 'teamB':
              this.teamB.score =
                this.teamB.score - 1 < 0 ? 0 : this.teamB.score - 1;

              this.broadcastState();
              break;

            default:
              break;
          }
          break;

        case 'reset':
          this.teamA.score = 0;
          this.teamB.score = 0;
          this.broadcastState();
          break;

        default:
          break;
      }
    });
    await listen('foul_step_event', (event: any) => {
      switch (event.payload.step) {
        case 'up':
          switch (event.payload.team) {
            case 'teamA':
              this.teamA.foul += 1;
              this.broadcastState();
              break;

            case 'teamB':
              this.teamB.foul += 1;
              this.broadcastState();
              break;

            default:
              break;
          }
          break;

        case 'down':
          switch (event.payload.team) {
            case 'teamA':
              this.teamA.foul =
                this.teamA.foul - 1 < 0 ? 0 : this.teamA.foul - 1;

              this.broadcastState();
              break;

            case 'teamB':
              this.teamB.foul =
                this.teamB.foul - 1 < 0 ? 0 : this.teamB.foul - 1;

              this.broadcastState();
              break;

            default:
              break;
          }
          break;

        case 'reset':
          this.teamA.foul = 0;
          this.teamB.foul = 0;
          this.broadcastState();
          break;

        default:
          break;
      }
    });
    await listen('timeout_step_event', (event: any) => {
      switch (event.payload.step) {
        case 'up':
          switch (event.payload.team) {
            case 'teamA':
              this.teamA.timeout =
                this.teamA.timeout + 1 > 3 ? 3 : this.teamA.timeout + 1;

              this.broadcastState();
              break;

            case 'teamB':
              this.teamB.timeout =
                this.teamB.timeout + 1 > 3 ? 3 : this.teamB.timeout + 1;

              this.broadcastState();
              break;

            default:
              break;
          }
          break;

        case 'down':
          switch (event.payload.team) {
            case 'teamA':
              this.teamA.timeout =
                this.teamA.timeout - 1 < 0 ? 0 : this.teamA.timeout - 1;

              this.broadcastState();
              break;

            case 'teamB':
              this.teamB.timeout =
                this.teamB.timeout - 1 < 0 ? 0 : this.teamB.timeout - 1;

              this.broadcastState();
              break;

            default:
              break;
          }
          break;

        case 'reset':
          this.teamA.timeout = 0;
          this.teamB.timeout = 0;
          this.broadcastState();
          break;

        default:
          break;
      }
    });
    await listen('play_ad_directory', async (event: any) => {
        const dirPath = event.payload.path;
        const dirName = event.payload.name;
        
        console.log('Received play_ad_directory event:', dirPath);
        
        this.playVideosFromDirectory(dirPath);
    });
  },
  watch: {
    teamA: {
      handler(newVal, oldVal) {
        console.log(newVal, oldVal);
        emit('team_a_event', {
          teamA: {
            name: this.teamA.name,
            score: this.teamA.score,
            foul: this.teamA.foul,
            timeout: this.teamA.timeout,
          },
        });
      },
      deep: true,
    },
    teamB: {
      handler(newVal, oldVal) {
        console.log(newVal, oldVal);
        emit('team_b_event', {
          teamB: {
            name: this.teamB.name,
            score: this.teamB.score,
            foul: this.teamB.foul,
            timeout: this.teamB.timeout,
          },
        });
      },
      deep: true,
    },
    quarter: {
      handler(newVal, oldVal) {
        console.log(newVal, oldVal);
        emit('quarter_event', {
          quarter: this.quarter,
        });
      },
    },
  },
  computed: {
    formatedTime() {
      if (this.time > 60000) {
        const milliseconds = (this.time % 1000) / 10;
        const seconds = Math.floor(this.time / 1000) % 60;
        const minutes = Math.floor(this.time / (1000 * 60)) % 60;

        const strMinutes = String(
          minutes < 10 ? '0' + minutes.toFixed(0) : minutes.toFixed(0),
        );
        const strSeconds = String(
          seconds < 10 ? '0' + seconds.toFixed(0) : seconds.toFixed(0),
        );
        const strMilliseconds = String(
          milliseconds < 10
            ? '0' + milliseconds.toFixed(0)
            : milliseconds.toFixed(0),
        );

        return strMinutes + ':' + strSeconds; // + "." + strMilliseconds;
      } else {
        const milliseconds = (this.time % 1000) / 10;
        const seconds = Math.floor(this.time / 1000) % 60;
        const minutes = Math.floor(this.time / (1000 * 60)) % 60;
        return `${seconds}.${milliseconds.toFixed(0)}`; //minutes + ":" + seconds;
      }
    },
    formatedTimeout() {
      const milliseconds = (this.timeout % 1000) / 10;
      const seconds = Math.floor(this.timeout / 1000) % 60;
      const minutes = Math.floor(this.timeout / (1000 * 60)) % 60;

      const strMinutes = String(
        minutes < 10 ? '0' + minutes.toFixed(0) : minutes.toFixed(0),
      );
      const strSeconds = String(
        seconds < 10 ? '0' + seconds.toFixed(0) : seconds.toFixed(0),
      );
      const strMilliseconds = String(
        milliseconds < 10
          ? '0' + milliseconds.toFixed(0)
          : milliseconds.toFixed(0),
      );

      return strSeconds;
    },
    quarterName() {
      switch (String(this.quarter)) {
        case '1':
          return 'Q1';
        case '2':
          return 'Q2';
        case '3':
          return 'Q3';
        case '4':
          return 'Q4';
        case '5':
          return 'OT';
        default:
          return '';
      }
    },
  },
  methods: {
        toggleFullscreen() {
      invoke('toggle_fullscreen');
    },
    startTimer(initialTime: number = 600000) {
      // const adsWindow = new WebviewWindow('ads');
      // adsWindow.show();
      console.log(initialTime);
      if (!this.isRunning) {
        this.isRunning = true;
        this.time = initialTime;

        this.timer = setInterval(() => {
          this.timerUpdateCounter += 1;
          if (this.timerUpdateCounter >= 10) {
            this.timerUpdateCounter = 0;
            this.broadcastState();
            invoke('update_time', { time: this.formatedTime });
            emit('timer_event', { value: this.time });
          }
          this.time -= 10; // Increment every 10 milliseconds
          if (this.time <= 0) {
            this.time = 0;
            this.stopTimer();
            this.broadcastState();
            emit('timer_event', { value: this.time });
            emit('timer_stop_event');
          }
        }, 10);
      }
    },

    resetTimer() {
      this.time = 0;
      this.stopTimer();
      invoke('update_time', { time: this.formatedTime });
    },
    stopTimer() {
      this.isRunning = false;
      clearInterval(this.timer);
    },
    startTimeout(team: String, initialTime: number = 60000) {
      console.log(team);
      if (!this.isTimeout) {
        this.isTimeout = true;
        this.timeout = initialTime;
        this.timerTimeout = setInterval(() => {
          this.timeoutUpdateCounter += 1;
          if (this.timeoutUpdateCounter >= 10) {
            this.timeoutUpdateCounter = 0;
            emit('timeout_event', { value: this.timeout });
            this.broadcastState();
          }
          this.timeout -= 10;
          if (this.timeout <= 0) {
            this.stopTimeout();
            emit('timeout_event', { value: this.timeout });
            this.broadcastState();
          }
        }, 10);
      }
    },
    stopTimeout() {
      this.isTimeout = false;
      clearInterval(this.timerTimeout);
    },
    showBanner(url: string) {
      this.previewUrl = url;
      this.isBannerShown = true;
      // setTimeout(() => {
      //     this.isBannerShown = false;
      // }, 3000);
    },
    hideBanner() {
      this.isBannerShown = false;
    },
    async sendToFirebase() {
      const { $firestore: firestore } = useNuxtApp();

      // Reference to a document in Firestore
      const docRef = doc(firestore, 'scoreboard_timer', 'stream1');

      try {
        await updateDoc(docRef, {
          minutes: this.formatedTime, // Fields to update
          // ... more fields to update
        });
        console.log('Document updated successfully');
      } catch (error) {
        console.error('Error updating document: ', error);
      }
    },
    async playVideosFromDirectory(dirPath: string) {
        try {
            // Baca semua file di directory
            const files = await readDir(dirPath);
            
            // Filter hanya file .mp4
            const videoFiles = files.filter(file => 
                file.name?.toLowerCase().endsWith('.mp4')
            );
            
            if (videoFiles.length === 0) {
                console.error('No MP4 files found in directory');
                return;
            }
            
            console.log('Found videos:', videoFiles.map(f => f.name));
            
            // Simpan playlist
            this.videoPlaylist = videoFiles.map(f => f.path);
            this.currentVideoIndex = 0;
            
            // Play video pertama
            this.playVideo(this.videoPlaylist[0]);
            
        } catch (error) {
            console.error('Error reading directory:', error);
        }
    },
    
    async playVideo(videoPath: string) {
        // Convert path ke format yang bisa diakses browser
        const videoSrc = convertFileSrc(videoPath);
        
        console.log('Playing video:', videoSrc);
        
        // Show video overlay
        this.isVideoPlaying = true;
        this.currentVideoSrc = videoSrc;
        
        // Wait for DOM update
        await this.$nextTick();
        
        const videoElement = this.$refs.adVideoPlayer as HTMLVideoElement;
        if (videoElement) {
            videoElement.src = videoSrc;
            videoElement.play().catch(error => {
                console.error('Error playing video:', error);
            });
            videoElement.onended = () => {
                this.playNextVideo();
            };
        }
    },
    playNextVideo() {
        this.currentVideoIndex++;
        
        if (this.currentVideoIndex < this.videoPlaylist.length) {
            // Play next video
            this.playVideo(this.videoPlaylist[this.currentVideoIndex]);
        } else {
            // Semua video sudah selesai, tutup player
            this.stopVideo();
        }
    },
     broadcastState() {
      sendWS({
        teamAName: this.teamA.name,
        teamAScore: this.teamA.score,
        teamAFoul: this.teamA.foul,
        teamATimeOut: this.teamA.timeout,

        teamBName: this.teamB.name,
        teamBScore: this.teamB.score,
        teamBFoul: this.teamB.foul,
        teamBTimeOut: this.teamB.timeout,

        timer: this.formatedTime,
        timeout: this.formatedTimeout,
        quarter: this.quarter,
    });
  },
    stopVideo() {
        this.isVideoPlaying = false;
        const videoElement = this.$refs.adVideoPlayer as HTMLVideoElement;
        if (videoElement) {
            videoElement.pause();
            videoElement.src = '';
        }
    },
  },
}
</script>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=Noto+Sans:ital,wght@0,100..900;1,100..900&display=swap');

@import url('https://fonts.googleapis.com/css2?family=Roboto+Condensed:ital,wght@0,100..900;1,100..900&display=swap');

/* :root {
    --light1: #67bbf7;
    --light2: #1c448c;
    --light3: #183a77;
    --lightcolor: #fff;
    --dark1: #d92933;
    --dark2: #8a1b2b;
    --dark3: #771725;
    --darkcolor: #fff;
}

@font-face {
    font-family: BasementGrotesque;
    src: url("BasementGrotesque.otf") format("opentype");
} */
.font-noto {
  font-family: 'Noto Sans', sans-serif;
  font-weight: 900;
}
.font-roboto-condensed {
  font-family: 'Roboto Condensed', sans-serif;
  font-weight: bolder !important;
}
div#handle button {
  opacity: 0;
}

.layer-popup {
  transition: all 0.5s ease-out;
}

div#handle button:hover {
  opacity: 1;
}

div#handle {
  position: absolute;
  width: 100%;
  top: 0;
  background: transparent;
  z-index: 2;
  color: transparent;
}

.container-body {
  margin: 0;
  background: url(../assets/img/background.jpg);
  background-size: 100% auto;
  font-family: BasementGrotesque;
  height: 100vh;
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.logo-header img {
  max-width: 100%;
  display: block;
}

div.wrapper-score:before {
  content: '';
  display: block;
  position: absolute;
  width: calc(100% - 24px);
  height: calc(100% - 24px);
  background: #202538;
  top: 12px;
  left: 11px;
  border-radius: 20px;
}

div.wrapper-score {
  width: 1005px;
  margin: 0 auto;
  background: cadetblue;
  background: linear-gradient(143deg, #ffffff, #acabaa);
  border-image-slice: 1;
  border-radius: 30px;
  position: relative;
  margin-top: 60px;
}

@media (min-width: 1700px) {
  div.wrapper-score {
    transform: scale(1.3);
  }
}

.logo-header img {
  width: 100%;
  height: 100%;
  object-fit: fill;
}

.logo-header {
  width: 125px;
  height: 125px;
  margin: 0 auto;
  background: #202539;
  border-radius: 100%;
  padding: 10px;
  top: -64px;
  position: relative;
  z-index: 3;
  border: 5px solid #fff;
}

.header-score:after {
  content: '';
  display: block;
  position: absolute;
  width: 60%;
  height: 100%;
  right: -40px;
  top: 0;
  background: linear-gradient(
    180deg,
    rgba(254, 255, 254, 1) 25%,
    rgba(164, 164, 164, 1) 52%,
    rgb(147 147 147) 62%,
    rgb(199 199 199) 91%
  );
  transform: skew(-8deg);
  border-radius: 5px 5px 15px 5px;
}

.header-score:before {
  content: '';
  display: block;
  position: absolute;
  width: 60%;
  height: 100%;
  left: -40px;
  top: 0;
  background: linear-gradient(
    180deg,
    rgba(254, 255, 254, 1) 25%,
    rgba(164, 164, 164, 1) 52%,
    rgb(147 147 147) 62%,
    rgb(199 199 199) 91%
  );
  transform: skew(8deg);
  border-radius: 5px 5px 5px 15px;
}

.header-score {
  position: relative;
  height: 110px;
  background: #fff;
}

.label-score {
  width: 44%;
  background: linear-gradient(180deg, var(--light1) 20%, var(--light2) 96%);
  color: var(--lightcolor);
  text-transform: uppercase;
  letter-spacing: 2px;
  text-align: center;
  border-radius: 10px 10px 0 0;
  padding: 0;
  font-size: 40pt;
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  height: 73px;
  padding-top: 17px;
  line-height: normal;
}

.label-score div {
  height: auto;
  overflow: hidden;
  display: flex;
  width: 100%;
  padding: 0 10px 0;
  display: flex;
  align-items: start;
  justify-content: center;
  max-height: 100%;
  box-sizing: border-box;
}

.label-score:after {
  content: '';
  display: block;
  position: absolute;
  width: calc(100% + 12px);
  background: var(--light2);
  height: 20px;
  left: 0;
  top: 100%;
  margin-top: -1px;
  border-radius: 0 0 0px 10px;
}

.label-score:before {
  content: '';
  display: block;
  position: absolute;
  left: 100%;
  background: var(--light3);
  height: 25px;
  width: 28px;
  top: 100%;
  border-radius: 0 0 100px 100px;
  z-index: 1;
  margin-top: -6px;
  margin-bottom: 5px;
}

.right-label.label-score {
  background: linear-gradient(180deg, var(--dark1) 20%, var(--dark2) 96%);
  color: var(--darkcolor);
}

.right-label.label-score:before {
  left: auto;
  background: var(--dark3);
  right: 100%;
}

.right-label.label-score:after {
  background: var(--dark2);
  border-radius: 0 0 10px 0px;
  left: auto;
  right: 0;
}

.label-header {
  display: flex;
  justify-content: space-around;
  align-items: center;
  position: absolute;
  width: 100%;
  bottom: -6px;
  z-index: 1;
}

.body-score {
  padding-top: 50px;
}

.score-block {
  position: relative;
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: space-around;
}

.score-count div {
  position: relative;
  transform: skew(13deg);
}

.score-count {
  background: linear-gradient(143deg, #ffffff, #dededc);
  width: 38%;
  position: relative;
  color: var(--lightcolor);
  font-size: 110pt;
  letter-spacing: 2px;
  text-align: center;
  padding: 40px 0 10px;
  transform: skew(-13deg);
  border-radius: 18px;
  /* margin-left: 40px; */
}

.score-count:before {
  content: '';
  display: block;
  position: absolute;
  width: calc(100% - 20px);
  top: 10px;
  border-radius: 12px;
  height: calc(100% - 20px);
  left: 10px;
  background: linear-gradient(180deg, var(--light1) 20%, var(--light2) 60%);
}

.right-score.score-count {
  transform: skew(13deg);
  /* margin-right: 40px; */
  /* margin-left: 0px; */
  color: var(--darkcolor);
}

.right-score.score-count div {
  transform: skew(-13deg);
}

.right-score.score-count:before {
  background: linear-gradient(180deg, var(--dark1) 20%, var(--dark2) 60%);
}

.footer-score {
  position: relative;
  width: 100%;
  color: #fff;
  display: flex;
  justify-content: space-around;
  align-items: end;
  margin-bottom: 40px;
  margin-top: 30px;
}

.timer {
  color: #ff0;
  font-size: 65pt;
  border: 5px solid #e1e1e1;
  border-radius: 15px;
  padding: 2px 20px 4px;
  width: 34%;
  text-align: center;
}

.foul-count {
  background: linear-gradient(180deg, var(--light1) 20%, var(--light2) 60%);
  text-align: center;
  font-size: 50pt;
  padding: 15px 0 6px;
  border: 5px solid #fff;
  color: var(--lightcolor);
  line-height: 1;
  border-radius: 15px;
}

.timout-count {
  display: flex;
  flex-direction: column-reverse;
}

.right-foul .foul-count {
  background: linear-gradient(180deg, var(--dark1) 20%, var(--dark2) 60%);
  color: var(--darkcolor);
}

.left-foul {
  margin-left: 60px;
}

.right-foul {
  margin-right: 60px;
}

.foul-team h3 {
  margin: 0 0 8px;
  text-align: center;
  letter-spacing: 1px;
}

.right-foul .list-timout {
  margin-right: 40px;
  margin-left: 0px;
}

.right-foul .list-timout.active {
  background: var(--dark1);
}

.list-timout.active {
  background: var(--light1);
}

.list-timout {
  width: 20px;
  height: 20px;
  border: 3px solid #999;
  border-radius: 100%;
  margin-top: 10px;
  margin-left: 40px;
}

.foul-team {
  display: flex;
  align-items: end;
}

img.light-score {
  position: absolute;
  z-index: 999;
  top: 68px;
  transform: skew(-8deg) rotate(90deg) translateX(-50%);
  width: 70px;
  left: 41%;
}

img.light-big {
  position: absolute;
  max-width: 100%;
  margin: 0 auto;
}

img.light-left {
  position: absolute;
  left: -45px;
  z-index: -1;
  height: 100%;
}

img.light-right {
  position: absolute;
  right: -45px;
  z-index: -1;
  height: 100%;
}

.timer.quarter {
  position: absolute;
  left: 50%;
  transform: translate(-50%);
  bottom: 100%;
  font-size: 35pt;
  width: 91px;
  text-align: center;
  margin-bottom: 20px;
  padding: 19px 0 15px;
  height: 81px;
  display: flex;
  align-items: center;
  color: #fff;
  justify-content: center;
  /* background: linear-gradient(180deg, #ff0 20%, #aa0 60%); */
}

.label-quarter {
  color: #fff;
  font-size: 12pt;
  position: absolute;
  bottom: 100%;
  margin-bottom: 10px;
}

video {
  position: absolute;
  width: 100%;
  height: 177vh;
  top: 50%;
  left: 0;
  transform: translateY(-50%);
}

.bg-iframe {
  position: absolute;
  height: 100vh;
  top: 0;
  left: 0;
  width: 100%;
}

.video-section div {
  width: 100%;
  height: 100%;
  position: absolute;
}

.video-section {
  position: absolute;
  width: 100%;
  height: 100%;
}
</style>
