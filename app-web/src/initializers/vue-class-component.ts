/**
 * Configures `vue-class-component` Component decorator,
 * such as registering plugin hooks, etc.
 */
import Component from 'vue-class-component';

Component.registerHooks([
  'beforeRouteEnter',
  'beforeRouteLeave',
  'beforeRouteUpdate',
])
