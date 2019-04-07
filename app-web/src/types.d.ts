import Vue from 'vue';
import { NavigationGuard } from 'vue-router';

/**
 * Vue
 */

type Next = Parameters<NavigationGuard>[2];

/**
 * Organizations
 *
 *   The "organization" basically represents a self-contained data store.
 *   While they nest data within themselves, it is an attempt to keep any
 *   cross-contamination to a minimum, and as a singular entity an
 *   organization should itself remain flat and normalized.
 */

interface OrganizationUser {
  name: string;
}

interface OrganizationUserMap {
  [name: string]: OrganizationUser;
}

interface Organization {
  shortId: string;
  title: string;
  users: OrganizationUserMap;
}
