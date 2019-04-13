import Router, { Route } from 'vue-router';
import { Next } from '@/types';
import Auth from '@/lib/auth';
import Home from '@/views/Home.vue';
import Signout from '@/views/Signout.vue';
import OrgViews from '@/views/orgs/index';
import StreamViews from '@/views/orgs/streams/index';
import StyleViews from '@/views/style/index';

const router = new Router({
  mode: 'history',
  base: process.env.BASE_URL,
  routes: [
    { path: '/', name: 'home', component: Home },
    { path: '/sign-out', name: 'signout', component: Signout },

    { path: '/style-guide', component: StyleViews.Root, children: [
      { path: 'text', name: 'styleGuideText', component: StyleViews.Text },
      { path: 'components', name: 'styleGuideComponents', component: StyleViews.Components }]},

    { path: '/orgs', component: OrgViews.Orgs, children: [
      { path: 'new', name: 'newOrganization', component: OrgViews.New },

      { path: ':org', name: 'organizationRoot', component: OrgViews.OrgRoot, children: [
        { path: ':orgTitle', name: 'organization', component: OrgViews.OrgShow, children: [
          { path: ':stream', name: 'streamRoot', component: StreamViews.Root, children: [
            { path: ':streamName', name: 'stream', component: StreamViews.Stream }]}]}]},

      { path: '', name: 'organizations', component: OrgViews.OrgsIndex }]},
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
