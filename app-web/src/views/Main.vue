<template>
  <div class="Main">
    <div class="org-sidebar">
      <ul>
        <li v-for="org in orgs">{{ org.short_id }}</li>
      </ul>

      <router-link to="/main/new-organization">+ New</router-link>
    </div>
    <div class="org-content">
      <router-view></router-view>
    </div>
  </div>
</template>

<script lang='ts'>
import Vue from 'vue';
import { RawLocation, Route } from 'vue-router';
import { Next } from '@/types';
import Component from 'vue-class-component';
import Auth from '@/lib/auth';

@Component
export default class Main extends Vue {
  beforeRouteEnter(to: Route, from: Route, next: Next) {
    if (!Auth.isAuthenticated()) {
      next({ name: 'home' });
      return;
    }

    next();
  }

  get orgs() {
    const data = localStorage.getItem('authenticatedUerData') || '{}';

    return JSON.parse(data).organizations;
  }
}
</script>

<style scoped lang='scss'>
.Main {
  display: flex;

  .org-sidebar {
    border-right: 1px solid #999;
    height: 80vh;
    width: 150px;
  }

  .org-content {
    flex: 1;
  }
}
</style>
