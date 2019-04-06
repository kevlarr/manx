import Vue from 'vue';
import { NavigationGuard } from 'vue-router';

type Next = Parameters<NavigationGuard>[2];

interface Organization {
  shortId: string;
  title: string;
}
