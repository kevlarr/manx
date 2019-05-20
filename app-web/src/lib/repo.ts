import { action, observable } from "mobx";
import Api, { ParsedResponse } from '@/lib/api';
import {
  Organization,
  NewOrganization,
  OrganizationUser,
  Stream,
  Topic,
  NewTopic,
  Comment,
} from '@/types';


export class ApplicationRepo {
  @observable organizations: OrganizationRepo[] = [];

  @action.bound addOrganization(org: Organization): OrganizationRepo {
    let repo = this.organizations.find(repo => repo.organization.id === org.id);

    if (!repo) {
      repo = new OrganizationRepo(org);
      this.organizations.push(repo);
    }

    return repo;
  }

  @action.bound async loadOrganizations() {
    const { body }: any = await Api.get('organizations');

    const repos = body.organizations.reduce((acc: object, org: Organization) => ({
      ...acc,
      [org.id]: this.addOrganization(org),
    }), {});

    body.streams.forEach((stream: Stream) => (
      repos[stream.organizationId].addStream(stream)
    ));

    body.organizationUsers.forEach((orgUser: OrganizationUser) => (
      repos[orgUser.organizationId].addUser(orgUser)
    ));
  }

  @action.bound async createOrganization({ title, userName }: NewOrganization): Promise<OrganizationRepo> {
    const data = { organization: { title }, organizationUser: { name: userName } };
    const { body }: any = await Api.post('organizations', data);

    const repo = this.addOrganization(body.organization);

    repo.addUser(body.organizationUser);
    repo.addStream(body.stream);

    return repo;
  }

  @action.bound reset() {
    this.organizations = [];
  }

  getOrganization(shortId: string): OrganizationRepo | null {
    return this.organizations.find(o => o.organization.shortId === shortId) || null;
  }
}


export class OrganizationRepo {
  constructor(org: Organization) {
    this.organization = Object.freeze(org);
  }

  @observable organization: Organization;
  @observable streams: StreamRepo[] = [];
  @observable users: OrganizationUser[] = [];

  @action.bound addStream(stream: Stream) {
    let repo = this.streams.find(repo => repo.stream.id === stream.id);

    if (!repo) {
      repo = new StreamRepo(stream);
      this.streams.push(repo);
    }

    return repo;
  }

  @action.bound addUser(user: OrganizationUser) {
    let u = this.users.find(u => u.id === user.id);

    if (!u) {
      u = user;
      this.users.push(u);
    }

    return u;
  }

  getStream(shortId: string): StreamRepo | null {
    return this.streams.find(s => s.stream.shortId === shortId) || null;
  }

  getGlobalStream(): StreamRepo | null {
    return this.streams.find(s => s.stream.global) || null;
  }
}


export class StreamRepo {
  constructor(stream: Stream) {
    this.stream = Object.freeze(stream);
  }

  @observable stream: Stream;
  @observable topics: TopicRepo[] = [];

  @action.bound addTopic(topic: Topic) {
    if (this.topics.find(repo => repo.topic.id === topic.id)) { return; }

    //this.topics = [...this.topics, new TopicRepo(topic)];
    this.topics.push(new TopicRepo(topic));
  }

  @action.bound async createTopic(newTopic: NewTopic) {
    //const url = `streams/${newTopic.streamId}/topics`;
    //const { body }: any = await Api.post(url, { topic: newTopic });

    this.addTopic({...newTopic, id: Number(new Date()), rendered: '' });

    // FIXME
    //return body;
  }
}


export class TopicRepo {
  constructor(topic: Topic) {
    this.topic = Object.freeze(topic);
  }

  @observable topic: Topic;
  @observable comments: Comment[] = [];

  //@action async addComment(comment: Comment) {
    // FIXME: API call
    // this._comments[comment.id] = Object.freeze(comment);
  //}
}


export default new ApplicationRepo();
