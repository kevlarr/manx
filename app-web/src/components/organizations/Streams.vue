<template>
  <div class="Streams">
    <ul>
      <li v-for="stream in streams">
        <router-link :to="streamLink(stream)">
          {{ streamName(stream) }}
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
    const shortId = this.$route.params.org;
    const streams = Repo.getOrganization(shortId)!.streams.map(repo => repo.stream);

    return streams;
  }

  streamLink(stream: Stream): object {
    const { org, orgTitle } = this.$route.params;

    return {
      name: 'streamRoot',
      params: { org, orgTitle, stream: stream.shortId },
    };
  }

  streamName(stream: Stream): string {
    return stream.global ? 'Global' : stream.name;
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
