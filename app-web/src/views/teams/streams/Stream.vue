<template>
  <div id="StreamView">
    <div class="topics">
      <TopicList/>
      <MessageInput @submit="submitTopic"/>
    </div>
    <router-view/>
  </div>
</template>

<script lang='ts'>
import Vue from 'vue';
import Component from 'vue-class-component';
import TopicList from '@/components/streams/TopicList.vue';
import { NewTopic } from '@/types';
import Repo, { StreamRepo } from '@/lib/repo';


@Component({
  components: {
    TopicList,
  },
})
export default class Stream extends Vue {
  get stream(): StreamRepo {
    const { team, stream } = this.$route.params;
    return Repo.getTeam(team)!.getStream(stream)!;
  }

  submitTopic(text: string) {
    const repo = this.stream;
    const topic: NewTopic = { raw: text };

    repo.createTopic(topic)
      .catch(err => alert(JSON.stringify(err)));
  }
}
</script>

<style lang='scss'>
#StreamView {
  display: flex;
  height: 100%;

  #TopicView {
    flex: 1;
  }

  .topics {
    display: flex;
    flex: 1;
    flex-direction: column;
    height: 100%;
  }

  .TopicList {
    flex: 1;
  }

  .MessageInput {
    height: 80px;
  }
}
</style>
