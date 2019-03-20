<template></template>

<script lang='ts'>
import Vue from 'vue';
import Component from 'vue-class-component';
import { Route } from 'vue-router';
import { Next } from '@/types';

@Component
export default class extends Vue {
  /**
   * If organizations are present, redirects to the first in the list.
   * Otherwise, just redirects to new form if none are present.
   */
  beforeRouteEnter(to: Route, from: Route, next: Next) {
    const orgs = JSON.parse(localStorage.getItem('authUser') || '{}').organizations;

    if (orgs.length > 0) {
      const shortId = orgs[0].short_id;

      next({ name: 'organizationRoot', params: { orgId: shortId } });
      return;
    }

    next({ name: 'newOrganization' });
  }
}
</script>
