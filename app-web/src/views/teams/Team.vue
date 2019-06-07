<template>
  <div id="TeamView">
    <Streams/>
    <content>
      <router-view/>
    </content>
  </div>
</template>

<script lang='ts'>
import Vue from 'vue';
import Component from 'vue-class-component';
import Streams from '@/components/teams/Streams.vue';
import { Route } from 'vue-router';
import { Next } from '@/types';
import Repo from '@/lib/repo';


const toStreamIfRoot = (to: Route, next: Next) => {
  if (to.name === 'team') {
    const stream = Repo.getTeam(to.params.team)!.getDefaultStream()!.stream;
    const params = { ...to.params, stream: stream.key };

    console.info('\tRedirecting to stream root');
    return next({ name: 'streamRoot', params });
  }

  next();
}

@Component({
  components: {
    Streams,
  },
})
export default class TeamView extends Vue {
  beforeRouteEnter(to: Route, from: Route, next: Next) {
    toStreamIfRoot(to, next);
  }

  beforeRouteUpdate(to: Route, from: Route, next: Next) {
    toStreamIfRoot(to, next);
  }
}
</script>

<style lang='scss'>
#TeamView {
  display: flex;
  height: 100%;

  .Streams {
    height: 100%;
    padding: 10px;
    width: 200px;
  }

  & > content {
    flex: 1;
    padding: 10px;
  }
}
</style>
