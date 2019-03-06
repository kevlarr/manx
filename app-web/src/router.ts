import Vue from 'vue';
import Router from 'vue-router';
import Home from './views/Home.vue';
import Signup from './views/Signup.vue';
import Signin from './views/Signin.vue';
import Signout from './views/Signout.vue';
import NewOrganization from './views/main/NewOrganization.vue';

Vue.use(Router);

export default new Router({
  mode: 'history',
  base: process.env.BASE_URL,
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home,
    },
    {
      path: '/sign-up',
      name: 'signup',
      component: Signup,
    },
    {
      path: '/sign-in',
      name: 'signin',
      component: Signin,
    },
    {
      path: '/sign-out',
      name: 'signout',
      component: Signout,
    },
    {
      path: '/main',
      name: 'main',
      component: () => import(/* webpackChunkName: "main" */ './views/Main.vue'),
      children: [
        {
          path: 'new-organization',
          name: 'newOrganization',
          component: NewOrganization,
        },
      ],
    },
  ],
});
