<template>
  <div class="TopicList">
    <router-link
        v-for="repo in topicRepos"
        class="topic-link"
        tag="div"
        :key="repo.topic.id"
        :to="topicLink(repo.topic)">

      <Topic :repo="repo"/>
    </router-link>
  </div>
</template>

<script lang='ts'>
import Vue from 'vue';
import Component from 'vue-class-component';
import { Observer } from 'mobx-vue';
import Topic from '@/components/topics/Topic.vue';
import Repo from '@/lib/repo';
import { Topic as TopicModel } from '@/types';

@Observer
@Component({
  components: {
    Topic,
  },
})
export default class TopicList extends Vue {
  get streamRepo() {
    return Repo
      .getTeam(this.$route.params.team)!
      .getStream(this.$route.params.stream)!;
  }

  get topicRepos() {
    return this.streamRepo.topics;
  }

  topicLink(topic: TopicModel): object {
    const params = { ...this.$route.params, topic: topic.key };
    return { name: 'topic', params };
  }
}
</script>

<style lang='scss'>
.TopicList {
  overflow: auto;

  .Topic {
    background: #f0f0f0;
    border-bottom: 1px solid #aaa;
    margin: 10px 0;
  }

  .router-link-active .Topic {
    background: #fff;
    border-right: 5px solid green;
  }

  .topic-link:last-child .Topic {
    border-bottom: 0;
  }
}
</style>
