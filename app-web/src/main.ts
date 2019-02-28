import './initializers';
import Vue from 'vue';
import upperFirst from 'lodash/upperFirst'
import camelCase from 'lodash/camelCase'
import App from './App.vue';
import router from './router';
import store from './store';

Vue.config.productionTip = false;

/**
 * Globally register `_global` components
 */
const globals = require.context(
  './components/_global',
  false,
  /[A-Z]\w+\.vue$/
);

globals.keys().forEach((fileName) => {
  const config = globals(fileName);

  // Get PascalCase name of component
  const componentName = upperFirst(
    camelCase(
      // Strip the leading `./` and extension from the filename
      fileName.replace(/^\.\/(.*)\.\w+$/, '$1')
    )
  );

  // Register component globally
  Vue.component(
    componentName,
    // Look for the component options on `.default`, which will
    // exist if the component was exported with `export default`,
    // otherwise fall back to module's root.
    config.default || config
  );
});

/**
 * Create app
 */
new Vue({
  router,
  store,
  render: h => h(App),
}).$mount('#app');
