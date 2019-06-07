<template>
  <div id="TeamsView">
    <Nav/>
    <div class="teams-content">
      <!-- Assign key so team change can re-create component -->
      <router-view :key="$route.params.team"/>
    </div>
  </div>
</template>

<script lang='ts'>
import Vue from 'vue';
import { Route } from 'vue-router';
import Component from 'vue-class-component';
import Repo from '@/lib/repo';
import Nav from '@/components/teams/Nav.vue';
import { Next } from '@/types';

@Component({
  components: {
    Nav,
  },
})
export default class extends Vue {
  beforeRouteEnter(to: Route, from: Route, next: Next) {
    Repo.loadTeams()
      .then(() => next())
      .catch(() => next(false));
  }
}
</script>

<style lang='scss'>
#TeamsView {
  display: flex;
  height: 100%;
  flex-direction: column;

  .teams-content {
    flex: 1;
    overflow: hidden;
  }
}
</style>
