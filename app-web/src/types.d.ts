import Vue from 'vue';
import { RawLocation } from 'vue-router';

export type Next<V extends Vue = Vue> =
  (to?: RawLocation | false | ((vm: V) => any) | void) => void;
