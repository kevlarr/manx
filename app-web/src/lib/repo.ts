import { action, observable } from "mobx";
import Api, { ParsedResponse } from '@/lib/api';
import {
  Team,
  NewTeam,
  Member,
  Stream,
  Topic,
  NewTopic,
  Comment,
  NewComment,
} from '@/types';


export class ApplicationRepo {
  @observable teams: TeamRepo[] = [];

  @action.bound addTeam(t: Team): TeamRepo {
    let repo = this.teams.find(repo => repo.team.id === t.id);

    if (!repo) {
      repo = new TeamRepo(t);
      this.teams.push(repo);
    }

    return repo;
  }

  @action.bound async loadTeams() {
    const { body }: any = await Api.get('teams');

    const repos = body.teams.reduce((acc: object, t: Team) => ({
      ...acc,
      [t.id]: this.addTeam(t),
    }), {});

    body.members.forEach((m: Member) => repos[m.teamId].addMember(m));

    const streamRepos = body.streams.reduce((acc: object, s: Stream) => ({
      ...acc,
      [s.id]: repos[s.teamId].addStream(s),
    }), {});

    const topicRepos = body.topics.reduce((acc: object, t: Topic) => ({
      ...acc,
      [t.id]: streamRepos[t.streamId].addTopic(t),
    }), {});

    body.comments.forEach((c: Comment) => topicRepos[c.topicId].addComment(c));
  }

  @action.bound async createTeam(nt: NewTeam): Promise<TeamRepo> {
    const { body }: any = await Api.post('teams', nt);
    const repo = this.addTeam(body.team);

    repo.addMember(body.member);
    repo.addStream(body.stream);

    return repo;
  }

  @action.bound reset() {
    this.teams = [];
  }

  getTeam(key: string): TeamRepo | null {
    return this.teams.find(repo => repo.team.key === key) || null;
  }
}


export class TeamRepo {
  constructor(t: Team) {
    this.team = Object.freeze(t);
  }

  @observable team: Team;
  @observable members: Member[] = [];
  @observable streams: StreamRepo[] = [];

  @action.bound addMember(mem: Member): Member {
    let member = this.members.find(m => m.id === mem.id);

    if (!member) {
      member = Object.freeze(mem);
      this.members.push(member);
    }

    return member;
  }

  @action.bound addStream(s: Stream): StreamRepo {
    let repo = this.streams.find(repo => repo.stream.id === s.id);

    if (!repo) {
      repo = new StreamRepo(s);
      this.streams.push(repo);
    }

    return repo;
  }

  getStream(key: string): StreamRepo | null {
    return this.streams.find(s => s.stream.key === key) || null;
  }

  getDefaultStream(): StreamRepo | null {
    return this.streams.length ? this.streams[0] : null;
  }
}


export class StreamRepo {
  constructor(stream: Stream) {
    this.stream = Object.freeze(stream);
  }

  @observable stream: Stream;
  @observable topics: TopicRepo[] = [];

  @action.bound addTopic(t: Topic): TopicRepo {
    let repo = this.topics.find(repo => repo.topic.id === t.id);

    if (!repo) {
      repo = new TopicRepo(t);
      this.topics.push(repo);
    }

    return repo;
  }

  @action.bound async createTopic(nt: NewTopic): Promise<TopicRepo> {
    const url = `streams/${this.stream.id}/topics`;
    const { body }: any = await Api.post(url, nt);

    return this.addTopic(body.topic);
  }

  getTopic(key: string): TopicRepo | null {
    return this.topics.find(t => t.topic.key === key) || null;
  }
}


export class TopicRepo {
  constructor(t: Topic) {
    this.topic = Object.freeze(t);
  }

  @observable topic: Topic;
  @observable comments: Comment[] = [];

  @action.bound addComment(cmt: Comment) {
    this.comments.push(Object.freeze(cmt));
  }

  @action.bound async createComment(nc: NewComment) {
    const url = `topics/${this.topic.id}/comments`;
    const { body }: any = await Api.post(url, nc);

    return this.addComment(body.comment);
  }
}


export default new ApplicationRepo();
