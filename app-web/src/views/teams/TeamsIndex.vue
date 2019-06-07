<template></template>

<script lang='ts'>
import Vue from 'vue';
import Component from 'vue-class-component';
import { Route } from 'vue-router';
import Repo from '@/lib/repo';
import { Next } from '@/types';

@Component
export default class TeamsIndex extends Vue {
  /**
   * If teams are present, redirects to the first in the list.
   * Otherwise, just redirects to new form if none are present.
   */
  beforeRouteEnter(to: Route, from: Route, next: Next) {
    const teams = Repo.teams;

    if (teams.length > 0) {
      const { key } = teams[0].team;

      next({ name: 'teamRoot', params: { team: key } });
      return;
    }

    next({ name: 'newTeam' });
  }
}
</script>
