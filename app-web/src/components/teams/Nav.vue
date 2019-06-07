<template>
  <nav class="TeamsNav">
    <ul class="teams">
      <li><router-link :to="{ name: 'newTeam' }">+ New</router-link></li>

      <li v-for="repo in teams">
        <router-link :to="{ name: 'teamRoot', params: { team: repo.team.key } }">
          {{ repo.team.title }}
        </router-link>
      </li>
    </ul>

    <ul class="user">
      <li><router-link :to="{ name: 'signout' }">Sign Out</router-link></li>
    </ul>
  </nav>
</template>

<script lang='ts'>
import Vue from 'vue';
import Component from 'vue-class-component';
import { Observer } from 'mobx-vue';
import { Team } from '@/types';
import Repo, { TeamRepo } from '@/lib/repo';

@Observer
@Component
export default class Nav extends Vue {
  get teams(): TeamRepo[] {
    return Repo.teams;
  }
}
</script>

<style scoped lang='scss'>
@import '@/styles/colors.scss';

.TeamsNav {
  background: $midBlue;
  color: white;
  display: flex;
  font-size: 14px;
  height: 27px;
  justify-content: space-between;

  a {
    color: white;
    display: inline-block;
    padding: 5px 15px;
    text-decoration: none;

    &:visited {
      color: white;
    }

    &.router-link-active {
      background: $lightBlue;
    }
  }

  ul {
    list-style-type: none;
    margin: 0;
    padding: 0;

    li {
      display: inline-block;
    }
  }
}
</style>
