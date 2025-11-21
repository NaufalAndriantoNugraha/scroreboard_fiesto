<template>
  <div class="template">
    <h1>Scoreboard Alternative</h1>
    <div class="scoreboard-container">
      <div class="scoreboard-team">
        <div class="team-name">
          <p>Team Name</p>
          <p>{{ teamA.name }}</p>
        </div>
        <p>Score: {{ teamA.score }}</p>
        <p>Foul: {{ teamA.foul }}</p>
        <div v-if="!isTimeout">
          <p>Quarter: {{ quarter }}</p>
        </div>
        <div v-else>
          <p>Timeout: {{ formatedTimeout }}</p>
        </div>
        <p>Timer: {{ formatedTime }}</p>
      </div>
      <div class="scoreboard-team">
        <div class="team-name">
          <p>Team Name</p>
          <p>{{ teamB.name }}</p>
        </div>
        <p>Score: {{ teamB.score }}</p>
        <p>Foul: {{ teamB.foul }}</p>
        <div v-if="!isTimeout">
          <p>Quarter: {{ quarter }}</p>
        </div>
        <div v-else>
          <p>Timeout: {{ formatedTimeout }}</p>
        </div>
        <p>Timer: {{ formatedTime }}</p>
      </div>
    </div>
  </div>
</template>

<style>
h1 {
  font-weight: bold;
  font-size: 2rem;
}

.template {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  gap: 10px;
  background-color: black;
  color: white;
  width: 100%;
  height: 100vh;
}

.scoreboard-container {
  display: flex;
  gap: 20px;
}

.scoreboard-team {
  border: 1px solid black;
  padding: 1rem;
}

.team-name p:first-child {
  font-weight: bold;
  font-size: 1.25rem;
}
</style>

<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { emit, listen } from '@tauri-apps/api/event';
import type { TeamInfo } from '~/types/TeamInfo';
import { doc, updateDoc } from 'firebase/firestore';
import { event } from '@tauri-apps/api';

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
    await listen('start_timer_event', (event: any) => {
      this.startTimer(event.payload.initialTime);
    });
    await listen('reset_timer_event', (event: any) => {
      this.resetTimer();
    });
    await listen('stop_timer_event', (event: any) => {
      this.stopTimer();
    });
    await listen('start_timeout_event', (event: any) => {
      this.startTimeout(event.payload.team, event.payload.initialTime);
    });
    await listen('stop_timeout_event', (event: any) => {
      this.stopTimeout();
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
    });

    await listen('quarter_step_event', (event: any) => {
      switch (event.payload.step) {
        case 'up':
          this.quarter += 1;
          break;

        case 'down':
          this.quarter = this.quarter - 1 < 0 ? 0 : this.quarter - 1;
          break;

        case 'reset':
          this.quarter = 1;
          break;

        default:
          break;
      }
    });

    await listen('change_time_event', (event: any) => {
      this.time = this.time + event.payload.value * 1000;
    });

    await listen('team_name_event', (event: any) => {
      switch (event.payload.team) {
        case 'teamA':
          this.teamA.name = event.payload.name;
          break;

        case 'teamB':
          this.teamB.name = event.payload.name;
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
              break;

            case 'teamB':
              this.teamB.score += 1;
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
              break;

            case 'teamB':
              this.teamB.score =
                this.teamB.score - 1 < 0 ? 0 : this.teamB.score - 1;
              break;

            default:
              break;
          }
          break;

        case 'reset':
          this.teamA.score = 0;
          this.teamB.score = 0;
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
              break;

            case 'teamB':
              this.teamB.foul += 1;
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
              break;

            case 'teamB':
              this.teamB.foul =
                this.teamB.foul - 1 < 0 ? 0 : this.teamB.foul - 1;
              break;

            default:
              break;
          }
          break;

        case 'reset':
          this.teamA.foul = 0;
          this.teamB.foul = 0;
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
              break;

            case 'teamB':
              this.teamB.timeout =
                this.teamB.timeout + 1 > 3 ? 3 : this.teamB.timeout + 1;
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
              break;

            case 'teamB':
              this.teamB.timeout =
                this.teamB.timeout - 1 < 0 ? 0 : this.teamB.timeout - 1;
              break;

            default:
              break;
          }
          break;

        case 'reset':
          this.teamA.timeout = 0;
          this.teamB.timeout = 0;
          break;

        default:
          break;
      }
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
            invoke('update_time', { time: this.formatedTime });
            emit('timer_event', { value: this.time });
          }
          this.time -= 10; // Increment every 10 milliseconds
          if (this.time <= 0) {
            this.time = 0;
            this.stopTimer();
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
          }
          this.timeout -= 10;
          if (this.timeout <= 0) {
            this.stopTimeout();
            emit('timeout_event', { value: this.timeout });
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
  },
};
</script>
