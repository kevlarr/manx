import Router, { Route } from 'vue-router';
import { Next } from '@/types';
import Auth from '@/lib/auth';
import Home from '@/views/Home.vue';
import Organizations from '@/views/Organizations.vue';
import OrganizationsIndex from '@/views/organizations/OrganizationsIndex.vue';
import NewOrganization from '@/views/organizations/NewOrganization.vue';
import OrganizationRoot from '@/views/organizations/OrganizationRoot.vue';
import Organization from '@/views/organizations/Organization.vue';
import Signout from '@/views/Signout.vue';
import StyleGuide from '@/views/StyleGuide.vue';
import StyleGuideText from '@/views/style-guide/Text.vue';
import StyleGuideComponents from '@/views/style-guide/Components.vue';

const router = new Router({
  mode: 'history',
  base: process.env.BASE_URL,
  routes: [
    {
      path: '/style-guide',
      component: StyleGuide,
      children: [
        {
          path: 'text',
          name: 'styleGuideText',
          component: StyleGuideText,
        },
        {
          path: 'components',
          name: 'styleGuideComponents',
          component: StyleGuideComponents,
        },
      ],
    },
    {
      path: '/',
      name: 'home',
      component: Home,
    },
    {
      path: '/sign-out',
      name: 'signout',
      component: Signout,
    },
    {
      path: '/orgs',
      component: Organizations,
      children: [
        {
          path: 'new',
          name: 'newOrganization',
          component: NewOrganization,
        },
        {
          path: ':orgId',
          name: 'organizationRoot',
          component: OrganizationRoot,
          children: [
            {
              path: ':orgTitle',
              name: 'organization',
              component: Organization,
            },
          ],
        },
        {
          path: '',
          name: 'organizations',
          component: OrganizationsIndex,
        },
      ],
    },
  ],
});

router.beforeEach((to: Route, from: Route, next: Next) => {
  const isAuthed = Auth.isAuthenticated();

  // Require an authenticated user for anything under /orgs
  if (to.path.startsWith('/orgs') && !isAuthed) {
    return next({ name: 'home' });
  }

  // Redirect from home sign in/up if already logged in
  if (to.name === 'home' && isAuthed) {
    return next({ name: 'organizations' });
  }

  next();
});

export default router;
