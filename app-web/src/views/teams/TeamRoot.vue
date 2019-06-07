<template><router-view/></template>

<script lang='ts'>
import Vue from 'vue';
import Component from 'vue-class-component';
import { Route } from 'vue-router';
import Repo from '@/lib/repo';
import { Next } from '@/types';
import { slugerize } from '@/lib/helpers';


const verifyTeam = (key: string, next: Next) => Repo.getTeam(key)
  ? next()
  : next({ name: 'teams' });


@Component
export default class TeamRoot extends Vue {
  beforeRouteEnter(to: Route, from: Route, next: Next) {
    console.info('TeamRoot:beforeEnter');
    verifyTeam(to.params.team, next);
  }

  beforeRouteUpdate(to: Route, from: Route, next: Next) {
    console.info('TeamRoot:beforeUpdate');

    if (to.params.team === from.params.team) {
      // Don't allow navigation back up to root, otherwise everything
      // in `created` will need to happen again
      to.name === 'teamRoot' ? next(false) : next();
      return;
    }

    verifyTeam(to.params.team, next);
  }

  /**
   * Re-slugerizes the team title so that it's always up-to-date, which
   * allows old bookmarks to work if team name changes. If the route is
   * TeamRoot, it will transition to the inner team. Otherwise, only the
   * params are updated and not the route name;
   */
  created() {
    console.info('TeamRoot:created');

    const key = this.$route.params.team;
    const team = Repo.getTeam(key)!.team;
    const newTitle = slugerize(team.title);

    if (newTitle === this.$route.params.teamTitle) { return; }

    const params = { ...this.$route.params, teamTitle: newTitle };
    const toName = this.$route.name === 'teamRoot'
      ? 'team'
      : this.$route.name;

    console.info('\tUpdating Team name');
    this.$router.push({ name: toName, params });
  }
}
</script>
