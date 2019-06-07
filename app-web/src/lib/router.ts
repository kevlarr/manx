import Router, { Route } from 'vue-router';
import { Next } from '@/types';
import Auth from '@/lib/auth';
import Home from '@/views/Home.vue';
import Signout from '@/views/Signout.vue';
import TeamViews from '@/views/teams/index';
import StreamViews from '@/views/teams/streams/index';
import TopicViews from '@/views/teams/streams/topics/index';
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

    { path: '/t', component: TeamViews.Root, children: [
      { path: 'new', name: 'newTeam', component: TeamViews.New },

      { path: ':team', name: 'teamRoot', component: TeamViews.TeamRoot, children: [
        { path: ':teamTitle', name: 'team', component: TeamViews.Team, children: [
          { path: ':stream', name: 'streamRoot', component: StreamViews.Root, children: [
            { path: ':streamTitle', name: 'stream', component: StreamViews.Stream, children: [
              { path: ':topic', name: 'topic', component: TopicViews.Topic },
            ]}
          ]},
        ]},
      ]},
      { path: '', name: 'teams', component: TeamViews.Index }]},
  ],
});

router.beforeEach((to: Route, from: Route, next: Next) => {
  const isAuthed = Auth.isAuthenticated();

  // Require an authenticated user for anything under /t (teams)
  if ((to.path === '/t' || to.path.startsWith('/t/')) && !isAuthed) {
    return next({ name: 'home' });
  }

  // Redirect from home sign in/up if already logged in
  if (to.name === 'home' && isAuthed) {
    return next({ name: 'teams' });
  }

  next();
});

export default router;
