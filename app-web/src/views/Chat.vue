<template>
  <h1>chats</h1>
</template>

<script lang="ts">
import Vue from 'vue';
import { RawLocation, Route } from 'vue-router';
import { Next } from '@/types';
import Component from 'vue-class-component';

@Component
export default class Chat extends Vue {
  beforeRouteEnter(to: Route, from: Route, next: Next) {
    if (localStorage.getItem('authenticated') !== 'true') {
      next({ name: 'home' });
      return;
    }

    next();
  }
}
</script>
