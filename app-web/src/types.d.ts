import Vue from 'vue';
import { NavigationGuard } from 'vue-router';

/**
 * Vue
 */

type Next = Parameters<NavigationGuard>[2];

/**
 * Data models
 */

interface Org {
  id: number;
  shortId: string;
  title: string;
}

interface NewOrg {
  title: string;
  userName: string;
}

interface OrgUser {
  id: number;
  orgId: string;
  name: string;
}

interface Stream {
  id: number;
  parentId: number | null;
  orgId: string;
  global: boolean;
  name: string;
  shortId: string;
}

/**
 * State management
 *
 *   The "organization" basically represents a self-contained data store.
 *   While each nests data within itself, it is an attempt to keep any
 *   cross-contamination to a minimum, and as a singular entity an
 *   organization should itself remain flat and normalized.
 */

interface OrgUserMap {
  [id: string]: OrgUser;
}

interface StreamMap {
  [id: string]: Stream;
}

interface OrgState {
  org: Org;
  ownUser: OrgUser;
  allUsers: OrgUserMap;
  streams: StreamMap;
}

// FIXME: Want to store "current OrgState" on here for faster lookup
// of users, streams, etc.
interface OrgsState {
  [id: string]: OrgState;
}
