<template><router-view/></template>

<script lang='ts'>
import Vue from 'vue';
import Component from 'vue-class-component';
import Store from '@/lib/store';
import { Route } from 'vue-router';
import { Next, Org } from '@/types';
import { slugerize } from '@/lib/helpers';

const findOrg = (shortId: string) => Store
  .getters['organizations/byShortId'](shortId);

const verifyOrg = (shortId: string, next: Next) => findOrg(shortId)
  ? next()
  : next({ name: 'organizations' });

@Component
export default class OrganizationRoot extends Vue {
  beforeRouteEnter(to: Route, from: Route, next: Next) {
    console.info('OrganizationRoot:beforeEnter');
    verifyOrg(to.params.org, next);
  }

  beforeRouteUpdate(to: Route, from: Route, next: Next) {
    console.info('OrganizationRoot:beforeUpdate');

    if (to.params.org === from.params.org) {
      // Don't allow navigation back up to root, otherwise everything
      // in `created` will need to happen again
      to.name === 'organizationRoot' ? next(false) : next();
      return;
    }

    verifyOrg(to.params.org, next);
  }

  /**
   * Re-slugerizes the org. title so that it's always up-to-date, which
   * allows old bookmarks to work if org. name changes. If the route is
   * OrgRoot, it will transition to the inner org. Otherwise, only the
   * params are updated and not the route name;
   */
  created() {
    console.info('OrganizationRoot:created');

    const shortId = this.$route.params.org;
    const org = findOrg(shortId);
    const newTitle = slugerize(org.title);

    if (newTitle === this.$route.params.orgTitle) { return; }

    const params = { ...this.$route.params, orgTitle: newTitle };
    const toName = this.$route.name === 'organizationRoot'
      ? 'organization'
      : this.$route.name;

    console.info('\tUpdating Org name');
    this.$router.push({ name: toName, params });
  }
}
</script>
