import Vuex from 'vuex';
import createLogger from 'vuex/dist/logger';

const debug = process.env.NODE_ENV !== 'production';

export default new Vuex.Store({
  state: {

  },
  mutations: {

  },
  actions: {

  },
  plugins: debug ? [createLogger()] : [],
});
