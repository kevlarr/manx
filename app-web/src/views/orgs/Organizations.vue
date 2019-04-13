<template>
  <div id="OrganizationsView">
    <Nav/>
    <div class="organizations-content">
      <!-- Assign key so org change can re-create component -->
      <router-view :key="$route.params.org"/>
    </div>
  </div>
</template>

<script lang='ts'>
import Vue from 'vue';
import { Route } from 'vue-router';
import Component from 'vue-class-component';
import Nav from '@/components/organizations/Nav.vue';
import Store from '@/lib/store';
import { Next } from '@/types';

@Component({
  components: {
    Nav,
  },
})
export default class extends Vue {
  beforeRouteEnter(to: Route, from: Route, next: Next) {
    Store.dispatch('organizations/fetchOrgs')
      .then(() => next())
      .catch(err => {
        alert(err);
        next(false);
      });
  }
}
</script>

<style lang='scss'>
#OrganizationsView {
  display: flex;
  height: 100%;
  flex-direction: column;

  .organizations-content {
    height: 100%;
  }
}
</style>
