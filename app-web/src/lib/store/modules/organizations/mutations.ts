import Vue from 'vue';
import {
  Org,
  OrgUser,
  Stream,
  OrgsState as State
} from '@/types';

const setOrg = (s: State, org: Org) => {
  Vue.set(s, org.id, {
    org,
    ownUser: null,
    allUsers: {},
    streams: {},
  });
};

const setOrgUser = (s: State, orgUser: OrgUser) => {
  Vue.set(s[orgUser.orgId].allUsers, orgUser.id, orgUser);
};

const setOwnOrgUser = (s: State, orgUser: OrgUser) => {
  Vue.set(s[orgUser.orgId], 'ownUser', orgUser);
  setOrgUser(s, orgUser);
};

const setStream = (s: State, stream: Stream) => {
  Vue.set(s[stream.orgId].streams, stream.id, stream);
};

export default {
  /**
   * addOrg(s)
   *    Adds the organization(s) to the module state
   */
  addOrg(s: State, org: Org) {
    setOrg(s, org);
  },

  addOrgs(s: State, orgs: Org[]) {
    orgs.forEach(org => setOrg(s, org));
  },

  /**
   * addOwnOrgUser*
   *    Adds the authenticated user's orgUser(s) to the organization(s)
   */
  addOwnOrgUser(s: State, orgUser: OrgUser) {
    setOwnOrgUser(s, orgUser);
  },

  addOwnOrgUsers(s: State, orgUsers: OrgUser[]) {
    orgUsers.forEach(orgUser => setOwnOrgUser(s, orgUser));
  },

  /**
   * addStream(s)
   *    Adds the stream(s) to the organization(s)
   */
  addStream(s: State, stream: Stream) {
    setStream(s, stream);
  },

  addStreams(s: State, streams: Stream[]) {
    streams.forEach(stream => setStream(s, stream));
  },

  /**
   * Clears all organizations from the state
   */
  reset(s: State) {
    Object.keys(s).forEach(key => delete s[key]);
  },
};
