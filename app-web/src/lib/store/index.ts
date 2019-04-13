import Vue from 'vue';
import Vuex from 'vuex';
import createLogger from 'vuex/dist/logger';
import OrgsMod from '@/lib/store/modules/organizations';

const isProduction = process.env.NODE_ENV === 'production';

export default new Vuex.Store({
  plugins: isProduction ? []: [createLogger()],
  strict: !isProduction,

  modules: {
    organizations: OrgsMod,
  },

  actions: {
    reset({ commit }: any) {
      commit('organizations/reset');
      return true;
    },
  },
});
