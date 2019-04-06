import Vuex, { ActionContext } from 'vuex';
import createLogger from 'vuex/dist/logger';
import { Organization } from '@/types';
import Api, { ParsedResponse } from '@/lib/api';

const isProduction = process.env.NODE_ENV === 'production';

interface State {
  organizations: { [shortId: string]: Organization };
}

type Context = ActionContext<State, {}>;

export default new Vuex.Store({
  plugins: isProduction ? []: [createLogger()],
  strict: !isProduction,

  state: {
    organizations: {},
  } as any,

  getters: {
    organizations(s: State) {
      return Object.values(s.organizations)
        .sort((a: Organization, b: Organization) => (
          a.title > b.title ? 1 : 0));
    },

    organization(s: State) {
      return (shortId: string) => s.organizations[shortId];
    }
  } as any,

  mutations: {
    addOrganizations(s: State, orgs: Organization[]) {
      orgs.forEach(org => s.organizations[org.shortId] = org);
    }
  } as any,

  actions: {
    getOrganizations({ commit }: Context) {
      return Api.get('organizations')
        .then(({ body }: any) => {
          if (!body.organizations) { return; }

          commit('addOrganizations', body.organizations.map((org: any) => ({
            shortId: org.short_id,
            title: org.title,
          })));
        });
    }
  } as any,
});
