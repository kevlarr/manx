<template></template>

<script lang="ts">
import Vue from 'vue';
import { Route, RawLocation } from 'vue-router';
import Component from 'vue-class-component';
import { Next } from '@/types';
import Api from '@/lib/api';
import Repo from '@/lib/repo';

@Component
export default class extends Vue {
  beforeRouteEnter(to: Route, from: Route, next: Next) {
    Api.delete('session')
      .then(() => Repo.reset())
      .then(() => next({ name: 'home' }))
      .catch(err => {
        alert(JSON.stringify(err));
        next(false);
      });
  }
}
</script>
