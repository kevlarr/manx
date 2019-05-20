<template>
  <div class="TopicList">
    <p v-for="repo in topicRepos">
      {{repo.topic.raw}}
    </p>
  </div>
</template>

<script lang='ts'>
import Vue from 'vue';
import Component from 'vue-class-component';
import { Observer } from 'mobx-vue';
import Repo from '@/lib/repo';

@Observer
@Component
export default class TopicList extends Vue {
  get streamRepo() {
    return Repo
      .getOrganization(this.$route.params.org)!
      .getStream(this.$route.params.stream)!;
  }

  get topicRepos() {
    return this.streamRepo.topics;
  }
}
</script>

<style lang='scss'>
.TopicList {
}
</style>
