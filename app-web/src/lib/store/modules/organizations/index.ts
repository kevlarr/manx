import Vue from 'vue';
import { OrgsState as State } from '@/types';
import actions from './actions';
import getters from './getters';
import mutations from './mutations';

export default {
  actions,
  getters,
  mutations,

  namespaced: true,
  state: {} as State,
};
