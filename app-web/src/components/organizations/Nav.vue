<template>
  <nav class="OrganizationsNav">
    <ul class="organizations">
      <li><router-link :to="{ name: 'newOrganization' }">+ New</router-link></li>

      <li v-for="org in orgs">
        <router-link :to="{ name: 'organizationRoot', params: { org: org.shortId } }">
          {{ org.title }}
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

@Component
export default class Nav extends Vue {
  get orgs() {
    return this.$store.getters['organizations/all'];
  }
}
</script>

<style scoped lang='scss'>
@import '@/styles/colors.scss';

.OrganizationsNav {
  background: $midBlue;
  color: white;
  display: flex;
  font-size: 14px;
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
