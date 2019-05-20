<template>
  <div id="OrganizationView">
    <Streams/>
    <content>
      <router-view/>
    </content>
  </div>
</template>

<script lang='ts'>
import Vue from 'vue';
import Component from 'vue-class-component';
import Streams from '@/components/organizations/Streams.vue';
import { Route } from 'vue-router';
import { Next } from '@/types';
import Repo from '@/lib/repo';


const toStreamIfRoot = (to: Route, next: Next) => {
  if (to.name === 'organization') {
    const global = Repo
      .getOrganization(to.params.org)!
      .getGlobalStream()!
      .stream;

    const params = { ...to.params, stream: global.shortId };

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
export default class Organization extends Vue {
  /**
   * Finds and redirects to global stream if there is no child route
   */
  beforeRouteEnter(to: Route, from: Route, next: Next) {
    console.info('Organization:beforeEnter');
    toStreamIfRoot(to, next);
  }

  beforeRouteUpdate(to: Route, from: Route, next: Next) {
    console.info('Organization:beforeUpdate');
    toStreamIfRoot(to, next);
  }
}
</script>

<style lang='scss'>
#OrganizationView {
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
