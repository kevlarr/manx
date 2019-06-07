<template>
  <div class="Streams">
    <ul>
      <li v-for="stream in streams">
        <router-link :to="streamLink(stream)">
          {{ stream.title }}
        </router-link>
      </li>
    </ul>
  </div>
</template>

<script lang='ts'>
import Vue from 'vue';
import Component from 'vue-class-component';
import { Stream } from '@/types';
import Repo from '@/lib/repo';


@Component
export default class Streams extends Vue {
  get streams() {
    const key = this.$route.params.team;
    const streams = Repo.getTeam(key)!.streams.map(repo => repo.stream);

    return streams;
  }

  streamLink(stream: Stream): object {
    const { team, teamTitle } = this.$route.params;

    return {
      name: 'streamRoot',
      params: { team, teamTitle, stream: stream.key },
    };
  }
}
</script>

<style lang='scss'>
.Streams {
  border-right: 1px solid #bbb;
  height: 100%;
  padding: 10px;

  & > ul {
    list-style-type: none;
    margin: 0;
    padding: 0;
  }
}
</style>
