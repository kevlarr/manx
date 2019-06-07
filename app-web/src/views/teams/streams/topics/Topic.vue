<template>
  <div id="TopicView">
    <router-link :to="{ name: 'stream', params: $route.params }">close</router-link>
    <CommentList :comments="repo.comments"/>
    <MessageInput @submit="submitComment"/>
  </div>
</template>

<script lang='ts'>
import Vue from 'vue';
import Component from 'vue-class-component';
import { Route } from 'vue-router';
import { Next, NewComment } from '@/types';
import CommentList from '@/components/comments/CommentList.vue';
import Repo, { TopicRepo } from '@/lib/repo';


function findTopic(team: string, stream: string, topicKey: string): TopicRepo | null {
  return Repo.getTeam(team)!.getStream(stream)!.getTopic(topicKey);
}

function verifyTopic(to: Route, next: Next) {
  const { team, stream, topic } = to.params;
  const repo = findTopic(team, stream, topic);

  if (!repo) {
    const params = { ...to.params };
    delete params.topic;

    return next({ name: 'stream', params });
  }

  next();
}

@Component({
  components: {
    CommentList,
  },
})
export default class TopicView extends Vue {
  beforeRouteEnter(to: Route, from: Route, next: Next) {
    verifyTopic(to, next);
  }

  beforeRouteUpdate(to: Route, from: Route, next: Next) {
    if (to.params.topic === from.params.topic) {
      return next(false);
    }

    verifyTopic(to, next);
  }

  get repo(): TopicRepo {
    const { team, stream, topic } = this.$route.params;
    return findTopic(team, stream, topic)!;
  }

  submitComment(text: string) {
    const repo = this.repo;
    const comment: NewComment = { raw: text };

    repo.createComment(comment)
      .catch((err: any) => alert(JSON.stringify(err)));
  }
}
</script>

<style lang='scss'>
#TopicView {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding-left: 20px;

  .topics {
    display: flex;
    flex: 1;
    flex-direction: column;
    height: 100%;
  }

  .CommentList {
    flex: 1;
  }

  .MessageInput {
    height: 80px;
  }
}
</style>
