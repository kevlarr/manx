import Api, { ParsedResponse } from '@/lib/api';
import { NewOrg } from '@/types';

const normalizeOrg = (org: any) => ({
  ...org,
  shortId: org.short_id,
});

const normalizeOrgUser = (orgUser: any) => ({
  id: orgUser.id,
  orgId: orgUser.organization_id,
  name: orgUser.name,
});

const normalizeStream = (stream: any) => ({
  id: stream.id,
  parentId: stream.parent_id,
  orgId: stream.organization_id,
  global: stream.global,
  name: stream.name,
  shortId: stream.short_id,
});

export default {
  /**
   * GET all organizations and relevant models for the authenticated user,
   * committing all to success on success
   */
  fetchOrgs({ commit }: any) {
    return Api.get('organizations')
      .then(({ body }: any) => {
        const orgs = body.organizations.map(normalizeOrg);
        const orgUsers = body.organization_users.map(normalizeOrgUser);
        const streams = body.streams.map(normalizeStream);

        commit('addOrgs', orgs);
        commit('addOwnOrgUsers', orgUsers);
        commit('addStreams', streams);

        return orgs;
      });
  },

  /**
   * POST new organization details, committing organization and other
   * models to state on success
   */
  createOrg({ commit }: any, newOrg: NewOrg) {
    const payload = {
      organization: { title: newOrg.title },
      organization_user: { name: newOrg.userName },
    };

    return Api.post('organizations', payload)
      .then(({ body }: any) => {
        const org = normalizeOrg(body.organization);
        const orgUser = normalizeOrgUser(body.organization_user);
        const stream = normalizeStream(body.stream);

        commit('addOrg', org);
        commit('addOwnOrgUser', orgUser);
        commit('addStream', stream);

        return org;
      });
  },
};
