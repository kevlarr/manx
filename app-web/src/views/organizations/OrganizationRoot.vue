<template><router-view/></template>

<script lang='ts'>
import Vue from 'vue';
import Component from 'vue-class-component';
import { Route } from 'vue-router';
import { Next } from '@/types';

interface Org {
  short_id: string;
  title: string;
}

function findOrg(shortId: string): Org | null {
  const orgs = JSON.parse(localStorage.getItem('authUser') || '{}').organizations;

  return orgs.find((o: Org) => o.short_id === shortId);
}

function verifyOrg(shortId: string, next: Next) {
    findOrg(shortId) ? next() : next({ name: 'organizations' });
}

@Component
export default class extends Vue {
  beforeRouteEnter(to: Route, from: Route, next: Next) {
    verifyOrg(to.params.orgId, next);
  }

  beforeRouteUpdate(to: Route, from: Route, next: Next) {
    // Disallow navigating back up to 'root', otherwise would need to
    // redo the hook that navigations BACK to 'organization' with title,
    // but any other route changes within the org should be allowed.
    if (to.params.orgId === from.params.orgId) {
      to.name === 'organizationRoot' ? next(false) : next();
      return;
    }

    // If org. itself does change, verify it exists
    verifyOrg(to.params.orgId, next);
  }

  created() {
    // If the `organization` route is loading it will have a title already.
    // Strip it in favor of regenerating the title slug anew, as this means
    // that the org title can be updated (and show in URL) without breaking links.
    const org = findOrg(this.$route.params.orgId);
    const title = this.titleSlug(org!);

    this.$router.push({ name: 'organization', params: {
      orgId: org!.short_id,
      orgTitle: title,
    }});
  }

  titleSlug(org: Org): string {
    return org.title.replace(/\s+/g, '-');
  }
}
</script>
