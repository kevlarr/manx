<template></template>

<script lang="ts">
import Vue from 'vue';
import { Route, RawLocation } from 'vue-router';
import { Next } from '@/types';
import Component from 'vue-class-component';
import Api from '@/lib/api';
import Auth from '@/lib/auth';

@Component
export default class extends Vue {
  beforeRouteEnter(to: Route, from: Route, next: Next) {
    Api.delete('sessions/1')
      .then(() => next({ name: 'home' }))
      .catch(() => next(false));
  }
}
</script>
