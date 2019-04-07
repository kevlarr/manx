import Vue from 'vue';
import Vuex, { ActionContext } from 'vuex';
import createLogger from 'vuex/dist/logger';
import { Organization, OrganizationUser, OrganizationUserMap } from '@/types';
import Api, { ParsedResponse } from '@/lib/api';

const isProduction = process.env.NODE_ENV === 'production';

/**
 * Normalizers to reshape API data into format store expects
 */
const normalizeOrganization = (org: any) => ({
  shortId: org.short_id,
  title: org.title,
  users: {},
});

/**
 * Types
 */
interface State {
  organizations: { [shortId: string]: Organization };
}

type Context = ActionContext<State, {}>;

interface NewOrganization {
  title: string;
  userName: string;
}

/**
 * Store
 */
export default new Vuex.Store({
  plugins: isProduction ? []: [createLogger()],
  strict: !isProduction,

  /**
   * State
   */
  state: {
    organizations: {},
  } as any,

  /**
   * Getters
   */
  getters: {

    /**
     * Return list of all organizations sorted by title
     */
    organizations(s: State) {
      return Object.values(s.organizations)
        .sort((a: Organization, b: Organization) => (
          a.title > b.title ? 1 : 0));
    },

    /**
     * Returns a closure that can retrieve organization from state by `shortId`
     */
    organization(s: State) {
      return (shortId: string) => s.organizations[shortId];
    }
  } as any,

  /**
   * Mutations
   */
  mutations: {

    /**
     * Adds array of organizations to the state
     */
    addOrganizations(s: State, orgs: Organization[]) {
      orgs.forEach(org => Vue.set(s.organizations, org.shortId, org));
    },

    /**
     * Adds a single organization to the state
     */
    addOrganization(s: State, org: Organization) {
      Vue.set(s.organizations, org.shortId, org);
    },

    /**
     * Clears all organizations from the state
     */
    resetState(s: State) {
      s.organizations = {};
    },
  } as any,

  /**
   * Actions
   */
  actions: {

    /**
     * GET all organizations for the authenticated user, committing to
     * store on success
     */
    getOrganizations({ commit }: Context) {
      return Api.get('organizations')
        .then(({ body: { organizations } }: any) => {
          if (!organizations) { return; }

          const orgs = organizations.map(normalizeOrganization);

          commit('addOrganizations', orgs);
          return orgs;
        });
    },

    /**
     * POST new organization details, committing to store on success
     */
    createOrganization({ commit }: Context, { title, userName }: NewOrganization) {
      const payload = {
        organization: { title },
        organization_user: { name: userName },
      };

      return Api.post('organizations', payload)
        .then(({ body }: any) => {
          const org = normalizeOrganization(body.organization);

          commit('addOrganization', org);
          return org;
        });
    },

    /**
     * Resets the store
     */
    reset({ commit }: Context) {
      commit('resetState');
      return true;
    },
  } as any,
});
