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
import { mapGetters } from 'vuex';
import Component from 'vue-class-component';
import { Stream } from '@/types';

@Component({
  computed: {
    ...mapGetters('organizations', ['streamsFor'])
  }
})
export default class Streams extends Vue {
  streamsFor!: (shortId: string) => Stream[];

  get streams() {
    const shortId = this.$route.params.org;

    return this.streamsFor(shortId);
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
